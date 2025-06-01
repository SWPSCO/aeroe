use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;
use std::fs;
use std::path::PathBuf;
use tracing::{info, error};

// Define the state for terms
pub struct TermsState {
    terms_of_use_accepted: bool,
    terms_of_use_file_path: PathBuf,
    privacy_policy_accepted: bool,
    privacy_policy_file_path: PathBuf,
}

impl TermsState {
    pub fn new(app_handle: &AppHandle) -> Self {
        let data_dir = app_handle.path().app_data_dir().expect("Failed to get app data dir for TermsState");
        
        // Handle TermsOfUse
        let terms_file_path = data_dir.join("TermsOfUse");
        info!("[TermsState] Initializing. TermsOfUse file path: {:?}", terms_file_path);
        let terms_accepted = Self::load_or_create_flag_file(&terms_file_path, "TermsOfUse");

        // Handle PrivacyPolicy
        let privacy_policy_file_path = data_dir.join("PrivacyPolicy");
        info!("[TermsState] Initializing. PrivacyPolicy file path: {:?}", privacy_policy_file_path);
        let privacy_policy_accepted = Self::load_or_create_flag_file(&privacy_policy_file_path, "PrivacyPolicy");

        TermsState {
            terms_of_use_accepted: terms_accepted,
            terms_of_use_file_path: terms_file_path,
            privacy_policy_accepted,
            privacy_policy_file_path,
        }
    }

    // Helper function to load or create a flag file
    fn load_or_create_flag_file(file_path: &PathBuf, name: &str) -> bool {
        if file_path.exists() {
            match fs::read_to_string(file_path) {
                Ok(content) => match content.trim().parse::<bool>() {
                    Ok(val) => {
                        info!("[TermsState] {} file exists. Content: '{}', Parsed as: {}", name, content.trim(), val);
                        val
                    },
                    Err(e) => {
                        error!("[TermsState] Failed to parse {} content '{}': {}. Defaulting to false.", name, content.trim(), e);
                        if let Err(write_err) = fs::write(file_path, "false") {
                            error!("[TermsState] Failed to overwrite corrupted {} file with 'false': {}", name, write_err);
                        }
                        false
                    }
                },
                Err(e) => {
                    error!("[TermsState] Failed to read {} file at {:?}: {}. Defaulting to false.", name, file_path, e);
                    false
                }
            }
        } else {
            info!("[TermsState] {} file does not exist at {:?}. Creating with 'false'.", name, file_path);
            if let Some(parent) = file_path.parent() {
                if !parent.exists() {
                    info!("[TermsState] Parent directory {:?} for {} does not exist. Attempting to create.", parent, name);
                    if let Err(e) = fs::create_dir_all(parent) {
                        error!("[TermsState] Failed to create parent directory for {} at {:?}: {}", name, parent, e);
                    } else {
                         info!("[TermsState] Successfully created parent directory for {} at {:?}", name, parent);
                    }
                }
            }
            match fs::write(file_path, "false") {
                Ok(_) => info!("[TermsState] Successfully created {} file with 'false' at {:?}", name, file_path),
                Err(e) => error!("[TermsState] Failed to create and write to {} file at {:?}: {}", name, file_path, e),
            }
            false
        }
    }

    // Helper function to accept and write to a flag file
    fn accept_flag(accepted_flag: &mut bool, file_path: &PathBuf, name: &str) -> Result<(), String> {
        *accepted_flag = true;
        info!("[TermsState] Accepting {}. Writing 'true' to {:?}", name, file_path);
        fs::write(file_path, "true").map_err(|e| {
            error!("[TermsState] Failed to write 'true' to {} file {:?}: {}", name, file_path, e);
            e.to_string()
        })
    }

    pub fn accept_terms_of_use(&mut self) -> Result<(), String> {
        Self::accept_flag(&mut self.terms_of_use_accepted, &self.terms_of_use_file_path, "TermsOfUse")
    }

    pub fn is_terms_of_use_accepted(&self) -> bool {
        self.terms_of_use_accepted
    }

    pub fn accept_privacy_policy(&mut self) -> Result<(), String> {
        Self::accept_flag(&mut self.privacy_policy_accepted, &self.privacy_policy_file_path, "PrivacyPolicy")
    }

    pub fn is_privacy_policy_accepted(&self) -> bool {
        self.privacy_policy_accepted
    }
}

#[tauri::command]
pub async fn privacy_policy_is_accepted(state: State<'_, Mutex<TermsState>>) -> Result<bool, String> {
    let terms_state = state.lock().await;
    let accepted_status = terms_state.is_privacy_policy_accepted();
    info!("[TermsCommand] privacy_policy_is_accepted called. Current status: {}", accepted_status);
    Ok(accepted_status)
}

#[tauri::command]
pub async fn terms_of_use_is_accepted(state: State<'_, Mutex<TermsState>>) -> Result<bool, String> {
    let terms_state = state.lock().await;
    let accepted_status = terms_state.is_terms_of_use_accepted();
    info!("[TermsCommand] terms_of_use_is_accepted called. Current status: {}", accepted_status);
    Ok(accepted_status)
}

#[tauri::command]
pub async fn accept_terms_of_use(state: State<'_, Mutex<TermsState>>) -> Result<(), String> {
    info!("[TermsCommand] accept_terms_of_use command received.");
    let mut terms_state = state.lock().await;
    match terms_state.accept_terms_of_use() {
        Ok(_) => {
            info!("[TermsCommand] Terms of use successfully accepted and file updated.");
            Ok(())
        }
        Err(e) => {
            error!("[TermsCommand] Failed to accept terms of use: {}", e);
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn accept_privacy_policy(state: State<'_, Mutex<TermsState>>) -> Result<(), String> {
    info!("[TermsCommand] accept_privacy_policy command received.");
    let mut terms_state = state.lock().await;
    match terms_state.accept_privacy_policy() {
        Ok(_) => {
            info!("[TermsCommand] Privacy policy successfully accepted and file updated.");
            Ok(())
        }
        Err(e) => {
            error!("[TermsCommand] Failed to accept privacy policy: {}", e);
            Err(e)
        }
    }
}