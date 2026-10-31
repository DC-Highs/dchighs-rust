use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("packages directory not found: ensure LOCALAPPDATA is set or specify path explicitly")]
    PackagesDirNotFound,

    #[error("dragon city directory starting with 'SocialPoint.DragonCityMobile_' not found in {0}")]
    DragonCityDirNotFound(PathBuf),

    #[error("user default file not found at {0}")]
    UserDefaultNotFound(PathBuf),

    #[error("xml parse error: {0}")]
    XmlParse(String),

    #[error("xml write error: {0}")]
    XmlWrite(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("parse error: {0}")]
    Parse(String),
}
