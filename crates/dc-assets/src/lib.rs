//! Download static assets (sprites, thumbnails, animations, music) from the
//! Dragon City servers.
//!
//! Every downloader is a small value type that knows its URL. It can either
//! save the file to disk with `download(path)` or give you the bytes with
//! `fetch()` (no disk needed).
//!
//! ```no_run
//! use dc_assets::{DragonPhase, DragonSpriteDownloader, SpriteQuality};
//!
//! # async fn run() -> Result<(), dc_assets::Error> {
//! let sprite = DragonSpriteDownloader::new("1000_dragon_nature", DragonPhase::Adult)
//!     .quality(SpriteQuality::Large);
//!
//! sprite.download("dragon-sprite.png").await?;
//! let bytes = sprite.fetch().await?;
//! # let _ = bytes;
//! # Ok(())
//! # }
//! ```
//!
//! Fan-made library, not affiliated with SocialPoint.

mod client;
mod dragons;
mod error;
mod flash;
mod islands;
mod sounds;
mod structures;

pub use client::{default_client, AssetSource};
pub use dragons::{
    DragonFlashAnimationDownloader, DragonSpineAnimationDownloader, DragonSpriteDownloader,
    DragonThumbnailDownloader,
};
pub use error::Error;
pub use flash::FlashClientDownloader;
pub use islands::{ChestSpriteDownloader, IslandPackageDownloader};
pub use sounds::MusicDownloader;
pub use structures::{
    BuildingSpriteDownloader, BuildingThumbnailDownloader, DecorationSpriteDownloader,
    DecorationThumbnailDownloader, HabitatSpriteDownloader, HabitatThumbnailDownloader,
};

pub use dc_core::{
    BuildingSpriteQuality, ChestSpriteQuality, DecorationSpriteQuality, DragonPhase,
    DragonSpriteQuality, HabitatSpriteQuality, IslandType, SpriteQuality,
    StaticFileUrlPlatformPrefix, TextureCompressionFormat,
};
