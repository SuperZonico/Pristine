/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         src-tauri/src/commands.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Tauri IPC command handlers bridging UI to the Rust engine.
 * ============================================================================
 */

use pristine_core::catalog::get_default_catalog;
use pristine_core::models::{SystemAuditReport, TweakDefinition};
use pristine_core::transaction::TransactionSession;
use pristine_engine::{
    apply_tweak, create_transaction_session, load_journal, remove_session, revert_session,
    run_full_audit, save_session,
};
use pristine_metrics::{HardwareMetrics, MetricsCollector};
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub metrics_collector: Mutex<MetricsCollector>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            metrics_collector: Mutex::new(MetricsCollector::new()),
        }
    }
}

#[tauri::command]
pub fn get_catalog() -> Vec<TweakDefinition> {
    get_default_catalog()
}

#[tauri::command]
pub fn audit_system() -> SystemAuditReport {
    let catalog = get_default_catalog();
    run_full_audit(&catalog)
}

#[tauri::command]
pub fn apply_tweaks(
    tweak_ids: Vec<String>,
    description: String,
) -> Result<TransactionSession, String> {
    let catalog = get_default_catalog();
    let mut session = create_transaction_session(description);

    for id in &tweak_ids {
        if let Some(tweak) = catalog.iter().find(|t| &t.id == id) {
            apply_tweak(tweak, &mut session).map_err(|e| e.to_string())?;
        }
    }

    // Persist session to local disk journal
    let _ = save_session(&session);

    Ok(session)
}

#[tauri::command]
pub fn get_transaction_history() -> Vec<TransactionSession> {
    load_journal()
}

#[tauri::command]
pub fn revert_transaction(session: TransactionSession) -> Result<(), String> {
    revert_session(&session).map_err(|e| e.to_string())?;
    let _ = remove_session(&session.session_id);
    Ok(())
}

#[tauri::command]
pub fn get_hardware_metrics(state: State<'_, AppState>) -> HardwareMetrics {
    if let Ok(mut collector) = state.metrics_collector.lock() {
        collector.collect()
    } else {
        HardwareMetrics {
            ram_total_mb: 0,
            ram_used_mb: 0,
            ram_percent: 0,
            cpu_percent: 0,
            timestamp_utc: 0,
        }
    }
}

#[derive(serde::Serialize)]
pub struct CleanupResult {
    pub bytes_freed: u64,
    pub files_deleted: usize,
    pub errors_encountered: usize,
}

#[tauri::command]
pub fn clean_safe_temporary_files() -> CleanupResult {
    let mut bytes_freed: u64 = 0;
    let mut files_deleted: usize = 0;
    let mut errors_encountered: usize = 0;

    let temp_dir = std::env::temp_dir();
    let one_day_ago = std::time::SystemTime::now()
        .checked_sub(std::time::Duration::from_secs(86400))
        .unwrap_or(std::time::SystemTime::now());

    if let Ok(entries) = std::fs::read_dir(temp_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = entry.metadata() {
                    let accessed = metadata.accessed().unwrap_or(std::time::SystemTime::now());
                    if accessed < one_day_ago {
                        let len = metadata.len();
                        if std::fs::remove_file(&path).is_ok() {
                            bytes_freed += len;
                            files_deleted += 1;
                        } else {
                            errors_encountered += 1;
                        }
                    }
                }
            }
        }
    }
    CleanupResult {
        bytes_freed,
        files_deleted,
        errors_encountered,
    }
}

#[tauri::command]
pub fn frontend_log(msg: String) {
    println!("[PRISTINE FRONTEND]: {}", msg);
}

#[derive(serde::Serialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub fn clean_winsxs_component_store() -> OperationResult {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = std::process::Command::new("dism.exe");
        cmd.args(["/Online", "/Cleanup-Image", "/StartComponentCleanup"]);
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                if output.status.success() {
                    OperationResult {
                        success: true,
                        message: "Almacén WinSxS compactado y limpiado exitosamente.".to_string(),
                    }
                } else {
                    OperationResult {
                        success: false,
                        message: format!(
                            "Error en DISM: {}",
                            if !stderr.is_empty() { stderr } else { stdout }
                        ),
                    }
                }
            }
            Err(e) => OperationResult {
                success: false,
                message: format!("No se pudo invocar DISM: {}", e),
            },
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        OperationResult {
            success: true,
            message: "Simulación de limpieza WinSxS en entorno no Windows.".to_string(),
        }
    }
}

#[tauri::command]
pub fn create_system_restore_point() -> OperationResult {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = std::process::Command::new("powershell.exe");
        cmd.args([
            "-NoProfile",
            "-Command",
            "Checkpoint-Computer -Description 'Pristine Punto de Seguridad' -RestorePointType 'MODIFY_SETTINGS' -ErrorAction Stop",
        ]);
        cmd.creation_flags(0x08000000);

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    OperationResult {
                        success: true,
                        message: "Punto de restauración del sistema (VSS) creado exitosamente."
                            .to_string(),
                    }
                } else {
                    let err = String::from_utf8_lossy(&output.stderr).to_string();
                    OperationResult {
                        success: false,
                        message: format!("No se pudo crear punto de restauración (puede requerir permisos de Administrador): {}", err),
                    }
                }
            }
            Err(e) => OperationResult {
                success: false,
                message: format!("Error al ejecutar Checkpoint-Computer: {}", e),
            },
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        OperationResult {
            success: true,
            message: "Simulación de punto de restauración en entorno no Windows.".to_string(),
        }
    }
}

