use tauri::{Manager, Emitter};
use tauri_plugin_updater::UpdaterExt;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    has_update: bool,
    update_version: String,
}

impl UpdateInfo {
    pub fn new(has_update: bool, update_version: String) -> Self {
        Self {
            has_update,
            update_version,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    downloaded: usize,
    total: u64,
}

#[tauri::command]
pub async fn download_and_install_update(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("main").unwrap();
    if let Some(update) = app.updater()
        .map_err(|e| e.to_string())?
        .check().await.map_err(|e| e.to_string())? {
        let mut downloaded: usize = 0;
        let mut total: u64 = 0;
        update
          .download_and_install(
            |chunk_length, content_length| {
              downloaded += chunk_length;
              total = content_length.unwrap_or(0);
              let _ = window.emit("update_downloaded", DownloadProgress {
                  downloaded,
                  total,
              });
            },
            || {}
          )
          .await.map_err(|e| e.to_string())?;
        app.restart();
      }
      Ok(())
}