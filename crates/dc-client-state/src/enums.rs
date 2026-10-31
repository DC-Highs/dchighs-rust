use std::fmt;
use std::path::Path;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

/// Supported asset types stored in the Dragon City local cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    /// Image files (.png, .jpg)
    Image,
    /// Audio files (.mp3, .ogg)
    Audio,
    /// Texture files (.dds)
    Texture,
    /// Mask files (.mask)
    Mask,
    /// Binary files (.bin)
    Binary,
}

impl AssetType {
    /// All asset types.
    pub const ALL: [Self; 5] = [
        Self::Image,
        Self::Audio,
        Self::Texture,
        Self::Mask,
        Self::Binary,
    ];

    /// Canonical string identifier (e.g. `"image"`, `"audio"`).
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Audio => "audio",
            Self::Texture => "texture",
            Self::Mask => "mask",
            Self::Binary => "binary",
        }
    }

    /// File extensions associated with this asset type (including leading dot).
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            Self::Image => &[".png", ".jpg"],
            Self::Audio => &[".mp3", ".ogg"],
            Self::Texture => &[".dds"],
            Self::Mask => &[".mask"],
            Self::Binary => &[".bin"],
        }
    }

    /// Returns whether the given file path matches any extension for this asset type.
    pub fn matches_extension(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        let filename = match path.file_name().and_then(|f| f.to_str()) {
            Some(f) => f.to_ascii_lowercase(),
            None => return false,
        };

        self.extensions()
            .iter()
            .any(|ext| filename.ends_with(ext))
    }

    /// Tries to resolve the `AssetType` from a file extension or path.
    pub fn from_extension(ext: &str) -> Option<Self> {
        let clean_ext = if let Some(stripped) = ext.strip_prefix('.') {
            stripped.to_ascii_lowercase()
        } else {
            ext.to_ascii_lowercase()
        };

        match clean_ext.as_str() {
            "png" | "jpg" | "jpeg" => Some(Self::Image),
            "mp3" | "ogg" => Some(Self::Audio),
            "dds" => Some(Self::Texture),
            "mask" => Some(Self::Mask),
            "bin" => Some(Self::Binary),
            _ => None,
        }
    }

    /// Tries to resolve the `AssetType` from a file path.
    pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
        let ext = path.as_ref().extension()?.to_str()?;
        Self::from_extension(ext)
    }
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for AssetType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "image" => Ok(Self::Image),
            "audio" => Ok(Self::Audio),
            "texture" => Ok(Self::Texture),
            "mask" => Ok(Self::Mask),
            "binary" => Ok(Self::Binary),
            other => Err(crate::Error::Parse(format!(
                "unknown asset type '{}', expected image, audio, texture, mask, or binary",
                other
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_type_extensions() {
        assert!(AssetType::Image.matches_extension("foo.png"));
        assert!(AssetType::Image.matches_extension("PATH/BAR.JPG"));
        assert!(!AssetType::Image.matches_extension("foo.dds"));

        assert_eq!(AssetType::from_extension("png"), Some(AssetType::Image));
        assert_eq!(AssetType::from_extension(".dds"), Some(AssetType::Texture));
        assert_eq!(AssetType::from_path("dragon.mp3"), Some(AssetType::Audio));
        assert_eq!(AssetType::from_path("mesh.bin"), Some(AssetType::Binary));
        assert_eq!(AssetType::from_path("alpha.mask"), Some(AssetType::Mask));
    }
}
