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
use pristine_engine::{apply_tweak, create_transaction_session, revert_session, run_full_audit};
use pristine_metrics::{HardwareMetrics, MetricsCollector};
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

    Ok(session)
}

#[tauri::command]
pub fn revert_transaction(session: TransactionSession) -> Result<(), String> {
    revert_session(&session).map_err(|e| e.to_string())
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
