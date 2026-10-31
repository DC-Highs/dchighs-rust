//! Access and manage the local client state of Dragon City (Windows version).
//!
//! Complete Rust port of `@dchighs/dc-client-state`.
//! Provides both low-level (`UserDefault`) and high-level (`Preferences`) access to `UserDefault.xml`,
//! as well as local cached assets management (`Assets`).
//!
//! > ⚠️ **Disclaimer**: This library is **NOT** intended for hacking or cheating.
//! > It only modifies local preferences and asset states; it does **not** provide any
//! > in-game advantage or modify server-side data.
//! >
//! > Fan-made library, not affiliated with SocialPoint.
//!
//! # Example
//!
//! ```no_run
//! use dc_client_state::{ClientState, AssetType};
//!
//! # fn main() -> Result<(), dc_client_state::Error> {
//! // Auto-detect Dragon City installation from %LOCALAPPDATA%\Packages
//! let client = ClientState::new()?;
//! println!("Dragon City path: {:?}", client.dragon_city_dir_path());
//!
//! // High-level preferences
//! let preferences = client.preferences();
//! preferences.disable_music()?;
//! if let Some(user_id) = preferences.get_user_id()? {
//!     println!("Current User Origin ID: {user_id}");
//! }
//!
//! // Low-level access to UserDefault.xml
//! let music_disabled = client.user_default().get("options_music_disabled")?;
//! assert_eq!(music_disabled.as_deref(), Some("true"));
//!
//! // Local assets management
//! let images = client.assets().list_assets(Some(&[AssetType::Image]))?;
//! println!("Found {} cached images", images.len());
//! # Ok(())
//! # }
//! ```

pub mod assets;
pub mod client_state;
pub mod enums;
pub mod error;
pub mod preferences;
pub mod user_default;

pub use assets::Assets;
pub use client_state::{ClientState, ClientStateBuilder, ClientStateOptions};
pub use enums::AssetType;
pub use error::Error;
pub use preferences::{Preferences, SanctuaryBreedingDragons};
pub use user_default::{UserDefault, UserDefaultEntry};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    const MOCK_USER_DEFAULT: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<userDefaultRoot>
    <entry key="hydra_app_last_version">1805120442</entry>
    <entry key="pvp_arenas_tutorial_shown">true</entry>
    <entry key="heroic_races_tutorial_shown_137">true</entry>
    <entry key="options_music_disabled">true</entry>
    <entry key="left_retry_breeed_dragon_444">12605570</entry>
    <entry key="right_retry_breeed_dragon_444">7802883</entry>
    <entry key="retry_grow_type_food_1578">8</entry>
    <entry key="retry_grow_type_food_1288">8</entry>
    <entry key="retry_grow_type_food_1287">8</entry>
    <entry key="TAPRESEARCH_IDENTIFIER"/>
    <entry key="user_origin_id">314753482635337</entry>
    <entry key="last_execution">1791514321</entry>
