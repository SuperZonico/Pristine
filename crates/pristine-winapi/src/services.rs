/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-winapi/src/services.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Safe native Service Control Manager (SCM) abstractions.
 * ============================================================================
 */

use pristine_core::models::ServiceStartupMode;
use windows::core::PCWSTR;
use windows::Win32::Foundation::ERROR_SERVICE_DOES_NOT_EXIST;
use windows::Win32::System::Services::{
    ChangeServiceConfigW, CloseServiceHandle, ControlService, OpenSCManagerW, OpenServiceW,
    QueryServiceConfigW, QueryServiceStatusEx, ENUM_SERVICE_TYPE, QUERY_SERVICE_CONFIGW, SC_HANDLE,
    SC_MANAGER_ALL_ACCESS, SC_MANAGER_CONNECT, SC_STATUS_PROCESS_INFO, SERVICE_AUTO_START,
    SERVICE_CHANGE_CONFIG, SERVICE_CONTROL_STOP, SERVICE_DEMAND_START, SERVICE_DISABLED,
    SERVICE_ERROR, SERVICE_NO_CHANGE, SERVICE_QUERY_CONFIG, SERVICE_QUERY_STATUS, SERVICE_RUNNING,
    SERVICE_START_TYPE, SERVICE_STATUS, SERVICE_STATUS_PROCESS, SERVICE_STOP,
};

use crate::error::WinApiError;

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub struct SafeScHandle {
    handle: SC_HANDLE,
}

impl SafeScHandle {
    pub fn new(handle: SC_HANDLE) -> Self {
        Self { handle }
    }

    pub fn raw(&self) -> SC_HANDLE {
        self.handle
    }
}

impl Drop for SafeScHandle {
    fn drop(&mut self) {
        if !self.handle.is_invalid() {
            unsafe {
                let _ = CloseServiceHandle(self.handle);
            }
        }
    }
}

pub struct ServiceStatusInfo {
    pub is_running: bool,
    pub startup_mode: ServiceStartupMode,
}

pub fn query_service(service_name: &str) -> Result<Option<ServiceStatusInfo>, WinApiError> {
    let scm = unsafe { OpenSCManagerW(None, None, SC_MANAGER_CONNECT) }.map_err(|e| {
        WinApiError::ServiceError {
            service: service_name.to_string(),
            code: e.code().0 as u32,
            message: format!("Failed to connect to Service Control Manager: {}", e),
        }
    })?;
    let _scm_guard = SafeScHandle::new(scm);

    let service_name_w = to_wide(service_name);
    let service_res = unsafe {
        OpenServiceW(
            scm,
            PCWSTR(service_name_w.as_ptr()),
            SERVICE_QUERY_STATUS | SERVICE_QUERY_CONFIG,
        )
    };

    let service = match service_res {
        Ok(s) => s,
        Err(e) if e.code() == ERROR_SERVICE_DOES_NOT_EXIST.to_hresult() => return Ok(None),
        Err(e) => {
            return Err(WinApiError::ServiceError {
                service: service_name.to_string(),
                code: e.code().0 as u32,
                message: format!("Failed to open service for status query: {}", e),
            })
        }
    };
    let _svc_guard = SafeScHandle::new(service);

    // Query running status
    let mut status = SERVICE_STATUS_PROCESS::default();
    let mut bytes_needed = 0u32;
    let status_slice = unsafe {
        std::slice::from_raw_parts_mut(
            &mut status as *mut _ as *mut u8,
            std::mem::size_of::<SERVICE_STATUS_PROCESS>(),
        )
    };

    let ok = unsafe {
        QueryServiceStatusEx(
            service,
            SC_STATUS_PROCESS_INFO,
            Some(status_slice),
            &mut bytes_needed,
        )
    };

    let is_running = ok.is_ok() && status.dwCurrentState == SERVICE_RUNNING;

    // Query config for startup type
    let mut config_bytes = 0u32;
    unsafe {
        let _ = QueryServiceConfigW(service, None, 0, &mut config_bytes);
    };

    let mut config_buf = vec![0u8; config_bytes as usize];
    let config_ptr = config_buf.as_mut_ptr() as *mut QUERY_SERVICE_CONFIGW;

    let config_ok =
        unsafe { QueryServiceConfigW(service, Some(config_ptr), config_bytes, &mut config_bytes) };

    let startup_mode = if config_ok.is_ok() {
        let start_type = unsafe { (*config_ptr).dwStartType };
        match start_type {
            SERVICE_AUTO_START => ServiceStartupMode::Automatic,
            SERVICE_DEMAND_START => ServiceStartupMode::Demand,
            SERVICE_DISABLED => ServiceStartupMode::Disabled,
            _ => ServiceStartupMode::Demand,
        }
    } else {
        ServiceStartupMode::Demand
    };

    Ok(Some(ServiceStatusInfo {
        is_running,
        startup_mode,
    }))
}

pub fn configure_service(
    service_name: &str,
    target_startup: ServiceStartupMode,
    stop_if_running: bool,
) -> Result<Option<ServiceStatusInfo>, WinApiError> {
    let prior_status = query_service(service_name)?;
    if prior_status.is_none() {
        return Ok(None);
    }

    let scm = unsafe { OpenSCManagerW(None, None, SC_MANAGER_ALL_ACCESS) }.map_err(|e| {
        WinApiError::ServiceError {
            service: service_name.to_string(),
            code: e.code().0 as u32,
            message: format!("Elevated SCM access required to configure services: {}", e),
        }
    })?;
    let _scm_guard = SafeScHandle::new(scm);

    let service_name_w = to_wide(service_name);
    let service = unsafe {
        OpenServiceW(
            scm,
            PCWSTR(service_name_w.as_ptr()),
            SERVICE_CHANGE_CONFIG | SERVICE_STOP | SERVICE_QUERY_STATUS,
        )
    }
    .map_err(|e| WinApiError::ServiceError {
        service: service_name.to_string(),
        code: e.code().0 as u32,
        message: format!("Failed to open service for configuration: {}", e),
    })?;
    let _svc_guard = SafeScHandle::new(service);

    let new_start_type: SERVICE_START_TYPE = match target_startup {
        ServiceStartupMode::Automatic => SERVICE_AUTO_START,
        ServiceStartupMode::Demand => SERVICE_DEMAND_START,
        ServiceStartupMode::Disabled => SERVICE_DISABLED,
    };

    let change_ok = unsafe {
        ChangeServiceConfigW(
            service,
            ENUM_SERVICE_TYPE(SERVICE_NO_CHANGE),
            new_start_type,
            SERVICE_ERROR(SERVICE_NO_CHANGE),
            PCWSTR::null(),
            PCWSTR::null(),
            None,
            PCWSTR::null(),
            PCWSTR::null(),
            PCWSTR::null(),
            PCWSTR::null(),
        )
    };

    if let Err(e) = change_ok {
        return Err(WinApiError::ServiceError {
            service: service_name.to_string(),
            code: e.code().0 as u32,
            message: format!("Failed to change service startup type: {}", e),
        });
    }

    if stop_if_running {
        let mut status = SERVICE_STATUS::default();
        unsafe {
            let _ = ControlService(service, SERVICE_CONTROL_STOP, &mut status);
        }
    }

    Ok(prior_status)
}
