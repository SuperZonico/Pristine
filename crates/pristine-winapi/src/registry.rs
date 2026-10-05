/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-winapi/src/registry.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Safe native Win32 Registry bindings with rollback capture.
 * ============================================================================
 */

use pristine_core::models::{RegistryRoot, RegistryValueKind};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS, WIN32_ERROR};
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW,
    HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_SET_VALUE, REG_DWORD,
    REG_OPTION_NON_VOLATILE, REG_QWORD, REG_SAM_FLAGS, REG_SZ, REG_VALUE_TYPE,
};

use crate::error::WinApiError;

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn map_root(root: &RegistryRoot) -> HKEY {
    match root {
        RegistryRoot::HkeyLocalMachine => HKEY_LOCAL_MACHINE,
        RegistryRoot::HkeyCurrentUser => HKEY_CURRENT_USER,
    }
}

pub struct SafeRegKey {
    handle: HKEY,
}

impl SafeRegKey {
    pub fn open(
        root: &RegistryRoot,
        subkey: &str,
        sam_desired: REG_SAM_FLAGS,
    ) -> Result<Self, WinApiError> {
        let root_hkey = map_root(root);
        let subkey_w = to_wide(subkey);
        let mut handle = HKEY::default();

        let status = unsafe {
            RegOpenKeyExW(
                root_hkey,
                PCWSTR(subkey_w.as_ptr()),
                0,
                sam_desired,
                &mut handle,
            )
        };

        if status == ERROR_SUCCESS {
            Ok(Self { handle })
        } else {
            Err(WinApiError::RegistryError {
                key: subkey.to_string(),
                code: status.0,
                message: format!("Failed to open registry key: error code {}", status.0),
            })
        }
    }

    pub fn create_or_open(
        root: &RegistryRoot,
        subkey: &str,
        sam_desired: REG_SAM_FLAGS,
    ) -> Result<Self, WinApiError> {
        let root_hkey = map_root(root);
        let subkey_w = to_wide(subkey);
        let mut handle = HKEY::default();

        let status = unsafe {
            RegCreateKeyExW(
                root_hkey,
                PCWSTR(subkey_w.as_ptr()),
                0,
                None,
                REG_OPTION_NON_VOLATILE,
                sam_desired,
                None,
                &mut handle,
                None,
            )
        };

        if status == ERROR_SUCCESS {
            Ok(Self { handle })
        } else {
            Err(WinApiError::RegistryError {
                key: subkey.to_string(),
                code: status.0,
                message: format!(
                    "Failed to create/open registry key: error code {}",
                    status.0
                ),
            })
        }
    }

    pub fn query_value(&self, value_name: &str) -> Result<Option<RegistryValueKind>, WinApiError> {
        let value_name_w = to_wide(value_name);
        let mut value_type = REG_VALUE_TYPE::default();
        let mut data_size: u32 = 0;

        // Query size and type first
        let status = unsafe {
            RegQueryValueExW(
                self.handle,
                PCWSTR(value_name_w.as_ptr()),
                None,
                Some(&mut value_type),
                None,
                Some(&mut data_size),
            )
        };

        if status == ERROR_FILE_NOT_FOUND {
            return Ok(None);
        } else if status != ERROR_SUCCESS {
            return Err(WinApiError::RegistryError {
                key: value_name.to_string(),
                code: status.0,
                message: format!("Failed to query registry value size: {}", status.0),
            });
        }

        let mut buffer = vec![0u8; data_size as usize];
        let status = unsafe {
            RegQueryValueExW(
                self.handle,
                PCWSTR(value_name_w.as_ptr()),
                None,
                Some(&mut value_type),
                Some(buffer.as_mut_ptr()),
                Some(&mut data_size),
            )
        };

        if status != ERROR_SUCCESS {
            return Err(WinApiError::RegistryError {
                key: value_name.to_string(),
                code: status.0,
                message: format!("Failed to query registry value data: {}", status.0),
            });
        }

        match value_type {
            REG_DWORD => {
                if buffer.len() >= 4 {
                    let bytes: [u8; 4] = buffer[0..4].try_into().unwrap();
                    Ok(Some(RegistryValueKind::Dword(u32::from_ne_bytes(bytes))))
                } else {
                    Ok(None)
                }
            }
            REG_QWORD => {
                if buffer.len() >= 8 {
                    let bytes: [u8; 8] = buffer[0..8].try_into().unwrap();
                    Ok(Some(RegistryValueKind::Qword(u64::from_ne_bytes(bytes))))
                } else {
                    Ok(None)
                }
            }
            REG_SZ => {
                let u16_slice: &[u16] = unsafe {
                    std::slice::from_raw_parts(buffer.as_ptr() as *const u16, buffer.len() / 2)
                };
                let parsed = String::from_utf16_lossy(u16_slice)
                    .trim_matches(char::from(0))
                    .to_string();
                Ok(Some(RegistryValueKind::String(parsed)))
            }
            _ => Ok(None),
        }
    }

