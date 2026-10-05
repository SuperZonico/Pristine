/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-winapi/src/security.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Process elevation checks and UAC privilege management.
 * ============================================================================
 */

use std::os::windows::ffi::OsStrExt;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE, HWND};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD;

/// Determines whether the current process is running with elevated (Administrator) privileges.
pub fn is_process_elevated() -> bool {
    unsafe {
        let mut token = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
            let mut elevation = TOKEN_ELEVATION::default();
            let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            let res = GetTokenInformation(
                token,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                size,
                &mut size,
            );
            let _ = CloseHandle(token);
            if res.is_ok() {
                return elevation.TokenIsElevated != 0;
            }
        }
        false
    }
}

/// Attempts to relaunch the current application with elevated privileges via UAC prompt ("runas").
pub fn relaunch_elevated() -> bool {
    if let Ok(exe_path) = std::env::current_exe() {
        let exe_wide: Vec<u16> = exe_path
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let runas_wide: Vec<u16> = "runas\0".encode_utf16().collect();

        unsafe {
            let result = ShellExecuteW(
                HWND::default(),
                PCWSTR(runas_wide.as_ptr()),
                PCWSTR(exe_wide.as_ptr()),
                PCWSTR(std::ptr::null()),
                PCWSTR(std::ptr::null()),
                SHOW_WINDOW_CMD(1), // SW_SHOWNORMAL
            );
            (result.0 as usize) > 32
        }
    } else {
        false
    }
}
