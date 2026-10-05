/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-engine/src/rollback.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Reverts transaction sessions verified with cryptographic hash.
 * ============================================================================
 */

use pristine_core::transaction::TransactionSession;
use pristine_winapi::services::configure_service;
use pristine_winapi::{delete_registry_value, write_registry_value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RollbackError {
    #[error("Integrity verification failed for session '{0}'. File was tampered with.")]
    IntegrityTampered(String),

    #[error("Failed to restore registry key: {0}")]
    RegistryRestore(String),

    #[error("Failed to restore service: {0}")]
    ServiceRestore(String),
}

pub fn revert_session(session: &TransactionSession) -> Result<(), RollbackError> {
    if !session.verify_integrity() {
        return Err(RollbackError::IntegrityTampered(session.session_id.clone()));
    }

    // Revert registry mutations in reverse order
    for entry in session.registry_rollbacks.iter().rev() {
        if let Some(ref prior_val) = entry.previous_value {
            let _ = write_registry_value(&entry.root, &entry.subkey, &entry.value_name, prior_val);
        } else {
            // Delete value if it did not exist before
            let _ = delete_registry_value(&entry.root, &entry.subkey, &entry.value_name);
        }
    }

    // Revert service configurations
    for srv in session.service_rollbacks.iter().rev() {
        let _ = configure_service(&srv.service_name, srv.previous_startup, false);
    }

    // Revert task configurations
    for task in session.task_rollbacks.iter().rev() {
        let _ = pristine_winapi::configure_task(
            &task.task_path,
            &task.task_name,
            task.previously_enabled,
        );
    }

    Ok(())
}
