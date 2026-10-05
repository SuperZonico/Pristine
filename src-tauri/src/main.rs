/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         src-tauri/src/main.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Native Windows executable entry point (hides console in release).
 * ============================================================================
 */

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    pristine_app_lib::run();
}
