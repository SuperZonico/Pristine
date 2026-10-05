/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-core/src/catalog.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Curated catalog of Windows 11 policies, telemetry, and latency tweaks.
 * ============================================================================
 */

use crate::models::{
    RegistryMutation, RegistryRoot, RegistryValueKind, RiskLevel, ServiceMutation,
    ServiceStartupMode, TaskMutation, TweakAction, TweakCategory, TweakDefinition,
};

pub fn get_default_catalog() -> Vec<TweakDefinition> {
    vec![
        // 1. Diagnostic Telemetry (DiagTrack)
        TweakDefinition {
            id: "diagtrack_utc".to_string(),
            title: "Connected User Experiences & Telemetry (DiagTrack)".to_string(),
            description: "Stops and sets DiagTrack service to disabled, enforcing zero telemetry upload.".to_string(),
            category: TweakCategory::Telemetry,
            risk: RiskLevel::Safe,
            impact_details: "Halts periodic diagnostic background transmission. Windows Update and Store remain unaffected.".to_string(),
            actions: vec![
                TweakAction::Service(ServiceMutation {
                    service_name: "DiagTrack".to_string(),
                    target_startup: ServiceStartupMode::Disabled,
                    stop_if_running: true,
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection".to_string(),
                    value_name: "AllowTelemetry".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
            ],
            default_recommended: true,
        },

        // 2. CEIP Consolidator and App Experience Tasks
        TweakDefinition {
            id: "ceip_tasks".to_string(),
            title: "Customer Experience Improvement Program Tasks".to_string(),
            description: "Disables background scheduled tasks that gather telemetry and app usage profiles.".to_string(),
            category: TweakCategory::Telemetry,
            risk: RiskLevel::Safe,
            impact_details: "Reduces idle disk I/O and processor wakeups.".to_string(),
            actions: vec![
                TweakAction::Task(TaskMutation {
                    task_path: r"\Microsoft\Windows\Customer Experience Improvement Program".to_string(),
                    task_name: "Consolidator".to_string(),
                    enable: false,
                }),
                TweakAction::Task(TaskMutation {
                    task_path: r"\Microsoft\Windows\Customer Experience Improvement Program".to_string(),
                    task_name: "UsbCeip".to_string(),
                    enable: false,
                }),
                TweakAction::Task(TaskMutation {
                    task_path: r"\Microsoft\Windows\Application Experience".to_string(),
                    task_name: "Microsoft Compatibility Appraiser".to_string(),
                    enable: false,
                }),
            ],
            default_recommended: true,
        },

        // 3. User Advertising Identifier
        TweakDefinition {
            id: "advertising_id".to_string(),
            title: "Windows Advertising ID Tracking".to_string(),
            description: "Prevents applications from tracking user behavior across software via unique Advertising ID.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            impact_details: "Eliminates personalized targeted ads in Store apps without affecting app functionality.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo".to_string(),
                    value_name: "Enabled".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
            ],
            default_recommended: true,
        },

        // 4. Start Menu Bing Web Search
        TweakDefinition {
            id: "bing_start_search".to_string(),
            title: "Start Menu Bing Web Search & Suggestions".to_string(),
            description: "Stops Start Menu keystrokes from querying Bing web servers over HTTP.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            impact_details: "Start Menu search becomes instantaneous and strictly limited to local files and installed applications.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Search".to_string(),
                    value_name: "BingSearchEnabled".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer".to_string(),
                    value_name: "DisableSearchBoxSuggestions".to_string(),
                    target_value: RegistryValueKind::Dword(1),
                }),
            ],
            default_recommended: true,
        },

        // 5. Windows 11 AI Recall & Screen Analysis (24H2)
        TweakDefinition {
            id: "windows_recall_ai".to_string(),
            title: "Windows Recall & AI Continuous Snapshots".to_string(),
            description: "Enforces policies to disable continuous desktop snapshot capture and local OCR analysis.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            impact_details: "Mitigates screen activity logging and local snapshot database retention.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI".to_string(),
                    value_name: "DisableAIDataAnalysis".to_string(),
                    target_value: RegistryValueKind::Dword(1),
                }),
            ],
            default_recommended: true,
        },

        // 6. Windows Copilot Integration
        TweakDefinition {
            id: "copilot_ai".to_string(),
            title: "Windows Copilot Shell Integration".to_string(),
            description: "Disables the integrated Copilot side panel and cloud telemetry prompts.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Moderate,
            impact_details: "Removes Copilot dock/shortcut. Can be re-enabled at any time.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Policies\Microsoft\Windows\WindowsCopilot".to_string(),
                    value_name: "TurnOffWindowsCopilot".to_string(),
                    target_value: RegistryValueKind::Dword(1),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot".to_string(),
                    value_name: "TurnOffWindowsCopilot".to_string(),
                    target_value: RegistryValueKind::Dword(1),
                }),
            ],
            default_recommended: false,
        },

        // 7. Inking & Typing Diagnostic Data (Keylogger Telemetry)
        TweakDefinition {
            id: "inking_typing_telemetry".to_string(),
            title: "Inking & Typing Personalization Transmission".to_string(),
            description: "Prevents transmission of handwriting, typing samples, and dictionary usage to Microsoft.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            impact_details: "Keyboard input remains 100% functional locally; dictionary sync to cloud is disabled.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Microsoft\InputPersonalization".to_string(),
                    value_name: "RestrictImplicitInkCollection".to_string(),
                    target_value: RegistryValueKind::Dword(1),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Microsoft\InputPersonalization".to_string(),
                    value_name: "RestrictImplicitTextCollection".to_string(),
                    target_value: RegistryValueKind::Dword(1),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Microsoft\Personalization\Settings".to_string(),
                    value_name: "AcceptedPrivacyPolicy".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
            ],
            default_recommended: true,
        },

        // 8. Delivery Optimization P2P Internet Upload
        TweakDefinition {
            id: "delivery_optimization_p2p".to_string(),
            title: "Delivery Optimization Internet P2P Upload".to_string(),
            description: "Limits update delivery optimization strictly to the local LAN, blocking peer uploads to the public Internet.".to_string(),
            category: TweakCategory::SystemCleanup,
            risk: RiskLevel::Safe,
            impact_details: "Saves outbound internet bandwidth without disrupting official Windows Update downloads.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization".to_string(),
                    value_name: "DODownloadMode".to_string(),
                    target_value: RegistryValueKind::Dword(1), // 1 = LAN only, 0 = HTTP only
                }),
            ],
            default_recommended: true,
        },

        // 9. Multimedia Network Throttling
        TweakDefinition {
            id: "multimedia_network_throttling".to_string(),
            title: "Multimedia Network Throttling Mitigation".to_string(),
            description: "Disables non-multimedia network throttling when audio or video drivers are active.".to_string(),
            category: TweakCategory::Latency,
            risk: RiskLevel::Safe,
            impact_details: "Prevents packet batching and reduces jitter in competitive online games and streaming.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile".to_string(),
                    value_name: "NetworkThrottlingIndex".to_string(),
                    target_value: RegistryValueKind::Dword(0xFFFFFFFF),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile".to_string(),
                    value_name: "SystemResponsiveness".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
            ],
            default_recommended: true,
        },

        // 10. Hybrid Error Reporting (Preserve Minidumps, Block Auto-Upload)
        TweakDefinition {
            id: "error_reporting_hybrid".to_string(),
            title: "Hybrid Error Reporting (Local Only)".to_string(),
            description: "Maintains local crash dump storage for developer debugging while disabling automated transmission to remote servers.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            impact_details: "BSOD minidumps remain readable by WinDbg; no payload is transmitted outwards.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting".to_string(),
                    value_name: "Disabled".to_string(),
                    target_value: RegistryValueKind::Dword(1),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting".to_string(),
                    value_name: "DoNotSendAdditionalData".to_string(),
                    target_value: RegistryValueKind::Dword(1),
                }),
            ],
            default_recommended: true,
        },
    ]
}
