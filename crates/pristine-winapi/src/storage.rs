/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-winapi/src/storage.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Disk space analyzer for large files and cold/stale data cleanup.
 * ============================================================================
 */

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use windows::core::PCWSTR;
use windows::Win32::UI::Shell::{
    SHFileOperationW, FOF_ALLOWUNDO, FOF_NOCONFIRMATION, FOF_SILENT, FO_DELETE, SHFILEOPSTRUCTW,
};

use crate::error::WinApiError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeStaleFile {
    pub path: String,
    pub file_name: String,
    pub extension: String,
    pub size_bytes: u64,
    pub size_formatted: String,
    pub last_accessed_days_ago: u64,
    pub last_modified_days_ago: u64,
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    }
}

fn get_days_ago(time: SystemTime) -> u64 {
    match SystemTime::now().duration_since(time) {
        Ok(dur) => dur.as_secs() / 86400,
        Err(_) => 0,
    }
}

const SKIP_DIR_NAMES: &[&str] = &[
    "windows",
    "system32",
    "syswow64",
    "$recycle.bin",
    "system volume information",
    ".git",
    "node_modules",
    "target",
    "winsxs",
    "servicing",
];

fn should_skip_directory(dir_name: &str) -> bool {
    let lower = dir_name.to_lowercase();
    SKIP_DIR_NAMES.iter().any(|&s| lower == s)
}

fn scan_dir_recursive(
    dir: &Path,
    min_size_bytes: u64,
    min_days_stale: u64,
    current_depth: usize,
    max_depth: usize,
    results: &mut Vec<LargeStaleFile>,
) {
    if current_depth > max_depth || results.len() >= 500 {
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };

        if file_type.is_symlink() {
            continue;
        }

        if file_type.is_dir() {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if !should_skip_directory(name) {
                scan_dir_recursive(
                    &path,
                    min_size_bytes,
                    min_days_stale,
                    current_depth + 1,
                    max_depth,
                    results,
                );
            }
        } else if file_type.is_file() {
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let size = metadata.len();
            if size < min_size_bytes {
                continue;
            }

            let accessed = metadata
                .accessed()
                .unwrap_or_else(|_| metadata.modified().unwrap_or_else(|_| SystemTime::now()));
            let modified = metadata.modified().unwrap_or(accessed);

            let accessed_days = get_days_ago(accessed);
            let modified_days = get_days_ago(modified);

            // Cold file condition: hasn't been accessed or modified within the staleness threshold
            if accessed_days >= min_days_stale || modified_days >= min_days_stale {
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Archivo")
                    .to_string();

                let extension = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_uppercase();

                results.push(LargeStaleFile {
                    path: path.to_string_lossy().to_string(),
                    file_name,
                    extension,
                    size_bytes: size,
                    size_formatted: format_size(size),
                    last_accessed_days_ago: accessed_days,
                    last_modified_days_ago: modified_days,
                });
            }
        }
    }
}

/// Escanea ubicaciones clave o una ruta personalizada en busca de archivos de gran tamaño sin uso reciente.
pub fn scan_large_stale_files(
    min_size_mb: u64,
    min_days_stale: u64,
    custom_path: Option<String>,
) -> Vec<LargeStaleFile> {
    let min_size_bytes = min_size_mb * 1024 * 1024;
    let mut results = Vec::new();

    let scan_roots: Vec<PathBuf> = if let Some(custom) = custom_path {
        let p = PathBuf::from(custom);
        if p.is_dir() {
            vec![p]
        } else {
            Vec::new()
        }
    } else {
        let mut roots = Vec::new();
        if let Ok(user_profile) = std::env::var("USERPROFILE") {
            let user_path = PathBuf::from(user_profile);
            roots.push(user_path.join("Downloads"));
            roots.push(user_path.join("Documents"));
            roots.push(user_path.join("Videos"));
            roots.push(user_path.join("Desktop"));
        }
        if let Ok(temp_dir) = std::env::var("TEMP") {
            roots.push(PathBuf::from(temp_dir));
        }
        roots
    };

    for root in scan_roots {
        if root.is_dir() {
            scan_dir_recursive(&root, min_size_bytes, min_days_stale, 0, 8, &mut results);
        }
    }

    // Ordenar de mayor a menor tamaño
    results.sort_by_key(|b| std::cmp::Reverse(b.size_bytes));

    results
}

