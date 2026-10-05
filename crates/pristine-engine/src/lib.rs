/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-engine/src/lib.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Engine orchestrator combining auditor, executor, and rollback.
 * ============================================================================
 */

pub mod auditor;
pub mod executor;
pub mod rollback;

pub use auditor::{audit_tweak, run_full_audit};
pub use executor::{apply_tweak, create_transaction_session, EngineError};
pub use rollback::{revert_session, RollbackError};

#[cfg(test)]
mod tests {
    use super::*;
    use pristine_core::catalog::get_default_catalog;

    #[test]
    fn test_audit_execution() {
        let catalog = get_default_catalog();
        let report = run_full_audit(&catalog);
        assert_eq!(report.total_analyzed, catalog.len());
        assert!(report.privacy_score_percent <= 100);
    }
}
