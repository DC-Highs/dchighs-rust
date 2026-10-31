use std::path::PathBuf;

/// Error types for `dc-client-state`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The Windows Packages directory was not found.
    #[error("packages directory not found: ensure LOCALAPPDATA is set or specify path explicitly")]
    PackagesDirNotFound,

    /// The Dragon City directory was not found in the packages directory.
    #[error("dragon city directory starting with 'SocialPoint.DragonCityMobile_' not found in {0}")]
    DragonCityDirNotFound(PathBuf),

    /// The UserDefault.xml file was not found.
    #[error("user default file not found at {0}")]
    UserDefaultNotFound(PathBuf),

    /// XML parsing error.
    #[error("xml parse error: {0}")]
    XmlParse(String),

    /// XML writing error.
    #[error("xml write error: {0}")]
    XmlWrite(String),

    /// I/O error when reading or writing files.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Error parsing a value (e.g. numeric ID or date).
    #[error("parse error: {0}")]
    Parse(String),
}
