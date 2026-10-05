/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-winapi/src/network.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      DNS cache flush, hosts file telemetry sinkhole, and shell refresh.
 * ============================================================================
 */

use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

use crate::error::WinApiError;

const HOSTS_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts";
const SHIELD_START_MARKER: &str = "# === BEGIN PRISTINE TELEMETRY SHIELD ===";
const SHIELD_END_MARKER: &str = "# === END PRISTINE TELEMETRY SHIELD ===";

/// List of critical Microsoft telemetry, tracking, and diagnostic endpoints
pub const TELEMETRY_DOMAINS: &[&str] = &[
    "telemetry.microsoft.com",
    "watson.telemetry.microsoft.com",
    "watson.telemetry.microsoft.com.nsatc.net",
    "watson.ppe.telemetry.microsoft.com",
    "watson.microsoft.com",
    "watson.live.com",
    "vortex.data.microsoft.com",
    "vortex-win.data.microsoft.com",
    "telecommand.telemetry.microsoft.com",
    "telecommand.telemetry.microsoft.com.nsatc.net",
    "oca.telemetry.microsoft.com",
    "oca.telemetry.microsoft.com.nsatc.net",
    "sqm.telemetry.microsoft.com",
    "sqm.telemetry.microsoft.com.nsatc.net",
    "sq.telemetry.microsoft.com",
    "df.telemetry.microsoft.com",
    "reports.wes.df.telemetry.microsoft.com",
    "services.wes.df.telemetry.microsoft.com",
    "diagnostics.support.microsoft.com",
    "feedback.search.microsoft.com",
    "feedback.windows.com",
    "feedback.microsoft-hohm.com",
    "settings-sandbox.data.microsoft.com",
    "survey.watson.microsoft.com",
    "choice.microsoft.com",
    "choice.microsoft.com.nsatc.net",
    "compatexchange.cloudapp.net",
    "corpext.msitadfs.glbdns2.microsoft.com",
    "redir.metaservices.microsoft.com",
    "statsfe1.ws.microsoft.com",
    "statsfe2.ws.microsoft.com",
    "statsfe2.update.microsoft.com.akadns.net",
    "activity.windows.com",
    "edge.activity.windows.com",
];

/// Flushes the local Windows DNS Resolver Cache.
pub fn flush_dns_cache() -> Result<String, WinApiError> {
    let mut cmd = Command::new("ipconfig");
    cmd.arg("/flushdns");
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

    match cmd.output() {
        Ok(out) => {
            if out.status.success() {
                Ok("Caché del resolver DNS purgada exitosamente.".to_string())
            } else {
                let err = String::from_utf8_lossy(&out.stderr).to_string();
                Err(WinApiError::ServiceError {
                    service: "DNS".to_string(),
                    code: out.status.code().unwrap_or(-1) as u32,
                    message: format!("Error al ejecutar ipconfig /flushdns: {}", err),
                })
            }
        }
        Err(e) => Err(WinApiError::ServiceError {
            service: "DNS".to_string(),
            code: 1,
            message: format!("Fallo al invocar ipconfig: {}", e),
        }),
    }
}

/// Checks whether the Pristine Telemetry Shield is currently active in the hosts file.
pub fn is_hosts_shield_active() -> bool {
    if let Ok(content) = fs::read_to_string(HOSTS_PATH) {
        content.contains(SHIELD_START_MARKER)
    } else {
        false
    }
}

/// Toggles the Pristine Telemetry Shield in the Windows hosts file (0.0.0.0 sinkhole).
pub fn toggle_hosts_shield(enable: bool) -> Result<bool, WinApiError> {
    let path = Path::new(HOSTS_PATH);
    let original = fs::read_to_string(path).map_err(|e| WinApiError::RegistryError {
        key: HOSTS_PATH.to_string(),
        code: 5,
        message: format!(
            "No se pudo leer el archivo hosts (requiere permisos de Administrador): {}",
            e
        ),
    })?;

    let mut new_lines = Vec::new();
    let mut in_shield_block = false;

    for line in original.lines() {
        if line.trim() == SHIELD_START_MARKER {
            in_shield_block = true;
            continue;
        }
        if line.trim() == SHIELD_END_MARKER {
            in_shield_block = false;
            continue;
        }
        if !in_shield_block {
            new_lines.push(line.to_string());
        }
    }

    if enable {
        new_lines.push("".to_string());
        new_lines.push(SHIELD_START_MARKER.to_string());
        new_lines.push(
            "# Bloqueo local de telemetría y diagnósticos de Microsoft por Pristine".to_string(),
        );
        for domain in TELEMETRY_DOMAINS {
            new_lines.push(format!("0.0.0.0 {}", domain));
            new_lines.push(format!("0.0.0.0 www.{}", domain));
        }
        new_lines.push(SHIELD_END_MARKER.to_string());
    }

    let final_content = new_lines.join("\r\n") + "\r\n";
    fs::write(path, final_content).map_err(|e| WinApiError::RegistryError {
        key: HOSTS_PATH.to_string(),
        code: 5,
        message: format!(
            "No se pudo escribir en el archivo hosts (requiere elevación UAC): {}",
            e
        ),
    })?;

    // Automatically flush DNS after modifying hosts
    let _ = flush_dns_cache();

    Ok(enable)
}

/// Restarts Windows Explorer cleanly to refresh shell, taskbar, and Start Menu policies.
pub fn restart_windows_explorer() -> Result<(), WinApiError> {
    let mut cmd = Command::new("powershell.exe");
    cmd.args([
        "-NoProfile",
        "-Command",
        "Stop-Process -Name explorer -Force -ErrorAction SilentlyContinue; Start-Sleep -Milliseconds 400; Start-Process explorer",
    ]);
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

    let _ = cmd.output();
    Ok(())
}
