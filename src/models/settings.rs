use std::sync::{Mutex, MutexGuard};

use once_cell::sync::Lazy;

use crate::{error::AppError, sever::AppState};

pub struct AppSettings {
    pub base_folder: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            base_folder: "files".to_string(),
        }
    }
}

#[derive(sqlx::FromRow)]
struct SettingKeyVal {
    key: String,
    value: String,
}

impl AppSettings {
    pub async fn try_load_from_db(state: &AppState) -> Result<(), AppError> {
        let settings = sqlx::query_as::<_, SettingKeyVal>("SELECT key, value FROM settings")
            .fetch_all(&state.pool)
            .await?;
        let mut app_settings = AppSettings::default();
        for setting in settings {
            match setting.key.as_str() {
                "base_folder" => app_settings.base_folder = setting.value,
                _ => {}
            }
        }

        set_settings(app_settings);

        Ok(())
    }
}

static SETTINGS: Lazy<Mutex<AppSettings>> = Lazy::new(|| Mutex::new(AppSettings::default()));

pub fn get_settings() -> MutexGuard<'static, AppSettings> {
    SETTINGS.lock().unwrap()
}

pub fn set_settings(settings: AppSettings) {
    *SETTINGS.lock().unwrap() = settings;
}
