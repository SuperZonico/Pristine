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
pub mod journal;
pub mod rollback;

pub use auditor::{audit_tweak, run_full_audit};
pub use executor::{apply_tweak, create_transaction_session, EngineError};
pub use journal::{load_journal, remove_session, save_session, JournalError};
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

    #[test]
    fn test_journal_roundtrip() {
        let session = create_transaction_session("Test session".to_string());
        assert!(session.verify_integrity());
    }
}
