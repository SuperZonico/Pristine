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
- **Smart Non-Destructive Cleaning**: Temporary file cleaner strictly filters files older than 24 hours without active file descriptor locks.

---

## Master Documentation Suite

Comprehensive architecture, research, and threat modeling documents are located in the [`docs/`](docs/) directory:

| Document | Description |
| :--- | :--- |
| **[01. Vision & Philosophy](docs/01_VISION_Y_FILOSOFIA.md)** | "Never Break Windows" principles, consent-first architecture, and risk classification. |
| **[02. Technical Architecture](docs/02_ARQUITECTURA_TECNICA.md)** | Dual-process privilege model, Cargo workspace crates, and memory safety invariants. |
| **[03. Telemetry & Services Catalog](docs/03_CATALOGO_TELEMETRIA_Y_SERVICIOS.md)** | Detailed mapping of DiagTrack, Recall, Copilot, CEIP, WER, and Edge telemetry. |
| **[04. Cleanup & Optimization Engine](docs/04_OPTIMIZACIONES_Y_LIMPIEZA.md)** | Safe temp cleaner, WinSxS DISM pruning, and multimedia network latency mitigations. |
| **[05. Security, Rollback & Resilience](docs/05_SEGURIDAD_ROLLBACK_Y_RESILIENCIA.md)** | Transactional diff journal, VSS restore points, and emergency recovery script. |
| **[06. Design System & UX](docs/06_SISTEMA_DE_DISENO_Y_UX.md)** | Windows 11 Fluent/Mica visual specs, dark/light dynamic tokens, and UI layout. |
| **[07. Phase Plan & Testing Roadmap](docs/07_PLAN_DE_FASE_Y_ROADMAP.md)** | Sprints, verification matrix across Windows 11 22H2/23H2/24H2 builds. |
| **[08. Code Standards & GitHub Prep](docs/08_ESTANDARES_CODIGO_Y_GITHUB.md)** | File headers, author attribution, and vector iconography standards. |

---

## Workspace Structure

```
Pristine/
├── .github/workflows/          # Automated GitHub Actions CI pipeline
├── crates/
│   ├── pristine-core/          # Domain models, tweak catalog, and SHA-256 transactions
│   ├── pristine-winapi/        # Safe Win32 RAII wrappers (Registry, SCM, Restore)
│   ├── pristine-engine/        # System auditor, transaction planner, and rollback
│   ├── pristine-metrics/       # Real-time PDH / Win32 CPU & RAM hardware metrics
│   └── pristine-ipc/           # Typed inter-process protocol and SDDL access control
├── src-tauri/                  # Tauri v2 desktop shell and native command bindings
├── ui/                         # Handcrafted Mica/Acrylic interface (HTML, CSS, JS)
├── docs/                       # Technical architecture and research book
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
