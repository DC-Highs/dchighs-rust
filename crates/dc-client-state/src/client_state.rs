use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::assets::Assets;
use crate::error::Error;
use crate::preferences::Preferences;
use crate::user_default::UserDefault;

#[derive(Debug, Clone, Default)]
pub struct ClientStateOptions {
    pub packages_dir_path: Option<PathBuf>,
    pub dragon_city_dir_name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ClientStateBuilder {
    options: ClientStateOptions,
}

impl ClientStateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn packages_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.options.packages_dir_path = Some(path.into());
        self
    }

    pub fn dragon_city_dir_name(mut self, name: impl Into<String>) -> Self {
        self.options.dragon_city_dir_name = Some(name.into());
        self
    }

    pub fn build(self) -> Result<ClientState, Error> {
        ClientState::with_options(self.options)
    }
}

#[derive(Debug, Clone)]
pub struct ClientState {
    packages_dir_path: PathBuf,
    dragon_city_dir_name: String,
    dragon_city_dir_path: PathBuf,
    user_default_file_path: PathBuf,
    assets_dir_path: PathBuf,
    user_default: UserDefault,
    assets: Assets,
}

impl ClientState {
    pub fn new() -> Result<Self, Error> {
        Self::with_options(ClientStateOptions::default())
    }

    pub fn builder() -> ClientStateBuilder {
        ClientStateBuilder::new()
    }

    pub fn with_options(options: ClientStateOptions) -> Result<Self, Error> {
        let packages_dir_path = match options.packages_dir_path {
            Some(p) => p,
            None => Self::default_packages_dir().ok_or(Error::PackagesDirNotFound)?,
        };

        let dragon_city_dir_name = match options.dragon_city_dir_name {
            Some(name) => name,
            None => Self::find_dragon_city_dir_name(&packages_dir_path)?,
        };

        let dragon_city_dir_path = packages_dir_path.join(&dragon_city_dir_name);

        let assets_dir_path = dragon_city_dir_path
            .join("LocalCache")
            .join("Local")
            .join("Social Point")
            .join("DragonCity");

        let user_default_file_path = dragon_city_dir_path
            .join("LocalCache")
            .join("Local")
            .join("DragonCity")
            .join("UserDefault.xml");

        let user_default = UserDefault::new(user_default_file_path.clone());
        let assets = Assets::new(assets_dir_path.clone());

        Ok(Self {
            packages_dir_path,
            dragon_city_dir_name,
            dragon_city_dir_path,
            user_default_file_path,
            assets_dir_path,
            user_default,
            assets,
        })
    }

    pub fn default_packages_dir() -> Option<PathBuf> {
        env::var_os("LOCALAPPDATA").map(|local| PathBuf::from(local).join("Packages"))
    }

    pub fn find_dragon_city_dir_name(packages_dir: impl AsRef<Path>) -> Result<String, Error> {
        let packages_dir = packages_dir.as_ref();
        let prefix = "SocialPoint.DragonCityMobile_";

        if !packages_dir.exists() {
            return Err(Error::DragonCityDirNotFound(packages_dir.to_path_buf()));
        }

        for entry in fs::read_dir(packages_dir)? {
            let entry = entry?;
            let name = entry.file_name();
            if let Some(name_str) = name.to_str() {
                if name_str.starts_with(prefix) {
                    return Ok(name_str.to_string());
                }
            }
        }

        Err(Error::DragonCityDirNotFound(packages_dir.to_path_buf()))
    }

    pub fn packages_dir_path(&self) -> &Path {
        &self.packages_dir_path
    }

    pub fn dragon_city_dir_name(&self) -> &str {
        &self.dragon_city_dir_name
    }

    pub fn dragon_city_dir_path(&self) -> &Path {
        &self.dragon_city_dir_path
    }

    pub fn user_default_file_path(&self) -> &Path {
        &self.user_default_file_path
    }

    pub fn assets_dir_path(&self) -> &Path {
        &self.assets_dir_path
    }

    pub fn user_default(&self) -> &UserDefault {
        &self.user_default
    }

    pub fn assets(&self) -> &Assets {
        &self.assets
    }

    pub fn preferences(&self) -> Preferences {
        Preferences::new(self.user_default.clone())
    }
}
