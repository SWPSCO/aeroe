use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_updater::UpdaterExt;
use tracing::{error, warn};

use crate::commands::updater::UpdateInfo;

pub async fn check_update(app_handle: &AppHandle) {
    let window = match app_handle.get_webview_window("main") {
        Some(w) => w,
        None => {
            warn!("Update checker: could not get main window");
            return;
        }
    };
    let updater = match app_handle.updater() {
        Ok(u) => u,
        Err(e) => {
            error!("Update checker: error getting updater: {:?}", e);
            return;
        }
    };
    let check_result = updater.check().await;
    match check_result {
        Ok(Some(update)) => {
            let _ = window.emit("update", UpdateInfo::new(
                true,
                update.version,
            ));
        },
        Ok(None) => {
            let _ = window.emit("update", UpdateInfo::new(
                false,
                "".to_string(),
            ));
        },
        Err(e) => {
            warn!("Update checker: error during update check: {:?}", e);
        }
    }
} 