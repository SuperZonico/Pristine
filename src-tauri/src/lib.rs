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
            get_transaction_history,
            revert_transaction,
            get_hardware_metrics,
            clean_safe_temporary_files,
            clean_winsxs_component_store,
            create_system_restore_point,
            check_elevation,
            request_elevation,
            get_installed_apps,
            uninstall_app,
            scan_app_residuals,
            clean_app_residuals,
            flush_dns,
            get_hosts_shield_status,
            toggle_hosts_shield,
            restart_windows_explorer,
            frontend_log,
        ])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            } else {
                eprintln!("[PRISTINE] Error: Could not find main window!");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running pristine application");
}
