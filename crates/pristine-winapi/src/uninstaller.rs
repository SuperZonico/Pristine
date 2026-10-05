/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-winapi/src/uninstaller.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Native Win32 and UWP software discovery and uninstallation engine.
 * ============================================================================
 */

use std::collections::HashSet;
use std::os::windows::process::CommandExt;
use std::process::Command;

use serde::{Deserialize, Serialize};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{ERROR_NO_MORE_ITEMS, ERROR_SUCCESS};
use windows::Win32::System::Registry::{
    RegCloseKey, RegEnumKeyExW, RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_CURRENT_USER,
    HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY, REG_DWORD, REG_SZ,
    REG_VALUE_TYPE,
};

use crate::error::WinApiError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledApp {
    pub id: String,
    pub name: String,
    pub publisher: String,
    pub version: String,
    pub install_date: String,
    pub estimated_size_mb: u64,
    pub uninstall_cmd: String,
    pub quiet_uninstall_cmd: Option<String>,
    pub is_uwp: bool,
    pub is_system_component: bool,
    pub category: String, // "bloatware", "user", "system"
}

#[derive(Deserialize)]
struct UwpJsonItem {
    #[serde(default)]
    #[serde(rename = "Name")]
    name: String,
    #[serde(default)]
    #[serde(rename = "PackageFullName")]
    package_full_name: String,
    #[serde(default)]
    #[serde(rename = "Publisher")]
    publisher: String,
    #[serde(default)]
    #[serde(rename = "Version")]
    version: String,
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn from_wide(wide: &[u16]) -> String {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..len])
}

const KNOWN_BLOATWARE_PATTERNS: &[&str] = &[
    "Microsoft.BingNews",
    "Microsoft.BingWeather",
    "Microsoft.GetHelp",
    "Microsoft.Getstarted",
    "Microsoft.MicrosoftSolitaireCollection",
    "Microsoft.People",
    "Microsoft.Todos",
    "Microsoft.WindowsFeedbackHub",
    "Microsoft.YourPhone",
    "MicrosoftTeams",
    "Clipchamp.Clipchamp",
    "Microsoft.549981C3F5F10", // Cortana
    "Microsoft.ZuneMusic",
    "Microsoft.ZuneVideo",
    "Microsoft.GamingApp",
    "Microsoft.XboxGamingOverlay",
    "Microsoft.XboxGameOverlay",
    "Microsoft.XboxSpeechToTextOverlay",
    "Microsoft.XboxIdentityProvider",
    "CandyCrush",
    "TikTok",
    "Disney",
];

const PROTECTED_SYSTEM_PATTERNS: &[&str] = &[
    "Microsoft Visual C++",
    "Microsoft Edge WebView",
    "DirectX",
    "Windows App SDK",
    "Microsoft .NET",
    "Microsoft.WindowsStore",
    "Microsoft.DesktopAppInstaller",
    "Microsoft.StorePurchaseApp",
    "Security Health",
];

fn classify_app(name: &str, id: &str, is_system: bool) -> String {
    let lower_name = name.to_lowercase();
    let lower_id = id.to_lowercase();

    // Check protected system first
    for pattern in PROTECTED_SYSTEM_PATTERNS {
        let p_lower = pattern.to_lowercase();
        if lower_name.contains(&p_lower) || lower_id.contains(&p_lower) {
            return "system".to_string();
        }
    }

    if is_system {
        return "system".to_string();
    }

    // Check bloatware patterns
    for pattern in KNOWN_BLOATWARE_PATTERNS {
        let p_lower = pattern.to_lowercase();
        if lower_name.contains(&p_lower) || lower_id.contains(&p_lower) {
            return "bloatware".to_string();
        }
    }

    "user".to_string()
}

fn query_reg_string(hkey: HKEY, value_name: &str) -> Option<String> {
    let val_w = to_wide(value_name);
    let mut data_type = REG_VALUE_TYPE::default();
    let mut data_size = 0u32;

    let res = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(val_w.as_ptr()),
            None,
            Some(&mut data_type),
            None,
            Some(&mut data_size),
        )
    };

    if res != ERROR_SUCCESS || data_type != REG_SZ || data_size == 0 {
        return None;
    }

    let mut buf = vec![0u8; data_size as usize];
    let res2 = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(val_w.as_ptr()),
            None,
            Some(&mut data_type),
            Some(buf.as_mut_ptr()),
            Some(&mut data_size),
        )
    };

    if res2 == ERROR_SUCCESS {
        let u16_slice = unsafe {
            std::slice::from_raw_parts(buf.as_ptr() as *const u16, (data_size / 2) as usize)
        };
        Some(from_wide(u16_slice).trim().to_string())
    } else {
        None
    }
}