/// Envía un archivo a la Papelera de Reciclaje de Windows o lo elimina definitivamente.
pub fn delete_file_safely(path: &str, to_recycle_bin: bool) -> Result<(), WinApiError> {
    let p = Path::new(path);
    if !p.exists() || !p.is_file() {
        return Err(WinApiError::StorageError {
            path: path.to_string(),
            code: 2,
            message: "El archivo especificado no existe o es un directorio.".to_string(),
        });
    }

    if to_recycle_bin {
        // Enviar a la Papelera mediante la API nativa de Shell (SHFileOperationW)
        // La API requiere una cadena terminada en doble carácter nulo
        let mut wide_path: Vec<u16> = path
            .encode_utf16()
            .chain(std::iter::once(0))
            .chain(std::iter::once(0))
            .collect();

        let mut file_op = SHFILEOPSTRUCTW {
            hwnd: windows::Win32::Foundation::HWND::default(),
            wFunc: FO_DELETE,
            pFrom: PCWSTR(wide_path.as_mut_ptr()),
            pTo: PCWSTR::null(),
            fFlags: (FOF_ALLOWUNDO | FOF_NOCONFIRMATION | FOF_SILENT).0 as u16,
            fAnyOperationsAborted: windows::Win32::Foundation::BOOL(0),
            hNameMappings: std::ptr::null_mut(),
            lpszProgressTitle: PCWSTR::null(),
        };

        let res = unsafe { SHFileOperationW(&mut file_op) };
        if res == 0 {
            Ok(())
        } else {
            Err(WinApiError::StorageError {
                path: path.to_string(),
                code: res as u32,
                message: format!(
                    "Error al mover el archivo a la Papelera de Reciclaje (código: {}).",
                    res
                ),
            })
        }
    } else {
        // Eliminación permanente
        fs::remove_file(p).map_err(|e| WinApiError::StorageError {
            path: path.to_string(),
            code: 1,
            message: format!("Error al eliminar archivo permanentemente: {}", e),
        })
    }
}

/// Abre el Explorador de Windows resaltando el archivo especificado.
pub fn reveal_in_explorer(path: &str) -> Result<(), WinApiError> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(WinApiError::StorageError {
            path: path.to_string(),
            code: 2,
            message: "El archivo no existe en el disco.".to_string(),
        });
    }

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        Command::new("explorer.exe")
            .raw_arg(format!("/select,\"{}\"", path))
            .spawn()
            .map_err(|e| WinApiError::StorageError {
                path: path.to_string(),
                code: 1,
                message: format!("Error al invocar explorer.exe: {}", e),
            })?;
    }
    #[cfg(not(target_os = "windows"))]
    {
        Command::new("explorer")
            .arg(path)
            .spawn()
            .map_err(|e| WinApiError::StorageError {
                path: path.to_string(),
                code: 1,
                message: format!("Error al abrir archivo: {}", e),
            })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500 * 1024), "500.0 KB");
        assert_eq!(format_size(150 * 1024 * 1024), "150.0 MB");
        assert_eq!(format_size(2 * 1024 * 1024 * 1024), "2.00 GB");
    }

    #[test]
    fn test_should_skip_directory() {
        assert!(should_skip_directory("Windows"));
        assert!(should_skip_directory("System32"));
        assert!(should_skip_directory("node_modules"));
        assert!(should_skip_directory("target"));
        assert!(!should_skip_directory("MyOldVideos"));
    }
}
