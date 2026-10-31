# dc-assets

Download static assets from the [Dragon City](https://dragoncitygame.com/) servers (sprites, thumbnails, animations, island packages, music, Flash client versions).
Rust port of [`@dchighs/dc-assets`](https://github.com/DC-Highs/dc-assets). Fan-made, not affiliated with SocialPoint.

## Installation

```sh
cargo add dc-assets
```

(`tokio` is needed to run the async functions: `cargo add tokio -F macros,rt-multi-thread`.)

## Usage

Every downloader knows its `url()` and offers:

| method | what it does |
| --- | --- |
| `fetch()` | downloads into memory (`bytes::Bytes`) — no disk needed |
| `download(path)` | saves to disk and returns the path |
| `fetch_with(&client)` / `download_with(&client, path)` | same, with your own `reqwest::Client` (proxy, timeouts, connection pool) |

```rust
use dc_assets::{DragonPhase, DragonSpriteDownloader, SpriteQuality};

#[tokio::main]
async fn main() -> Result<(), dc_assets::Error> {
    let sprite = DragonSpriteDownloader::new("1000_dragon_nature", DragonPhase::Adult)
        .quality(SpriteQuality::Large);
        // .skin("_1")

    sprite.download("dragon-sprite.png").await?;

    let bytes = sprite.fetch().await?; // keep it in memory instead
    println!("{} bytes from {}", bytes.len(), sprite.url());
    Ok(())
}
```

### Available downloaders

| TypeScript | Rust |
| --- | --- |
| `dcAssets.dragons.sprite` | `DragonSpriteDownloader::new(image_name, phase)` |
| `dcAssets.dragons.thumbnail` | `DragonThumbnailDownloader::new(image_name, phase)` |
| `dcAssets.dragons.animations.flash` | `DragonFlashAnimationDownloader::new(image_name, phase)` (`.swf`) |
| `dcAssets.dragons.animations.spine` | `DragonSpineAnimationDownloader::new(image_name, phase)` (`.zip`; `.texture_compression_format(..)`, `.use_new_url_format(false)`) |
| `dcAssets.islands.package` | `IslandPackageDownloader::new(island_type, file_name)` or `::from_path(path)` |
| `dcAssets.sounds.music` | `MusicDownloader::new(key_name)` |
| `dcAssets.flash.client` | `FlashClientDownloader::new(version)` (`Base_sec_<version>.swf`, the Flash client itself) |
| `dcAssets.chests.sprite` | `ChestSpriteDownloader::new(image_name)` (falls back to `ui_basic_*`) |
| `dcAssets.buildings.sprite` / `.thumbnail` | `BuildingSpriteDownloader` / `BuildingThumbnailDownloader` |
| `dcAssets.decorations.sprite` / `.thumbnail` | `DecorationSpriteDownloader` / `DecorationThumbnailDownloader` |
| `dcAssets.habitats.sprite` / `.thumbnail` | `HabitatSpriteDownloader` / `HabitatThumbnailDownloader` |

Dragon downloaders accept `.skin("_1")` and `.platform_prefix("dca")` (iOS `dci` by default; see `StaticFileUrlPlatformPrefix`).

```rust
use dc_assets::{IslandPackageDownloader, IslandType, MusicDownloader};

IslandPackageDownloader::new(IslandType::HeroicRaces, "1000_dragon_nature.zip")
    .download("island-package.zip")
    .await?;

MusicDownloader::new("531_dc_party_planning_island").download("music.mp3").await?;
```

## Differences from the TypeScript package

* `ChestSpriteDownloader` uses the chosen quality for both URLs (the TS version interpolates the raw option and can produce `...undefined.png`).
* `fetch()` is new: get the bytes without touching the disk.
* Errors: HTTP status codes other than 2xx are errors (`Error::Http`), file problems are `Error::Io`.

## License

MIT
