/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-winapi/src/error.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Error definitions for Windows API interactions.
 * ============================================================================
 */

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WinApiError {
    #[error("Registry error on key '{key}': {message} (code: {code})")]
    RegistryError {
        key: String,
        code: u32,
        message: String,
    },

    #[error("Service '{service}' error: {message} (code: {code})")]
    ServiceError {
        service: String,
        code: u32,
        message: String,
    },

    #[error("Restore point creation failed: {message} (code: {code})")]
    RestorePointError { code: u32, message: String },

    #[error("Storage error on '{path}': {message} (code: {code})")]
    StorageError {
        path: String,
        code: u32,
        message: String,
    },

    #[error("Insufficient privileges: {0}")]
    AccessDenied(String),

    #[error("Conversion or encoding error: {0}")]
    EncodingError(String),
}
