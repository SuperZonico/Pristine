/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-core/src/lib.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Core library exports, data structures, and catalog definitions.
 * ============================================================================
 */

#![forbid(unsafe_code)]

pub mod catalog;
pub mod models;
pub mod transaction;

pub use catalog::get_default_catalog;
pub use models::*;
pub use transaction::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_integrity() {
        let catalog = get_default_catalog();
        assert!(!catalog.is_empty(), "Catalog should not be empty");
        for tweak in &catalog {
            assert!(!tweak.id.is_empty(), "Tweak ID must be defined");
            assert!(
                !tweak.actions.is_empty(),
                "Tweak must contain at least one action"
            );
        }
    }

    #[test]
    fn test_transaction_hash_verification() {
        let mut session = TransactionSession::new(
            "sess_001".to_string(),
            1728000000,
            "Initial Optimization".to_string(),
        );

        assert!(
            session.verify_integrity(),
            "Fresh session should have valid integrity hash"
        );

        session.description = "Tampered Description".to_string();
        assert!(
            !session.verify_integrity(),
            "Tampered session must fail integrity verification"
        );
    }
}
