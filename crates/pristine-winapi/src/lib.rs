/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-winapi/src/lib.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Windows API safe abstractions and FFI wrapper exports.
 * ============================================================================
 */

pub mod error;
pub mod registry;
pub mod services;

pub use error::WinApiError;
pub use registry::{delete_registry_value, read_registry_value, write_registry_value, SafeRegKey};
pub use services::{configure_service, query_service, ServiceStatusInfo};

#[cfg(test)]
mod tests {
    use super::*;
    use pristine_core::models::{RegistryRoot, RegistryValueKind};

    #[test]
    fn test_registry_read_system_root() {
        // Read CurrentVersion ProductName or SystemRoot which always exists on Windows
        let val = read_registry_value(
            &RegistryRoot::HkeyLocalMachine,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
            "SystemRoot",
        );

        assert!(
            val.is_ok(),
            "Reading standard Windows NT registry key should succeed"
        );
        if let Ok(Some(RegistryValueKind::String(path))) = val {
            assert!(!path.is_empty(), "SystemRoot should not be empty");
        }
    }
}
