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
            frontend_log,
        ])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
                let win_clone = window.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(2000));
                    println!("[PRISTINE] URL after 2s: {:?}", win_clone.url());
                    if let Err(e) = win_clone.eval(r#"
                        console.log("PRISTINE WEBVIEW TEST EVAL");
                    "#) {
                        eprintln!("[PRISTINE] eval error: {:?}", e);
                    }
                });
                #[cfg(debug_assertions)]
                window.open_devtools();
            } else {
                eprintln!("[PRISTINE] Error: Could not find main window!");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running pristine application");
}
