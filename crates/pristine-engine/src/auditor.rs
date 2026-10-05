/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-engine/src/auditor.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Audits live Windows 11 system state against the tweak catalog.
 * ============================================================================
 */

use pristine_core::models::{
    SystemAuditReport, TweakAction, TweakAuditStatus, TweakDefinition, TweakState,
};
use pristine_winapi::{query_service, query_task_enabled, read_registry_value};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn audit_tweak(tweak: &TweakDefinition) -> TweakAuditStatus {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut all_match = true;
    let mut any_match = false;

    for action in &tweak.actions {
        match action {
            TweakAction::Registry(reg) => {
                match read_registry_value(&reg.root, &reg.subkey, &reg.value_name) {
                    Ok(Some(current_val)) if current_val == reg.target_value => {
                        any_match = true;
                    }
                    _ => {
                        all_match = false;
                    }
                }
            }
            TweakAction::Service(srv) => match query_service(&srv.service_name) {
                Ok(Some(status)) if status.startup_mode == srv.target_startup => {
                    any_match = true;
                }
                _ => {
                    all_match = false;
                }
            },
            TweakAction::Task(task) => {
                let is_enabled = query_task_enabled(&task.task_path, &task.task_name);
                if is_enabled == task.enable {
                    any_match = true;
                } else {
                    all_match = false;
                }
            }
        }
    }

    let state = if all_match && any_match {
        TweakState::Active
    } else if any_match {
        TweakState::Partial
    } else {
        TweakState::Inactive
    };

    TweakAuditStatus {
        tweak_id: tweak.id.clone(),
        state,
        last_checked_epoch: now,
    }
}

pub fn run_full_audit(catalog: &[TweakDefinition]) -> SystemAuditReport {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut statuses = Vec::new();
    let mut active_count = 0;
    let mut inactive_count = 0;

    for tweak in catalog {
        let status = audit_tweak(tweak);
        match status.state {
            TweakState::Active => active_count += 1,
            TweakState::Inactive | TweakState::Partial => inactive_count += 1,
            TweakState::Unknown => {}
        }
        statuses.push(status);
    }

    let total = catalog.len();
    let privacy_score_percent = if total > 0 {
        ((active_count as f32 / total as f32) * 100.0).round() as u8
    } else {
        0
    };

    SystemAuditReport {
        timestamp_utc: now,
        total_analyzed: total,
        active_count,
        inactive_count,
        privacy_score_percent,
        statuses,
    }
}
