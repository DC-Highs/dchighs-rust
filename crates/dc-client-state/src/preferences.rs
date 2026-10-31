use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::user_default::{UserDefault, UserDefaultEntry};

/// Pair of dragon IDs used in the Breeding Sanctuary retry mechanism.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SanctuaryBreedingDragons {
    /// Dragon ID on the left breeding pedestal.
    pub left_account_dragon_id: u64,
    /// Dragon ID on the right breeding pedestal.
    pub right_account_dragon_id: u64,
}

/// High-level API for Dragon City user preferences.
#[derive(Debug, Clone)]
pub struct Preferences {
    user_default: UserDefault,
}

impl Preferences {
    /// Creates a new `Preferences` wrapper around `UserDefault`.
    pub fn new(user_default: UserDefault) -> Self {
        Self { user_default }
    }

    /// Access the underlying `UserDefault` handle.
    pub fn user_default(&self) -> &UserDefault {
        &self.user_default
    }

    /// Sets the crop plant ID for farms within the given range `[start, end)`.
    pub fn set_farm_crop_range(&self, plant_id: u32, start: usize, end: usize) -> Result<(), Error> {
        let plant_str = plant_id.to_string();
        self.user_default.mutate_entries(|entries| {
            let mut counter = 0;
            for entry in entries.iter_mut() {
                if entry.key.starts_with("retry_grow_type_food") {
                    if counter >= start && counter < end {
                        entry.value = Some(plant_str.clone());
                    }
                    counter += 1;
                }
            }
        })
    }

    /// Sets all farms to grow the specified crop plant ID.
    pub fn set_all_farm_crops(&self, plant_id: u32) -> Result<(), Error> {
        let plant_str = plant_id.to_string();
        self.user_default.mutate_entries(|entries| {
            for entry in entries.iter_mut() {
                if entry.key.starts_with("retry_grow_type_food") {
                    entry.value = Some(plant_str.clone());
                }
            }
        })
    }

    /// Deletes all farm crop retry entries.
    pub fn delete_all_farm_crops(&self) -> Result<(), Error> {
        self.user_default.mutate_entries(|entries| {
            entries.retain(|entry| !entry.key.starts_with("retry_grow_type_food"));
        })
    }

    /// Returns all farm crop entries (`retry_grow_type_food*`).
    pub fn get_all_farm_crops(&self) -> Result<Vec<UserDefaultEntry>, Error> {
        self.user_default.list_by_prefix("retry_grow_type_food")
    }

    /// Lists all entries related to tutorials.
    pub fn get_display_tutorials(&self) -> Result<Vec<UserDefaultEntry>, Error> {
        self.user_default.list_by_include("tutorial")
    }

    /// Sets whether tutorials are marked as shown (`true`) or hidden (`false`).
    pub fn set_display_tutorials(&self, shown: bool) -> Result<(), Error> {
        let val_str = if shown { "true" } else { "false" };
        self.user_default.mutate_entries(|entries| {
            for entry in entries.iter_mut() {
                if entry.key.contains("tutorial") {
                    entry.value = Some(val_str.to_string());
                }
            }
        })
    }

    /// Deletes all tutorial entries.
    pub fn delete_display_tutorials(&self) -> Result<(), Error> {
        self.user_default.mutate_entries(|entries| {
            entries.retain(|entry| !entry.key.contains("tutorial"));
        })
    }

    /// Disables background music in the game client.
    pub fn disable_music(&self) -> Result<(), Error> {
        self.user_default.set_bool("options_music_disabled", true)
    }

    /// Enables background music in the game client.
    pub fn enable_music(&self) -> Result<(), Error> {
        self.user_default.delete("options_music_disabled")?;
        Ok(())
    }

    /// Checks if music is currently disabled.
    pub fn is_music_disabled(&self) -> Result<bool, Error> {
        let val = self.user_default.get("options_music_disabled")?;
        Ok(val.as_deref() == Some("true"))
    }

