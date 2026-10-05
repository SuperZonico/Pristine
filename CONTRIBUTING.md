# Contributing to Pristine

Thank you for your interest in contributing to **Pristine**! Pristine is an open-source, high-efficiency privacy and performance optimization suite specifically engineered for Windows 11.

Our core commitment is **"Never Break Windows"**: every optimization must be deterministic, non-destructive, completely reversible, and verified with cryptographic integrity.

---

## Code of Conduct

All contributors and maintainers are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md). Please treat everyone with respect, professionalism, and kindness.

---

## Development Environment Setup

### Prerequisites
- **Operating System:** Windows 11 (22H2, 23H2, or 24H2)
- **Rust Toolchain:** Stable (1.80+) via `rustup default stable`
- **Node.js:** v20+ LTS
- **Build Tools:** Visual Studio 2022 C++ Build Tools (MSVC v143+)

### Cloning & Building

```powershell
# Clone the repository
git clone https://github.com/SuperZonico/Pristine.git
cd Pristine

# Install UI dev server dependencies
npm install

# Run unit tests across all workspace crates
cargo test --workspace

# Start local hot-reloading development environment
npm run dev
```

---

## Architectural Principles & Coding Standards

### 1. Zero Emojis in Code & UI
- Pristine adheres strictly to an enterprise-grade, clean aesthetic.
- The UI uses **100% mathematical vector SVG assets** with consistent stroke styling (`stroke-width="1.75"`, rounded caps). No emojis are permitted anywhere in application code or UI templates.

### 2. Mandatory File Header
All new source files must begin with the standard Pristine attribution header:

```rust
/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/...
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Brief description of this module's responsibility.
 * ============================================================================
 */
```

### 3. Safety & Reversibility
- Never delete system-critical components, AppX packages without dependency checks, or registry keys that cause `sfc /scannow` corruption.
- All tweaks must provide both an `apply` routine and a binary-exact `revert` routine registered in the transaction rollback engine.
- For disk cleanup, never delete active files or files modified within the last 24 hours. For user files, prefer moving to the Windows Recycle Bin (`SHFileOperationW` + `FOF_ALLOWUNDO`).

### 4. Verification Checklist Before Submitting a PR
Before submitting a pull request, ensure the following commands complete with zero errors and zero warnings:

```powershell
# 1. Check code formatting
cargo fmt --all -- --check

# 2. Run strict Clippy linter
cargo clippy --workspace --all-targets -- -D warnings

# 3. Execute all unit tests
cargo test --workspace

# 4. Validate release build
cargo build --release --workspace
```

---

## Pull Request Guidelines

1. **Fork & Branch:** Create a feature branch from `master` (e.g., `feat/my-feature` or `fix/issue-description`).
2. **Atomic Commits:** Write clear, conventional commit messages (`feat: ...`, `fix: ...`, `docs: ...`).
3. **Link Issues:** Reference any related issue in your pull request description (e.g., `Closes #12`).
4. **Testing:** Include unit tests for any new Win32 abstractions or parsing routines.

---

## Author & Maintainer

* **Lead Architect:** **SuperZonico**
* **License:** Released under the [MIT License](LICENSE).
