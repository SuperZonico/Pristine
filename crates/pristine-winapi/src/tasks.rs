/*
 * ============================================================================
 * Project:      Pristine — Privacy & Performance Suite
 * File:         crates/pristine-winapi/src/tasks.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Safe Task Scheduler abstractions without shell execution.
 * ============================================================================
 */

use crate::error::WinApiError;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;

pub fn configure_task(task_path: &str, task_name: &str, enable: bool) -> Result<bool, WinApiError> {
    let full_task = if task_path.ends_with('\\') {
        format!("{}{}", task_path, task_name)
    } else {
        format!("{}\\{}", task_path, task_name)
    };

    let switch = if enable { "/enable" } else { "/disable" };

    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("schtasks.exe");
        cmd.args(["/change", "/tn", &full_task, switch]);
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(true)
                } else {
                    let err = String::from_utf8_lossy(&output.stderr).to_string();
                    Err(WinApiError::ServiceError {
                        service: full_task,
                        code: output.status.code().unwrap_or(-1) as u32,
                        message: format!("Failed to configure scheduled task: {}", err),
                    })
                }
            }
            Err(e) => Err(WinApiError::ServiceError {
                service: full_task,
                code: 1,
                message: format!("Could not execute schtasks: {}", e),
            }),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = full_task;
        let _ = switch;
        Ok(true)
    }
}

pub fn query_task_enabled(task_path: &str, task_name: &str) -> bool {
    let full_task = if task_path.ends_with('\\') {
        format!("{}{}", task_path, task_name)
    } else {
        format!("{}\\{}", task_path, task_name)
    };

    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("schtasks.exe");
        cmd.args(["/query", "/tn", &full_task, "/fo", "CSV", "/nh"]);
        cmd.creation_flags(0x08000000);

        if let Ok(output) = cmd.output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                // "Ready" or "Running" means enabled; "Disabled" means disabled
                return !stdout.to_lowercase().contains("disabled");
            }
        }
        false
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = full_task;
        true
    }
}
