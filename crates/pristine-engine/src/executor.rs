/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-engine/src/executor.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Applies tweaks with automatic rollback journal generation.
 * ============================================================================
 */

use pristine_core::models::{TweakAction, TweakDefinition};
use pristine_core::transaction::{
    RegistryRollbackEntry, ServiceRollbackEntry, TaskRollbackEntry, TransactionSession,
};
use pristine_winapi::{
    configure_service, configure_task, query_task_enabled, write_registry_value, WinApiError,
};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("API failure: {0}")]
    Api(#[from] WinApiError),

    #[error("Session serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Tweak not found: {0}")]
    NotFound(String),
}

pub fn apply_tweak(
    tweak: &TweakDefinition,
    session: &mut TransactionSession,
) -> Result<(), EngineError> {
    for action in &tweak.actions {
        match action {
            TweakAction::Registry(reg) => {
                let prior = write_registry_value(
                    &reg.root,
                    &reg.subkey,
                    &reg.value_name,
                    &reg.target_value,
                )?;

                session.registry_rollbacks.push(RegistryRollbackEntry {
                    root: reg.root.clone(),
                    subkey: reg.subkey.clone(),
                    value_name: reg.value_name.clone(),
                    previous_value: prior,
                    created_key_if_missing: false,
                });
            }
            TweakAction::Service(srv) => {
                let prior =
                    configure_service(&srv.service_name, srv.target_startup, srv.stop_if_running)?;

                if let Some(status) = prior {
                    session.service_rollbacks.push(ServiceRollbackEntry {
                        service_name: srv.service_name.clone(),
                        previous_startup: status.startup_mode,
                        was_running: status.is_running,
                    });
                }
            }
            TweakAction::Task(task) => {
                let prior = query_task_enabled(&task.task_path, &task.task_name);
                let _ = configure_task(&task.task_path, &task.task_name, task.enable);
                session.task_rollbacks.push(TaskRollbackEntry {
                    task_path: task.task_path.clone(),
                    task_name: task.task_name.clone(),
                    previously_enabled: prior,
                });
            }
        }
    }

    session.recalculate_hash();
    Ok(())
}

pub fn create_transaction_session(description: String) -> TransactionSession {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let session_id = format!("sess_{}", now);
    TransactionSession::new(session_id, now, description)
}