/// Checks whether the running process has elevated (Administrator) rights.
#[tauri::command]
pub fn check_elevation() -> bool {
    #[cfg(target_os = "windows")]
    {
        pristine_winapi::security::is_process_elevated()
    }
    #[cfg(not(target_os = "windows"))]
    {
        true
    }
}

/// Requests UAC elevation by relaunching the application via ShellExecuteW 'runas'.
#[tauri::command]
pub fn request_elevation() -> bool {
    #[cfg(target_os = "windows")]
    {
        let launched = pristine_winapi::security::relaunch_elevated();
        if launched {
            // Exit the unelevated process after a brief moment to allow the elevated instance to start
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_millis(600));
                std::process::exit(0);
            });
            true
        } else {
            false
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        true
    }
}

#[tauri::command]
pub fn get_installed_apps() -> Vec<pristine_winapi::InstalledApp> {
    #[cfg(target_os = "windows")]
    {
        pristine_winapi::get_installed_apps()
    }
    #[cfg(not(target_os = "windows"))]
    {
        Vec::new()
    }
}

#[tauri::command]
pub fn uninstall_app(
    app_id: Option<String>,
    id: Option<String>,
    is_uwp: Option<bool>,
    uninstall_cmd: Option<String>,
) -> OperationResult {
    let target_id = match app_id.or(id) {
        Some(s) if !s.trim().is_empty() => s,
        _ => {
            return OperationResult {
                success: false,
                message: "No se especificó un identificador de aplicación válido.".to_string(),
            };
        }
    };
    let target_is_uwp = is_uwp.unwrap_or(false);
    let target_cmd = uninstall_cmd.unwrap_or_default();

    #[cfg(target_os = "windows")]
    {
        match pristine_winapi::uninstall_application(&target_id, target_is_uwp, &target_cmd) {
            Ok(msg) => OperationResult {
                success: true,
                message: msg,
            },
            Err(e) => OperationResult {
                success: false,
                message: e.to_string(),
            },
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        OperationResult {
            success: true,
            message: format!(
                "Simulación de desinstalación de '{}' en entorno no Windows.",
                target_id
            ),
        }
    }
}

#[tauri::command]
pub fn flush_dns() -> OperationResult {
    #[cfg(target_os = "windows")]
    {
        match pristine_winapi::flush_dns_cache() {
            Ok(msg) => OperationResult {
                success: true,
                message: msg,
            },
            Err(e) => OperationResult {
                success: false,
                message: e.to_string(),
            },
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        OperationResult {
            success: true,
            message: "Caché DNS simulada purgada.".to_string(),
        }
    }
}

#[tauri::command]
pub fn get_hosts_shield_status() -> bool {
    #[cfg(target_os = "windows")]
    {
        pristine_winapi::is_hosts_shield_active()
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

#[tauri::command]
pub fn toggle_hosts_shield(enable: bool) -> OperationResult {
    #[cfg(target_os = "windows")]
    {
        match pristine_winapi::toggle_hosts_shield(enable) {
            Ok(active) => OperationResult {
                success: true,
                message: if active {
                    "Escudo de telemetría hosts activado (dominios redirigidos a 0.0.0.0)."
                        .to_string()
                } else {
                    "Escudo de telemetría hosts desactivado.".to_string()
                },
            },
            Err(e) => OperationResult {
                success: false,
                message: e.to_string(),
            },
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        OperationResult {
            success: true,
            message: "Estado de escudo hosts simulado.".to_string(),
        }
    }
}

#[tauri::command]
pub fn restart_windows_explorer() -> OperationResult {
    #[cfg(target_os = "windows")]
    {
        match pristine_winapi::restart_windows_explorer() {
            Ok(()) => OperationResult {
                success: true,
                message: "Explorador de Windows reiniciado exitosamente.".to_string(),
            },
            Err(e) => OperationResult {
                success: false,
                message: e.to_string(),
            },
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        OperationResult {
            success: true,
            message: "Simulación de reinicio de explorador.".to_string(),
        }
    }
}

#[tauri::command]
pub fn scan_app_residuals(
    app_name: String,
    publisher: String,
) -> pristine_winapi::ResidualScanResult {
    #[cfg(target_os = "windows")]
    {
        pristine_winapi::scan_app_residuals(&app_name, &publisher)
    }
    #[cfg(not(target_os = "windows"))]
    {
        pristine_winapi::ResidualScanResult {
            app_name,
            residuals: Vec::new(),
            total_size_bytes: 0,
        }
    }
}

#[tauri::command]
pub fn clean_app_residuals(paths: Vec<String>) -> OperationResult {
    #[cfg(target_os = "windows")]
    {
        match pristine_winapi::clean_residuals(&paths) {
            Ok(count) => OperationResult {
                success: true,
                message: format!("Se eliminaron exitosamente {} elementos residuales.", count),
            },
            Err(e) => OperationResult {
                success: false,
                message: e.to_string(),
            },
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        OperationResult {
            success: true,
            message: format!(
                "Se simularon la eliminación de {} elementos residuales.",
                paths.len()
            ),
        }
    }
}
