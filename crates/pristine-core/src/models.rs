/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-core/src/models.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Core domain models, risk classifications, and tweak structures.
 * ============================================================================
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TweakCategory {
    Telemetry,
    Privacy,
    SystemCleanup,
    Latency,
    SecurityEnhancement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    /// Safe: Zero secondary impact on user applications or updates.
    Safe,
    /// Moderate: Modifies optional features like cloud clipboard or feedback prompts.
    Moderate,
    /// Specific: Affects integrations like Cortana, Xbox sync or Bing Start Search.
    Specific,
    /// Critical: Aggressive tuning requiring explicit confirmation from power users.
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryRoot {
    HkeyLocalMachine,
    HkeyCurrentUser,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryValueKind {
    Dword(u32),
    Qword(u64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryMutation {
    pub root: RegistryRoot,
    pub subkey: String,
    pub value_name: String,
    pub target_value: RegistryValueKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceStartupMode {
    Automatic,
    Demand,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceMutation {
    pub service_name: String,
    pub target_startup: ServiceStartupMode,
    pub stop_if_running: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskMutation {
    pub task_path: String,
    pub task_name: String,
    pub enable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action_type", content = "payload", rename_all = "snake_case")]
pub enum TweakAction {
    Registry(RegistryMutation),
    Service(ServiceMutation),
    Task(TaskMutation),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TweakDefinition {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: TweakCategory,
    pub risk: RiskLevel,
    pub impact_details: String,
    pub actions: Vec<TweakAction>,
    pub default_recommended: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TweakState {
    Active,
    Inactive,
    Partial,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TweakAuditStatus {
    pub tweak_id: String,
    pub state: TweakState,
    pub last_checked_epoch: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemAuditReport {
    pub timestamp_utc: u64,
    pub total_analyzed: usize,
    pub active_count: usize,
    pub inactive_count: usize,
    pub privacy_score_percent: u8,
    pub statuses: Vec<TweakAuditStatus>,
}
