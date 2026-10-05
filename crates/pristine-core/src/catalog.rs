/*
 * ============================================================================
 * Project:      Pristine — Privacy & Performance Suite
 * File:         crates/pristine-core/src/catalog.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Comprehensive catalog of Windows policies, telemetry, and latency tweaks.
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

        // 5. Windows AI Recall & Screen Analysis
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
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Policies\Microsoft\Windows\WindowsAI".to_string(),
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

        // 11. Microsoft Edge Telemetry & Background Mode
        TweakDefinition {
            id: "edge_telemetry".to_string(),
            title: "Microsoft Edge Telemetry & Background Acceleration".to_string(),
            description: "Disables telemetry reporting and prevents Edge from running silent background processes on startup.".to_string(),
            category: TweakCategory::Telemetry,
            risk: RiskLevel::Safe,
            impact_details: "Frees up RAM on boot and stops Edge diagnostic beacon transmissions.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Edge".to_string(),
                    value_name: "MetricsReportingEnabled".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Edge".to_string(),
                    value_name: "StartupBoostEnabled".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Edge".to_string(),
                    value_name: "BackgroundModeEnabled".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
            ],
            default_recommended: true,
        },

        // 12. Cortana Voice Telemetry & Search Sync
        TweakDefinition {
            id: "cortana_voice_telemetry".to_string(),
            title: "Cortana & Speech Telemetry".to_string(),
            description: "Disables Cortana digital assistant integration and background voice model upload.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            impact_details: "Stops background speech telemetry packets from sending audio data.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search".to_string(),
                    value_name: "AllowCortana".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Microsoft\Speech_OneCore\Preferences".to_string(),
                    value_name: "ModelDownloadAllowed".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
            ],
            default_recommended: true,
        },

        // 13. Activity History & Timeline Sync
        TweakDefinition {
            id: "activity_history_sync".to_string(),
            title: "Windows Activity History Tracking & Sync".to_string(),
            description: "Stops Windows from recording chronological application usage history and syncing it to Microsoft accounts.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            impact_details: "Keeps your application launching habits completely private and off cloud servers.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\System".to_string(),
                    value_name: "EnableActivityFeed".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\System".to_string(),
                    value_name: "PublishUserActivities".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\System".to_string(),
                    value_name: "UploadUserActivities".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
            ],
            default_recommended: true,
        },

        // 14. Geolocation Tracking Service
        TweakDefinition {
            id: "location_sensor".to_string(),
            title: "Background Geolocation Tracking".to_string(),
            description: "Disables the background location sensor service and blocks system-wide geofencing queries.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            impact_details: "Prevents background services from polling your physical coordinates.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors".to_string(),
                    value_name: "DisableLocation".to_string(),
                    target_value: RegistryValueKind::Dword(1),
                }),
                TweakAction::Service(ServiceMutation {
                    service_name: "lfsvc".to_string(),
                    target_startup: ServiceStartupMode::Disabled,
                    stop_if_running: true,
                }),
            ],
            default_recommended: true,
        },

        // 15. Tailored Experiences & Diagnostic Feedback Prompts
        TweakDefinition {
            id: "tailored_experiences".to_string(),
            title: "Tailored Experiences & Feedback Prompts".to_string(),
            description: "Stops diagnostic data from being used to serve suggestions, tips, and promotional recommendations.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            impact_details: "Removes intrusive 'How likely are you to recommend Windows?' popups and suggested Store apps.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Policies\Microsoft\Windows\CloudContent".to_string(),
                    value_name: "DisableTailoredExperiencesWithDiagnosticData".to_string(),
                    target_value: RegistryValueKind::Dword(1),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Microsoft\Siuf\Rules".to_string(),
                    value_name: "NumberOfSIUFInPeriod".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
            ],
            default_recommended: true,
        },

        // 16. GameDVR Background Capture (DPC Latency Optimizer)
        TweakDefinition {
            id: "game_dvr_latency".to_string(),
            title: "GameDVR Background Capture & Latency Tuning".to_string(),
            description: "Disables background video recording hooks that cause frame pacing drops and DPC latency spikes.".to_string(),
            category: TweakCategory::Latency,
            risk: RiskLevel::Safe,
            impact_details: "Smooths micro-stutter in competitive games without disabling Xbox Live authentication.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"System\GameConfigStore".to_string(),
                    value_name: "GameDVR_Enabled".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\GameDVR".to_string(),
                    value_name: "AllowGameDVR".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
            ],
            default_recommended: true,
        },

        // 17. Cloud Clipboard Synchronization
        TweakDefinition {
            id: "cloud_clipboard_sync".to_string(),
            title: "Cloud Clipboard Synchronization (Local Only)".to_string(),
            description: "Restricts clipboard history strictly to your local machine, blocking sync to remote Microsoft servers.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            impact_details: "Keeps sensitive copied passwords and tokens from ever leaving your device memory.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\System".to_string(),
                    value_name: "AllowCrossDeviceClipboard".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Microsoft\Clipboard".to_string(),
                    value_name: "EnableClipboardHistory".to_string(),
                    target_value: RegistryValueKind::Dword(1), // keep local history enabled
                }),
            ],
            default_recommended: true,
        },

        // 18. Cross-Application Diagnostics
        TweakDefinition {
            id: "app_diagnostics".to_string(),
            title: "Cross-App Diagnostic Access".to_string(),
            description: "Prevents third-party Store apps from interrogating diagnostic info of other concurrently running applications.".to_string(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            impact_details: "Enhances inter-process privacy and isolation.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyCurrentUser,
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics".to_string(),
                    value_name: "Value".to_string(),
                    target_value: RegistryValueKind::String("Deny".to_string()),
                }),
            ],
            default_recommended: true,
        },

        // 19. Wi-Fi Sense Automatic Shared Network Connection
        TweakDefinition {
            id: "wifi_sense".to_string(),
            title: "Wi-Fi Sense Credential Sharing".to_string(),
            description: "Disables automatic connection to open hotspots and contacts-shared Wi-Fi networks.".to_string(),
            category: TweakCategory::SecurityEnhancement,
            risk: RiskLevel::Safe,
            impact_details: "Prevents accidental connection to untrusted open wireless access points.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SOFTWARE\Microsoft\WcmSvc\wifinetworkmanager\config".to_string(),
                    value_name: "AutoConnectAllowedOEM".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
            ],
            default_recommended: true,
        },

        // 20. Remote Assistance Solicited Requests
        TweakDefinition {
            id: "remote_assistance_solicited".to_string(),
            title: "Unsolicited Remote Assistance Offers".to_string(),
            description: "Blocks external systems from offering remote assistance sessions to this computer.".to_string(),
            category: TweakCategory::SecurityEnhancement,
            risk: RiskLevel::Safe,
            impact_details: "Mitigates unauthorized remote takeover attempts while standard RDP remains under user control.".to_string(),
            actions: vec![
                TweakAction::Registry(RegistryMutation {
                    root: RegistryRoot::HkeyLocalMachine,
                    subkey: r"SYSTEM\CurrentControlSet\Control\Remote Assistance".to_string(),
                    value_name: "fAllowToGetHelp".to_string(),
                    target_value: RegistryValueKind::Dword(0),
                }),
            ],
            default_recommended: true,
        },
    ]
}
