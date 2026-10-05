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
pub mod network;
pub mod registry;
pub mod security;
pub mod services;
pub mod storage;
pub mod tasks;
pub mod uninstaller;

pub use error::WinApiError;
pub use network::{
    flush_dns_cache, is_hosts_shield_active, restart_windows_explorer, toggle_hosts_shield,
};
pub use registry::{delete_registry_value, read_registry_value, write_registry_value, SafeRegKey};
pub use security::{is_process_elevated, relaunch_elevated};
pub use services::{configure_service, query_service, ServiceStatusInfo};
pub use storage::{delete_file_safely, reveal_in_explorer, scan_large_stale_files, LargeStaleFile};
pub use tasks::{configure_task, query_task_enabled};
pub use uninstaller::{
    clean_residuals, get_installed_apps, scan_app_residuals, uninstall_application, InstalledApp,
    ResidualItem, ResidualScanResult,
};

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
