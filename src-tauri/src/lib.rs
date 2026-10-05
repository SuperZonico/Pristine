/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         src-tauri/src/lib.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Main desktop application runtime, state registration, and IPC.
 * ============================================================================
 */

pub mod commands;

use commands::*;
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_catalog,
            audit_system,
            apply_tweaks,
            revert_transaction,
            get_hardware_metrics,
            clean_safe_temporary_files,
        ])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                // Window configuration
                let _ = window.set_focus();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running pristine application");
}
