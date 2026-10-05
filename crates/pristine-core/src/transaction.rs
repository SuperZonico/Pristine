/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-core/src/transaction.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Deterministic rollback journal types with SHA-256 integrity.
 * ============================================================================
 */

use crate::models::{RegistryRoot, RegistryValueKind, ServiceStartupMode};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryRollbackEntry {
    pub root: RegistryRoot,
    pub subkey: String,
    pub value_name: String,
    pub previous_value: Option<RegistryValueKind>,
    pub created_key_if_missing: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceRollbackEntry {
    pub service_name: String,
    pub previous_startup: ServiceStartupMode,
    pub was_running: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskRollbackEntry {
    pub task_path: String,
    pub task_name: String,
    pub previously_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionSession {
    pub session_id: String,
    pub created_at_utc: u64,
    pub description: String,
    pub registry_rollbacks: Vec<RegistryRollbackEntry>,
    pub service_rollbacks: Vec<ServiceRollbackEntry>,
    pub task_rollbacks: Vec<TaskRollbackEntry>,
    pub integrity_hash: String,
}

impl TransactionSession {
    pub fn new(session_id: String, created_at_utc: u64, description: String) -> Self {
        let mut session = Self {
            session_id,
            created_at_utc,
            description,
            registry_rollbacks: Vec::new(),
            service_rollbacks: Vec::new(),
            task_rollbacks: Vec::new(),
            integrity_hash: String::new(),
        };
        session.recalculate_hash();
        session
    }

    pub fn recalculate_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.session_id.as_bytes());
        hasher.update(self.created_at_utc.to_le_bytes());
        hasher.update(self.description.as_bytes());

        if let Ok(reg_bytes) = serde_json::to_vec(&self.registry_rollbacks) {
            hasher.update(&reg_bytes);
        }
        if let Ok(srv_bytes) = serde_json::to_vec(&self.service_rollbacks) {
            hasher.update(&srv_bytes);
        }
        if let Ok(task_bytes) = serde_json::to_vec(&self.task_rollbacks) {
            hasher.update(&task_bytes);
        }

        self.integrity_hash = format!("{:x}", hasher.finalize());
    }

    pub fn verify_integrity(&self) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(self.session_id.as_bytes());
        hasher.update(self.created_at_utc.to_le_bytes());
        hasher.update(self.description.as_bytes());

        if let Ok(reg_bytes) = serde_json::to_vec(&self.registry_rollbacks) {
            hasher.update(&reg_bytes);
        }
        if let Ok(srv_bytes) = serde_json::to_vec(&self.service_rollbacks) {
            hasher.update(&srv_bytes);
        }
        if let Ok(task_bytes) = serde_json::to_vec(&self.task_rollbacks) {
            hasher.update(&task_bytes);
        }

        let computed = format!("{:x}", hasher.finalize());
        computed == self.integrity_hash
    }
}