fn query_reg_dword(hkey: HKEY, value_name: &str) -> Option<u32> {
    let val_w = to_wide(value_name);
    let mut data_type = REG_VALUE_TYPE::default();
    let mut val = 0u32;
    let mut data_size = std::mem::size_of::<u32>() as u32;

    let res = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(val_w.as_ptr()),
            None,
            Some(&mut data_type),
            Some(&mut val as *mut _ as *mut u8),
            Some(&mut data_size),
        )
    };

    if res == ERROR_SUCCESS && data_type == REG_DWORD {
        Some(val)
    } else {
        None
    }
}

fn scan_registry_uninstall_key(
    root: HKEY,
    subkey: &str,
    extra_flags: u32,
    apps: &mut Vec<InstalledApp>,
    seen_ids: &mut HashSet<String>,
) {
    let subkey_w = to_wide(subkey);
    let mut hkey = HKEY::default();

    let open_res = unsafe {
        RegOpenKeyExW(
            root,
            PCWSTR(subkey_w.as_ptr()),
            0,
            KEY_READ | windows::Win32::System::Registry::REG_SAM_FLAGS(extra_flags),
            &mut hkey,
        )
    };

    if open_res != ERROR_SUCCESS {
        return;
    }

    let mut index = 0u32;
    let mut name_buf = [0u16; 256];

    loop {
        let mut name_len = name_buf.len() as u32;
        let enum_res = unsafe {
            RegEnumKeyExW(
                hkey,
                index,
                windows::core::PWSTR(name_buf.as_mut_ptr()),
                &mut name_len,
                None,
                windows::core::PWSTR::null(),
                None,
                None,
            )
        };

        if enum_res == ERROR_NO_MORE_ITEMS {
            break;
        }

        if enum_res == ERROR_SUCCESS {
            let sub_name = from_wide(&name_buf[..name_len as usize]);
            let child_path = format!(r"{}\{}", subkey, sub_name);
            let child_path_w = to_wide(&child_path);
            let mut child_hkey = HKEY::default();

            let child_open = unsafe {
                RegOpenKeyExW(
                    root,
                    PCWSTR(child_path_w.as_ptr()),
                    0,
                    KEY_READ | windows::Win32::System::Registry::REG_SAM_FLAGS(extra_flags),
                    &mut child_hkey,
                )
            };

            if child_open == ERROR_SUCCESS {
                let display_name = query_reg_string(child_hkey, "DisplayName");
                let uninstall_string = query_reg_string(child_hkey, "UninstallString");
                let is_system = query_reg_dword(child_hkey, "SystemComponent").unwrap_or(0) == 1;
                let parent_key = query_reg_string(child_hkey, "ParentKeyName");

                // Only include items that have a DisplayName and an UninstallString (and are not sub-patches)
                if let (Some(name), Some(uninstall)) = (display_name, uninstall_string) {
                    if !name.trim().is_empty() && parent_key.is_none() {
                        let unique_id = format!("{}:{}", name, sub_name);
                        if !seen_ids.contains(&unique_id) {
                            seen_ids.insert(unique_id.clone());

                            let version = query_reg_string(child_hkey, "DisplayVersion")
                                .unwrap_or_else(|| "1.0".to_string());
                            let publisher = query_reg_string(child_hkey, "Publisher")
                                .unwrap_or_else(|| "Desconocido".to_string());
                            let install_date =
                                query_reg_string(child_hkey, "InstallDate").unwrap_or_default();
                            let estimated_kb =
                                query_reg_dword(child_hkey, "EstimatedSize").unwrap_or(0);
                            let quiet_uninstall =
                                query_reg_string(child_hkey, "QuietUninstallString");
                            let category = classify_app(&name, &sub_name, is_system);

                            apps.push(InstalledApp {
                                id: unique_id,
                                name,
                                publisher,
                                version,
                                install_date,
                                estimated_size_mb: (estimated_kb as u64) / 1024,
                                uninstall_cmd: uninstall,
                                quiet_uninstall_cmd: quiet_uninstall,
                                is_uwp: false,
                                is_system_component: is_system,
                                category,
                            });
                        }
                    }
                }

                unsafe {
                    let _ = RegCloseKey(child_hkey);
                }
            }
        }

        index += 1;
    }

    unsafe {
        let _ = RegCloseKey(hkey);
    }
}

