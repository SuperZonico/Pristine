/*
 * ============================================================================
 * Project:      Pristine — Privacy & Performance Suite
 * File:         crates/pristine-engine/src/journal.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Persistent transaction journal storage with atomic disk writes.
 * ============================================================================
 */

use pristine_core::transaction::TransactionSession;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JournalError {
    #[error("I/O error during journal operation: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Session '{0}' not found in journal")]
    SessionNotFound(String),

    #[error("Integrity check failed: session tampering detected in '{0}'")]
    IntegrityFailure(String),
}

fn get_journal_path() -> PathBuf {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let dir = PathBuf::from(local_app_data).join("Pristine");
        let _ = create_dir_all(&dir);
        dir.join("journal.json")
    } else {
        PathBuf::from("journal.json")
    }
}

pub fn load_journal() -> Vec<TransactionSession> {
    let path = get_journal_path();
    if !path.exists() {
        return Vec::new();
    }

    if let Ok(mut file) = File::open(&path) {
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() {
            if let Ok(sessions) = serde_json::from_str::<Vec<TransactionSession>>(&content) {
                // Return sessions whose cryptographic integrity matches
                return sessions
                    .into_iter()
                    .filter(|s| s.verify_integrity())
                    .collect();
            }
        }
    }

    Vec::new()
}

pub fn save_session(session: &TransactionSession) -> Result<(), JournalError> {
    let path = get_journal_path();
    let mut sessions = load_journal();

    // Remove if already exists with same id (replace)
    sessions.retain(|s| s.session_id != session.session_id);
    sessions.insert(0, session.clone());

    let json = serde_json::to_string_pretty(&sessions)?;

    // Atomic write via temporary file
    let tmp_path = path.with_extension("tmp");
    {
        let mut tmp_file = File::create(&tmp_path)?;
        tmp_file.write_all(json.as_bytes())?;
        tmp_file.sync_all()?;
    }

    std::fs::rename(tmp_path, path)?;
    Ok(())
}

pub fn remove_session(session_id: &str) -> Result<(), JournalError> {
    let path = get_journal_path();
    let mut sessions = load_journal();

    let initial_len = sessions.len();
    sessions.retain(|s| s.session_id != session_id);

    if sessions.len() == initial_len {
        return Err(JournalError::SessionNotFound(session_id.to_string()));
    }

    let json = serde_json::to_string_pretty(&sessions)?;
    let tmp_path = path.with_extension("tmp");
    {
        let mut tmp_file = File::create(&tmp_path)?;
        tmp_file.write_all(json.as_bytes())?;
        tmp_file.sync_all()?;
    }

    std::fs::rename(tmp_path, path)?;
    Ok(())
}
