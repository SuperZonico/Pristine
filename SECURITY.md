/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         SECURITY.md
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Security and vulnerability disclosure policy.
 * ============================================================================
 */

# Security Policy

## Supported Versions

Pristine actively receives security patches and compatibility updates for the following Windows 11 builds:

| Pristine Version | Windows 11 Edition / Build | Supported |
| :--- | :--- | :--- |
| **1.0.x (Latest)** | Windows 11 24H2 (Build 26100+) | Yes |
| **1.0.x (Latest)** | Windows 11 23H2 (Build 22631) | Yes |
| **1.0.x (Latest)** | Windows 11 22H2 (Build 22621) | Yes |
| **1.0.x (Latest)** | Windows 11 21H2 (Build 22000) | Security Only |
| `< 1.0.0` | Any | No |

---

## Architectural Security Commitments

1. **Privilege Separation**:
   - The user interface executes exclusively under **Medium Integrity** (standard user token).
   - High-privilege registry and service manipulations are strictly isolated to an elevated engine process communicating via secured Windows Named Pipes protected by SDDL access control lists.
2. **Deterministic Rollback**:
   - Every registry mutation maintains a binary-exact before/after state entry in an atomic session journal verified with cryptographic SHA-256 integrity checksums.
3. **Zero Dynamic Remote Execution**:
   - Pristine operates 100% offline. No code is pulled or executed remotely from the internet.

---

## Reporting a Vulnerability

If you discover any potential security vulnerability, privilege escalation vector, or stability regression in Pristine:

1. **Do not open a public GitHub issue immediately.**
2. Send a detailed report directly to the security contact for **SuperZonico** via GitHub Security Advisories or private channel.
3. Include:
   - Specific component or crate affected (`pristine-winapi`, `pristine-ipc`, etc.).
   - Windows 11 exact build number (`winver`).
   - Proof of concept (PoC) or reproduction steps.

Security issues are triaged within 48 hours and prioritized for prompt remediation.