fn scan_uwp_apps(apps: &mut Vec<InstalledApp>, seen_ids: &mut HashSet<String>) {
    let mut cmd = Command::new("powershell.exe");
    cmd.args([
        "-NoProfile",
        "-Command",
        r#"Get-AppxPackage -AllUsers | Where-Object { -not $_.IsFramework -and $_.NonRemovable -ne $true } | Select-Object Name, PackageFullName, Publisher, Version | ConvertTo-Json -Compress"#,
    ]);
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

    if let Ok(output) = cmd.output() {
        if output.status.success() {
            let json_str = String::from_utf8_lossy(&output.stdout);
            if json_str.trim().starts_with('[') {
                if let Ok(items) = serde_json::from_str::<Vec<UwpJsonItem>>(&json_str) {
                    for item in items {
                        add_uwp_item(item, apps, seen_ids);
                    }
                }
            } else if json_str.trim().starts_with('{') {
                if let Ok(item) = serde_json::from_str::<UwpJsonItem>(&json_str) {
                    add_uwp_item(item, apps, seen_ids);
                }
            }
        }
    }
}

fn add_uwp_item(item: UwpJsonItem, apps: &mut Vec<InstalledApp>, seen_ids: &mut HashSet<String>) {
    if item.package_full_name.is_empty() || seen_ids.contains(&item.package_full_name) {
        return;
    }
    seen_ids.insert(item.package_full_name.clone());

    let category = classify_app(&item.name, &item.package_full_name, false);

    // Format cleaner display name for UWP
    let display_name = item.name.replace("Microsoft.", "").replace("CN=", "");

    apps.push(InstalledApp {
        id: item.package_full_name.clone(),
        name: display_name,
        publisher: if item.publisher.trim().is_empty() {
            "Microsoft Corporation / UWP".to_string()
        } else {
            item.publisher.replace("CN=", "")
        },
        version: item.version,
        install_date: "UWP Store Package".to_string(),
        estimated_size_mb: 0,
        uninstall_cmd: format!(
            "Remove-AppxPackage -Package '{}' -AllUsers",
            item.package_full_name
        ),
        quiet_uninstall_cmd: Some(format!(
            "Remove-AppxPackage -Package '{}' -AllUsers",
            item.package_full_name
        )),
        is_uwp: true,
        is_system_component: category == "system",
        category,
    });
}

/// Retrieves all installed Win32 software and UWP applications on the system.
pub fn get_installed_apps() -> Vec<InstalledApp> {
    let mut apps = Vec::new();
    let mut seen_ids = HashSet::new();

    // 1. Scan 64-bit Registry Uninstall
    scan_registry_uninstall_key(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        KEY_WOW64_64KEY.0,
        &mut apps,
        &mut seen_ids,
    );

    // 2. Scan 32-bit Registry Uninstall (WOW6432Node)
    scan_registry_uninstall_key(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
        KEY_WOW64_32KEY.0,
        &mut apps,
        &mut seen_ids,
    );

    // 3. Scan Current User Registry Uninstall
    scan_registry_uninstall_key(
        HKEY_CURRENT_USER,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        0,
        &mut apps,
        &mut seen_ids,
    );

    // 4. Scan Windows 11 UWP Apps (Bloatware)
    scan_uwp_apps(&mut apps, &mut seen_ids);

    // Sort: Bloatware first, then user apps, then system
    apps.sort_by(|a, b| {
        let order = |cat: &str| match cat {
            "bloatware" => 0,
            "user" => 1,
            _ => 2,
        };
        order(&a.category)
            .cmp(&order(&b.category))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    apps
}

/// Executes uninstallation for a specific application.
pub fn uninstall_application(
    app_id: &str,
    is_uwp: bool,
    uninstall_cmd: &str,
) -> Result<String, WinApiError> {
    if is_uwp {
        let mut cmd = Command::new("powershell.exe");
        cmd.args([
            "-NoProfile",
            "-Command",
            &format!("Remove-AppxPackage -Package '{}' -AllUsers", app_id),
        ]);
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        let output = cmd.output().map_err(|e| WinApiError::ServiceError {
            service: "Appx".to_string(),
            code: 1,
            message: format!("Error al invocar PowerShell para desinstalar UWP: {}", e),
        })?;

        if output.status.success() {
            Ok(format!(
                "Paquete UWP '{}' desinstalado exitosamente.",
                app_id
            ))
        } else {
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            Err(WinApiError::ServiceError {
                service: "Appx".to_string(),
                code: output.status.code().unwrap_or(-1) as u32,
                message: format!("Fallo al remover paquete Appx: {}", err),
            })
        }
    } else {
        // Win32 Uninstallation
        let mut cmd = Command::new("cmd.exe");
        cmd.args(["/c", uninstall_cmd]);
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        let output = cmd.output().map_err(|e| WinApiError::ServiceError {
            service: "Uninstaller".to_string(),
            code: 1,
            message: format!("Error al invocar el desinstalador Win32: {}", e),
        })?;

        if output.status.success() {
            Ok("Desinstalador ejecutado con éxito.".to_string())
        } else {
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            Err(WinApiError::ServiceError {
                service: "Uninstaller".to_string(),
                code: output.status.code().unwrap_or(-1) as u32,
                message: format!("El desinstalador retornó código de error: {}", err),
            })
        }
    }
}