    pub fn set_value(
        &self,
        value_name: &str,
        value: &RegistryValueKind,
    ) -> Result<(), WinApiError> {
        let value_name_w = to_wide(value_name);

        let (val_type, bytes): (REG_VALUE_TYPE, Vec<u8>) = match value {
            RegistryValueKind::Dword(v) => (REG_DWORD, v.to_ne_bytes().to_vec()),
            RegistryValueKind::Qword(v) => (REG_QWORD, v.to_ne_bytes().to_vec()),
            RegistryValueKind::String(s) => {
                let wide = to_wide(s);
                let byte_slice: &[u8] = unsafe {
                    std::slice::from_raw_parts(wide.as_ptr() as *const u8, wide.len() * 2)
                };
                (REG_SZ, byte_slice.to_vec())
            }
        };

        let status: WIN32_ERROR = unsafe {
            RegSetValueExW(
                self.handle,
                PCWSTR(value_name_w.as_ptr()),
                0,
                val_type,
                Some(&bytes),
            )
        };

        if status == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(WinApiError::RegistryError {
                key: value_name.to_string(),
                code: status.0,
                message: format!("Failed to set registry value: error code {}", status.0),
            })
        }
    }

    pub fn delete_value(&self, value_name: &str) -> Result<(), WinApiError> {
        let value_name_w = to_wide(value_name);
        let status = unsafe { RegDeleteValueW(self.handle, PCWSTR(value_name_w.as_ptr())) };

        if status == ERROR_SUCCESS || status == ERROR_FILE_NOT_FOUND {
            Ok(())
        } else {
            Err(WinApiError::RegistryError {
                key: value_name.to_string(),
                code: status.0,
                message: format!("Failed to delete registry value: error code {}", status.0),
            })
        }
    }
}

impl Drop for SafeRegKey {
    fn drop(&mut self) {
        if !self.handle.is_invalid() {
            unsafe {
                let _ = RegCloseKey(self.handle);
            }
        }
    }
}

pub fn read_registry_value(
    root: &RegistryRoot,
    subkey: &str,
    value_name: &str,
) -> Result<Option<RegistryValueKind>, WinApiError> {
    match SafeRegKey::open(root, subkey, KEY_READ) {
        Ok(key) => key.query_value(value_name),
        Err(WinApiError::RegistryError { code, .. }) if code == ERROR_FILE_NOT_FOUND.0 => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn write_registry_value(
    root: &RegistryRoot,
    subkey: &str,
    value_name: &str,
    value: &RegistryValueKind,
) -> Result<Option<RegistryValueKind>, WinApiError> {
    let key = SafeRegKey::create_or_open(root, subkey, KEY_READ | KEY_SET_VALUE)?;
    let previous = key.query_value(value_name)?;
    key.set_value(value_name, value)?;
    Ok(previous)
}

pub fn delete_registry_value(
    root: &RegistryRoot,
    subkey: &str,
    value_name: &str,
) -> Result<(), WinApiError> {
    if let Ok(key) = SafeRegKey::open(root, subkey, KEY_SET_VALUE) {
        key.delete_value(value_name)?;
    }
    Ok(())
}