</userDefaultRoot>
"#;

    fn setup_mock_environment() -> (tempfile::TempDir, ClientState) {
        let dir = tempdir().unwrap();
        let packages_dir = dir.path().join("Packages");
        let dc_folder_name = "SocialPoint.DragonCityMobile_jahftqv9k5jer";
        let dc_dir = packages_dir.join(dc_folder_name);

        let user_default_dir = dc_dir.join("LocalCache").join("Local").join("DragonCity");
        fs::create_dir_all(&user_default_dir).unwrap();
        fs::write(user_default_dir.join("UserDefault.xml"), MOCK_USER_DEFAULT).unwrap();

        let assets_dir = dc_dir
            .join("LocalCache")
            .join("Local")
            .join("Social Point")
            .join("DragonCity");
        fs::create_dir_all(&assets_dir).unwrap();
        fs::write(assets_dir.join("test_ui.png"), b"image_data").unwrap();
        fs::write(assets_dir.join("ambient.mp3"), b"audio_data").unwrap();
        fs::write(assets_dir.join("texture.dds"), b"dds_data").unwrap();

        let client = ClientState::builder()
            .packages_dir(&packages_dir)
            .dragon_city_dir_name(dc_folder_name)
            .build()
            .unwrap();

        (dir, client)
    }

    #[test]
    fn test_client_state_discovery() {
        let dir = tempdir().unwrap();
        let packages_dir = dir.path().join("Packages");
        let dc_folder_name = "SocialPoint.DragonCityMobile_test123";
        fs::create_dir_all(packages_dir.join(dc_folder_name)).unwrap();

        let found_name = ClientState::find_dragon_city_dir_name(&packages_dir).unwrap();
        assert_eq!(found_name, dc_folder_name);
    }

    #[test]
    fn test_preferences_high_level_flow() {
        let (_dir, client) = setup_mock_environment();
        let pref = client.preferences();

        // User ID
        assert_eq!(pref.get_user_id().unwrap(), Some(314753482635337));
        pref.set_user_id(99999999).unwrap();
        assert_eq!(pref.get_user_id().unwrap(), Some(99999999));

        // Music & Sound
        assert!(pref.is_music_disabled().unwrap());
        pref.enable_music().unwrap();
        assert!(!pref.is_music_disabled().unwrap());
        pref.disable_music().unwrap();
        assert!(pref.is_music_disabled().unwrap());

        assert!(!pref.is_sound_disabled().unwrap());
        pref.disable_sound().unwrap();
        assert!(pref.is_sound_disabled().unwrap());
        pref.enable_sound().unwrap();
        assert!(!pref.is_sound_disabled().unwrap());

        // Last execution
        let last_secs = pref.get_last_execution_secs().unwrap();
        assert_eq!(last_secs, Some(1791514321));
        assert!(pref.get_last_execution().unwrap().is_some());

        // Farm crops
        let crops = pref.get_all_farm_crops().unwrap();
        assert_eq!(crops.len(), 3);
        pref.set_farm_crop_range(20, 1, 3).unwrap();
        let crops = pref.get_all_farm_crops().unwrap();
        assert_eq!(crops[0].value.as_deref(), Some("8"));
        assert_eq!(crops[1].value.as_deref(), Some("20"));
        assert_eq!(crops[2].value.as_deref(), Some("20"));

        pref.set_all_farm_crops(50).unwrap();
        let crops = pref.get_all_farm_crops().unwrap();
        for crop in crops {
            assert_eq!(crop.value.as_deref(), Some("50"));
        }

        pref.delete_all_farm_crops().unwrap();
        assert_eq!(pref.get_all_farm_crops().unwrap().len(), 0);

        // Tutorials
        let tuts = pref.get_display_tutorials().unwrap();
        assert_eq!(tuts.len(), 2);
        pref.set_display_tutorials(false).unwrap();
        for tut in pref.get_display_tutorials().unwrap() {
            assert_eq!(tut.value.as_deref(), Some("false"));
        }
        pref.delete_display_tutorials().unwrap();
        assert_eq!(pref.get_display_tutorials().unwrap().len(), 0);

        // Sanctuary breeding retry dragons
        let sanctuary = pref.get_retry_breed_sanctuary_dragons().unwrap().unwrap();
        assert_eq!(sanctuary.left_account_dragon_id, 12605570);
        assert_eq!(sanctuary.right_account_dragon_id, 7802883);

        pref.set_retry_breed_sanctuary_dragons(1111111, 2222222).unwrap();
        let updated = pref.get_retry_breed_sanctuary_dragons().unwrap().unwrap();
        assert_eq!(updated.left_account_dragon_id, 1111111);
        assert_eq!(updated.right_account_dragon_id, 2222222);
    }

    #[test]
    fn test_assets_management_workflow() {
        let (_dir, client) = setup_mock_environment();
        let assets = client.assets();

        let all = assets.list_assets(None).unwrap();
        assert_eq!(all.len(), 3);

        let images = assets.list_assets(Some(&[AssetType::Image])).unwrap();
        assert_eq!(images.len(), 1);
        assert!(images[0].ends_with("test_ui.png"));

        let audio = assets.list_assets(Some(&[AssetType::Audio])).unwrap();
        assert_eq!(audio.len(), 1);
        assert!(audio[0].ends_with("ambient.mp3"));

        let textures = assets.list_assets(Some(&[AssetType::Texture])).unwrap();
        assert_eq!(textures.len(), 1);
        assert!(textures[0].ends_with("texture.dds"));

        // Delete asset
        assets.delete("test_ui.png").unwrap();
        assert_eq!(assets.list_assets(Some(&[AssetType::Image])).unwrap().len(), 0);

        // Clear all
        assets.clear_assets().unwrap();
        assert_eq!(assets.list_assets(None).unwrap().len(), 0);
    }

    #[test]
    fn test_real_client_state_read_only_if_installed() {
        if let Ok(client) = ClientState::new() {
            if client.dragon_city_dir_path().exists() && client.user_default_file_path().exists() {
                let entries = client.user_default().read_raw().expect("failed to parse real UserDefault.xml");
                assert!(!entries.is_empty(), "real UserDefault.xml should contain entries");
                let pref = client.preferences();
                let user_id = pref.get_user_id().expect("failed to read real user_id");
                assert!(user_id.is_some(), "expected real user_origin_id in real file");
            }
        }
    }
}
