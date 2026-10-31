use std::fs;
use std::path::{Path, PathBuf};

use crate::enums::AssetType;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Assets {
    assets_dir_path: PathBuf,
}

impl Assets {
    pub fn new(assets_dir_path: impl Into<PathBuf>) -> Self {
        Self {
            assets_dir_path: assets_dir_path.into(),
        }
    }

    pub fn assets_dir_path(&self) -> &Path {
        &self.assets_dir_path
    }

    pub fn get_file_path(&self, asset_file_name: impl AsRef<Path>) -> PathBuf {
        self.assets_dir_path.join(asset_file_name)
    }

    pub fn set(
        &self,
        current_asset_file_name: impl AsRef<Path>,
        substitute_asset_file_path: impl AsRef<Path>,
    ) -> Result<(), Error> {
        let dest_path = self.get_file_path(current_asset_file_name);
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(substitute_asset_file_path, dest_path)?;
        Ok(())
    }

    pub fn delete(&self, asset_file_name: impl AsRef<Path>) -> Result<(), Error> {
        let file_path = self.get_file_path(asset_file_name);
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }

    pub fn list_assets(&self, asset_types: Option<&[AssetType]>) -> Result<Vec<PathBuf>, Error> {
        if !self.assets_dir_path.exists() {
            return Ok(Vec::new());
        }

        let mut paths = Vec::new();

        for entry in fs::read_dir(&self.assets_dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(types) = asset_types {
                    let matches = types.iter().any(|t| t.matches_extension(&path));
                    if matches {
                        paths.push(path);
                    }
                } else {
                    paths.push(path);
                }
            }
        }

        paths.sort();
        Ok(paths)
    }

    pub fn clear_assets(&self) -> Result<(), Error> {
        if !self.assets_dir_path.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.assets_dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(path)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_assets_management() {
        let dir = tempdir().unwrap();
        let assets = Assets::new(dir.path());

        let sub_img = dir.path().join("source.png");
        fs::write(&sub_img, b"fake png data").unwrap();

        let sub_audio = dir.path().join("source.mp3");
        fs::write(&sub_audio, b"fake audio data").unwrap();

        assets.set("dragon_01.png", &sub_img).unwrap();
        assets.set("intro.mp3", &sub_audio).unwrap();

        assert_eq!(assets.get_file_path("dragon_01.png"), dir.path().join("dragon_01.png"));

        let all = assets.list_assets(None).unwrap();
        assert!(all.iter().any(|p| p.ends_with("dragon_01.png")));

        let images = assets.list_assets(Some(&[AssetType::Image])).unwrap();
        for img in images {
            assert!(img.extension().unwrap() == "png");
        }

        assets.delete("dragon_01.png").unwrap();
        assert!(!assets.get_file_path("dragon_01.png").exists());

        assets.clear_assets().unwrap();
        assert_eq!(assets.list_assets(None).unwrap().len(), 0);
    }
}
