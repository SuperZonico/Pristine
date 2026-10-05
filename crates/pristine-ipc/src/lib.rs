/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-ipc/src/lib.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Protocol types and IPC contracts for inter-process isolation.
 * ============================================================================
 */

use pristine_core::models::SystemAuditReport;
use pristine_core::transaction::TransactionSession;
use serde::{Deserialize, Serialize};

pub const PRISTINE_PIPE_NAME: &str = r"\\.\pipe\pristine-ipc-secure";

/// Strict SDDL granting Generic All only to SYSTEM (SY), Built-in Administrators (BA),
/// and the Interactive Owner (OW). Remote connections are completely rejected.
pub const PRISTINE_PIPE_SDDL: &str = "D:(A;;GA;;;SY)(A;;GA;;;BA)(A;;GA;;;OW)";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", content = "payload", rename_all = "snake_case")]
pub enum IpcRequest {
    AuditSystem,
    ApplyTweaks {
        tweak_ids: Vec<String>,
        description: String,
    },
    RevertSession {
        session: TransactionSession,
    },
    GetHardwareMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "response", content = "payload", rename_all = "snake_case")]
pub enum IpcResponse {
    AuditReport(SystemAuditReport),
    TweaksApplied { session: TransactionSession },
    SessionReverted,
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_serialization() {
        let req = IpcRequest::ApplyTweaks {
            tweak_ids: vec!["diagtrack_utc".to_string()],
            description: "Test Session".to_string(),
        };

        let json = serde_json::to_string(&req).expect("Failed to serialize request");
        assert!(json.contains("apply_tweaks"));

        let deserialized: IpcRequest =
            serde_json::from_str(&json).expect("Failed to deserialize request");
        match deserialized {
            IpcRequest::ApplyTweaks { tweak_ids, .. } => {
                assert_eq!(tweak_ids.len(), 1);
                assert_eq!(tweak_ids[0], "diagtrack_utc");
            }
            _ => panic!("Expected ApplyTweaks variant"),
        }
    }
}