    /// Disables sound effects in the game client.
    pub fn disable_sound(&self) -> Result<(), Error> {
        self.user_default.set_bool("options_sound_disabled", true)
    }

    /// Enables sound effects in the game client.
    pub fn enable_sound(&self) -> Result<(), Error> {
        self.user_default.delete("options_sound_disabled")?;
        Ok(())
    }

    /// Checks if sound effects are currently disabled.
    pub fn is_sound_disabled(&self) -> Result<bool, Error> {
        let val = self.user_default.get("options_sound_disabled")?;
        Ok(val.as_deref() == Some("true"))
    }

    /// Gets the game's User Origin ID (`user_origin_id`).
    pub fn get_user_id(&self) -> Result<Option<u64>, Error> {
        match self.user_default.get("user_origin_id")? {
            Some(s) => {
                let id = s.trim().parse::<u64>().map_err(|e| {
                    Error::Parse(format!("failed to parse user_origin_id '{s}': {e}"))
                })?;
                Ok(Some(id))
            }
            None => Ok(None),
        }
    }

    /// Sets the game's User Origin ID (`user_origin_id`).
    pub fn set_user_id(&self, user_id: u64) -> Result<(), Error> {
        self.user_default.set_number("user_origin_id", user_id)
    }

    /// Gets the last execution timestamp as raw seconds since UNIX epoch.
    pub fn get_last_execution_secs(&self) -> Result<Option<u64>, Error> {
        match self.user_default.get("last_execution")? {
            Some(s) => {
                let secs = s.trim().parse::<u64>().map_err(|e| {
                    Error::Parse(format!("failed to parse last_execution '{s}': {e}"))
                })?;
                Ok(Some(secs))
            }
            None => Ok(None),
        }
    }

    /// Gets the last execution timestamp converted to [`SystemTime`].
    pub fn get_last_execution(&self) -> Result<Option<SystemTime>, Error> {
        let secs = self.get_last_execution_secs()?;
        Ok(secs.map(|s| UNIX_EPOCH + Duration::from_secs(s)))
    }

    /// Sets the last execution timestamp in seconds.
    pub fn set_last_execution_secs(&self, timestamp_secs: u64) -> Result<(), Error> {
        self.user_default.set_number("last_execution", timestamp_secs)
    }

    /// Gets the retry breeding dragons for Sanctuary (`left_retry_breeed_dragon` and `right_retry_breeed_dragon`).
    pub fn get_retry_breed_sanctuary_dragons(&self) -> Result<Option<SanctuaryBreedingDragons>, Error> {
        let left_entries = self.user_default.list_by_include("left_retry_breeed_dragon")?;
        let right_entries = self.user_default.list_by_include("right_retry_breeed_dragon")?;

        let left_entry = left_entries.into_iter().next();
        let right_entry = right_entries.into_iter().next();

        match (left_entry.and_then(|e| e.value), right_entry.and_then(|e| e.value)) {
            (Some(left_str), Some(right_str)) => {
                let left_id = left_str.trim().parse::<u64>().map_err(|e| {
                    Error::Parse(format!("invalid left breeding dragon id '{left_str}': {e}"))
                })?;
                let right_id = right_str.trim().parse::<u64>().map_err(|e| {
                    Error::Parse(format!("invalid right breeding dragon id '{right_str}': {e}"))
                })?;
                Ok(Some(SanctuaryBreedingDragons {
                    left_account_dragon_id: left_id,
                    right_account_dragon_id: right_id,
                }))
            }
            _ => Ok(None),
        }
    }

    /// Sets retry breeding dragons for Sanctuary (`left_retry_breeed_dragon_444` and `right_retry_breeed_dragon_444`).
    pub fn set_retry_breed_sanctuary_dragons(
        &self,
        left_account_dragon_id: u64,
        right_account_dragon_id: u64,
    ) -> Result<(), Error> {
        self.user_default
            .set_number("left_retry_breeed_dragon_444", left_account_dragon_id)?;
        self.user_default
            .set_number("right_retry_breeed_dragon_444", right_account_dragon_id)?;
        Ok(())
    }
}
