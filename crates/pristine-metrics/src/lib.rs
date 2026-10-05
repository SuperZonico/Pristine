/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-metrics/src/lib.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Real-time low-overhead system hardware metrics via Win32 APIs.
 * ============================================================================
 */

use serde::{Deserialize, Serialize};
use windows::Win32::Foundation::FILETIME;
use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use windows::Win32::System::Threading::GetSystemTimes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareMetrics {
    pub ram_total_mb: u64,
    pub ram_used_mb: u64,
    pub ram_percent: u8,
    pub cpu_percent: u8,
    pub timestamp_utc: u64,
}

pub struct MetricsCollector {
    last_idle_time: u64,
    last_kernel_time: u64,
    last_user_time: u64,
}

fn filetime_to_u64(ft: &FILETIME) -> u64 {
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
}

impl MetricsCollector {
    pub fn new() -> Self {
        let mut collector = Self {
            last_idle_time: 0,
            last_kernel_time: 0,
            last_user_time: 0,
        };
        collector.sample_cpu();
        collector
    }

    fn sample_cpu(&mut self) -> u8 {
        let mut idle_time = FILETIME::default();
        let mut kernel_time = FILETIME::default();
        let mut user_time = FILETIME::default();

        let ok = unsafe {
            GetSystemTimes(
                Some(&mut idle_time),
                Some(&mut kernel_time),
                Some(&mut user_time),
            )
        };

        if ok.is_err() {
            return 0;
        }

        let idle = filetime_to_u64(&idle_time);
        let kernel = filetime_to_u64(&kernel_time);
        let user = filetime_to_u64(&user_time);

        if self.last_kernel_time == 0 {
            self.last_idle_time = idle;
            self.last_kernel_time = kernel;
            self.last_user_time = user;
            return 0;
        }

        let idle_diff = idle.saturating_sub(self.last_idle_time);
        let kernel_diff = kernel.saturating_sub(self.last_kernel_time);
        let user_diff = user.saturating_sub(self.last_user_time);

        self.last_idle_time = idle;
        self.last_kernel_time = kernel;
        self.last_user_time = user;

        let total = kernel_diff.saturating_add(user_diff);
        if total == 0 {
            return 0;
        }

        let busy = total.saturating_sub(idle_diff);
        ((busy as f64 / total as f64) * 100.0).clamp(0.0, 100.0) as u8
    }

    pub fn collect(&mut self) -> HardwareMetrics {
        let cpu_percent = self.sample_cpu();

        let mut mem_status = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };

        let (ram_total_mb, ram_used_mb, ram_percent) = unsafe {
            if GlobalMemoryStatusEx(&mut mem_status).is_ok() {
                let total = mem_status.ullTotalPhys / (1024 * 1024);
                let avail = mem_status.ullAvailPhys / (1024 * 1024);
                let used = total.saturating_sub(avail);
                (total, used, mem_status.dwMemoryLoad as u8)
            } else {
                (0, 0, 0)
            }
        };

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        HardwareMetrics {
            ram_total_mb,
            ram_used_mb,
            ram_percent,
            cpu_percent,
            timestamp_utc: now,
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collection() {
        let mut collector = MetricsCollector::new();
        let metrics = collector.collect();
        assert!(
            metrics.ram_total_mb > 0,
            "RAM total must be greater than zero"
        );
        assert!(metrics.ram_percent <= 100, "RAM percent must be valid");
    }
}
