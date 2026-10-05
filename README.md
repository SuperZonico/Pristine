# Pristine — Windows 11 Optimization & Privacy Suite

[![CI Status](https://img.shields.io/badge/build-passing-00f0a8?style=flat-square&logo=githubactions&logoColor=white)](.github/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2011%20(22H2%20%7C%2023H2%20%7C%2024H2)-00d2ff?style=flat-square)](SECURITY.md)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-black?style=flat-square&logo=rust)](Cargo.toml)

> Pure performance, robust telemetry suppression, and deterministic rollback for Windows 11 — engineered in Rust without breaking system stability.

---

## Overview

**Pristine** is an open-source, high-efficiency system optimization and privacy suite tailored specifically for Windows 11. Unlike traditional debloating scripts that execute destructive PowerShell routines or corrupt package stores, Pristine interfaces directly with native Win32, Registry, and Service Control Manager (SCM) APIs using enterprise-grade policy directives.

Every modification is transparent, granular, and backed by an atomic transaction journal verified with cryptographic SHA-256 checksums for 1-click reversible rollback.

---

## Architectural Highlights

- **Native Windows Policy Engine**: Directly manages Group Policy registry keys (`HKLM\SOFTWARE\Policies\Microsoft\Windows`) and SCM handles rather than spawning shell interpreters.
- **Privilege Separation Model**: The UI executes under standard user Medium Integrity, communicating via Windows Named Pipes secured with strict SDDL access control lists.
- **Deterministic 1-Click Rollback**: Binary-exact snapshots captured before every mutation allow reverting single sessions or returning to factory baseline instantly.
- **Ultra-Low Resource Footprint**: Zero garbage collection pauses, instant boot, and ~35 MB idle RAM utilization powered by Tauri v2 and native WebView2.
- **Precision Iconography & Visuals**: 100% handcrafted mathematical vector SVG assets. Seamless automatic synchronization with Windows 11 Light/Dark theme schedules.
- **Smart Non-Destructive Cleaning**: Temporary file cleaner strictly filters files older than 24 hours without active file descriptor locks, plus WinSxS component store pruning via DISM.
- **Large & Stale Files Analyzer**: Identifies heavy, cold files (>100MB, 1GB+) untouched for months across user libraries and temp storage, with safe Windows Recycle Bin (`SHFileOperationW` with `FOF_ALLOWUNDO`) integration.
- **Clean Software & Bloatware Uninstaller**: Discovers Win32 programs and UWP apps, uninstalls without background bloat, and performs deep orphan residual scanning across AppData, ProgramData, and Registry with strict token matching.
- **DNS Telemetry Sinkhole Shield**: One-click local redirection of 40+ Microsoft diagnostic telemetry endpoints to `0.0.0.0` inside `C:\Windows\System32\drivers\etc\hosts` with automatic DNS resolver flush.

---

## Core Capabilities & Pillars

### 1. Privacy & Telemetry Suppression
- **Deterministic Group Policy Directives**: Enforces `HKLM\SOFTWARE\Policies\Microsoft\Windows` policies across DiagTrack (Universal Telemetry Client), CEIP, Windows Error Reporting (WER), Advertising ID, and Windows Copilot/Recall.
- **DNS Sinkhole Shield**: Instantaneous redirection of over 40 known diagnostic and telemetry hostnames to `0.0.0.0` within `C:\Windows\System32\drivers\etc\hosts`, combined with automatic DNS resolver cache flushing (`DnsFlushResolverCache`).

### 2. Deterministic Rollback Journal
- **Cryptographic Checksums**: Every applied tweak is preceded by a state snapshot saved into a transaction journal, signed with SHA-256 integrity hashes.
- **1-Click Atomic Restoration**: Easily revert any session or restore system state to the exact baseline prior to modification.
- **VSS Integration**: Create Volume Shadow Copy (VSS) restore points natively via Windows API before running major optimizations.

### 3. Clean Software & Bloatware Uninstaller
- **Win32 & Modern UWP Support**: Intelligently discovers desktop software and Windows 11 AppX/MSIX packages.
- **Zero-Residual Scanning**: Deep scanning across `AppData\Local`, `AppData\Roaming`, `ProgramData`, and registry paths (`HKCU`, `HKLM`) with strict whole-word token matching and TLD stripping to eliminate false positives.

### 4. Smart Non-Destructive Cleaning
- **Safe Temporary Files**: Purgers files strictly older than 24 hours without active file locks, protecting running processes.
- **WinSxS Component Store Pruning**: Automated DISM component store cleanup to reclaim space occupied by superseded Windows updates.
- **Large & Stale Files Analyzer**: Scans user directories for heavy, cold files (>100MB, >1GB) unused for months, offering safe movement to the Windows Recycle Bin (`SHFileOperationW` with `FOF_ALLOWUNDO`).

---

## Workspace Structure

```
Pristine/
├── .github/workflows/          # Automated GitHub Actions CI/CD pipelines
├── crates/
│   ├── pristine-core/          # Domain models, tweak catalog, and SHA-256 transactions
│   ├── pristine-winapi/        # Safe Win32 RAII wrappers (Registry, SCM, Restore, Shell)
│   ├── pristine-engine/        # System auditor, transaction planner, and rollback
│   ├── pristine-metrics/       # Real-time PDH / Win32 CPU & RAM hardware metrics
│   └── pristine-ipc/           # Typed inter-process protocol and SDDL access control
├── src-tauri/                  # Tauri v2 desktop shell and native command bindings
├── ui/                         # Handcrafted Mica/Acrylic interface (HTML, CSS, JS)
├── LICENSE                     # MIT License
└── SECURITY.md                 # Vulnerability disclosure policy
```

---

## Building from Source

### Prerequisites

1. **Windows 11** (x86_64 or aarch64)
2. **Rust Toolchain**: `rustup default stable` (1.80+)
3. **Microsoft C++ Build Tools** (via Visual Studio Installer or MSVC v143+)

### Build & Test Suite

```powershell
# Clone the repository
git clone https://github.com/SuperZonico/Pristine.git
cd Pristine

# Execute all workspace unit and integration tests
cargo test --workspace

# Run strict Clippy audit (zero warnings enforced)
cargo clippy --workspace -- -D warnings

# Verify formatting
cargo fmt --all -- --check

# Compile production-hardened binary
cargo build --release --workspace
```

---

## Author & License

* **Architect & Author**: **SuperZonico**
* **License**: Released under the [MIT License](LICENSE).
