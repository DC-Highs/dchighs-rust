use dc_core::StaticFileUrlPlatformPrefix;

use crate::client::impl_downloader;

/// A version of the Dragon City Flash client (`Base_sec_<version>.swf`), e.g. `"2.2.5"`.
#[derive(Debug, Clone)]
pub struct FlashClientDownloader {
    pub version: String,
    pub platform_prefix: String,
    url: String,
}

impl FlashClientDownloader {
    pub fn new(version: impl Into<String>) -> Self {
        let mut this = Self {
            version: version.into(),
            platform_prefix: StaticFileUrlPlatformPrefix::default().into(),
            url: String::new(),
        };
        this.rebuild();
        this
    }

    /// Static server prefix (defaults to iOS, `dci`).
    pub fn platform_prefix(mut self, platform_prefix: impl Into<String>) -> Self {
        self.platform_prefix = platform_prefix.into();
        self.rebuild();
        self
    }

    fn rebuild(&mut self) {
        self.url = format!(
            "https://{}-static-s1.socialpointgames.com/static/dragoncity/flash/Base_sec_{}.swf",
            self.platform_prefix, self.version
        );
    }
}

impl_downloader!(FlashClientDownloader);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flash_client_url() {
        assert_eq!(
            FlashClientDownloader::new("2.2.5").url(),
            "https://dci-static-s1.socialpointgames.com/static/dragoncity/flash/Base_sec_2.2.5.swf"
        );
        assert!(FlashClientDownloader::new("1.0")
            .platform_prefix("dca")
            .url()
            .starts_with("https://dca-static-s1."));
    }
}
