# dc-client-state

Access and manage the local client state of [Dragon City](https://dragoncitygame.com/) (Windows version).
Rust port of [`@dchighs/dc-client-state`](https://github.com/DC-Highs/packages/tree/main/packages/dc-client-state). Fan-made, not affiliated with SocialPoint.

> ⚠️ **Disclaimer**: This library is **NOT** intended for hacking or cheating. The author is radically against hacking. This library only modifies local preferences and asset states; it does **not** provide any in-game advantage or modify server-side data.

## 📦 Installation

```sh
cargo add dc-client-state
```

## 🚀 Usage

To start interacting with the Dragon City client state, instantiate [`ClientState`]. It automatically locates the Dragon City installation directory in `%LOCALAPPDATA%\Packages`.

```rust
use dc_client_state::ClientState;

fn main() -> Result<(), dc_client_state::Error> {
    let client = ClientState::new()?;
    println!("Game directory: {:?}", client.dragon_city_dir_path());
    Ok(())
}
```

## 🏗 High-Level vs Low-Level Access

This library provides two ways to interact with stored game data: **UserDefault** (Low-Level) and **Preferences** (High-Level).

### 1. UserDefault (Low-Level)

`UserDefault` provides direct access to the `UserDefault.xml` file with raw key-value manipulation.

```rust
// Read a raw value
let music_disabled = client.user_default().get("options_music_disabled")?;

// Set a raw value manually
client.user_default().set("options_music_disabled", "true")?;

// Or using type-safe helpers
client.user_default().set_bool("options_music_disabled", true)?;
```

### 2. Preferences (High-Level)

`Preferences` is a high-level wrapper around `UserDefault` providing semantic methods for common configurations.

```rust
let pref = client.preferences();

// Music & Sound controls
pref.disable_music()?;
pref.enable_sound()?;

// Read user ID
if let Some(user_id) = pref.get_user_id()? {
    println!("User Origin ID: {user_id}");
}

// Farm crops configuration
pref.set_all_farm_crops(1)?;

// Breeding Sanctuary retry dragons
let dragons = pref.get_retry_breed_sanctuary_dragons()?;
```

## 🎨 Assets Management

The `Assets` manager (`client.assets()`) allows inspecting, replacing, and clearing cached game files (textures, music, sprites, etc.).

```rust
use dc_client_state::AssetType;

// List all cached assets
let all_assets = client.assets().list_assets(None)?;
println!("Found {} assets", all_assets.len());

// Filter by asset type (.png, .jpg)
let images = client.assets().list_assets(Some(&[AssetType::Image]))?;

// Replace an asset with a custom local file
client.assets().set("ui_button_close.png", "./my-custom-assets/new-close.png")?;

// Delete a specific asset
client.assets().delete("old_banner.png")?;

// Clear all cached files (game re-downloads needed ones on next launch)
client.assets().clear_assets()?;
```

## License

MIT
