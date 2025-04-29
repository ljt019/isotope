use std::fs;
use std::path::{Path, PathBuf};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn list_files(dir: String) -> Result<Vec<FileInfo>, String> {
    let path = std::path::Path::new(&dir);
    let mut files = Vec::new();

    match std::fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        let file_path = entry.path().to_string_lossy().to_string();
                        files.push(FileInfo {
                            name: file_name,
                            path: file_path,
                        });
                    }
                    Err(e) => return Err(format!("Error reading entry: {}", e)),
                }
            }
            Ok(files)
        }
        Err(e) => Err(format!("Error reading directory: {}", e)),
    }
}

const SAFE_FILE_DIR: &str =
    "C:\\Users\\lucie\\Desktop\\Projects\\personal\\isotope\\src\\files_for_ai";

fn is_within_safe_dir(target: &Path) -> Result<bool, String> {
    let safe_dir = Path::new(SAFE_FILE_DIR)
        .canonicalize()
        .map_err(|e| format!("Error canonicalizing safe directory: {}", e))?;

    // Fallback to non-canonicalized comparison if canonicalize fails (e.g., file doesn't exist)
    let target_path = target
        .canonicalize()
        .unwrap_or_else(|_| target.to_path_buf());
    Ok(target_path.starts_with(&safe_dir))
}

#[tauri::command]
fn read_file(file_path: String) -> Result<String, String> {
    let target_path = Path::new(&file_path);
    if !is_within_safe_dir(target_path)? {
        return Err("Access denied: Cannot read files outside the allowed directory".into());
    }
    fs::read_to_string(target_path).map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
fn write_file(file_path: String, file_content: String) -> Result<String, String> {
    let target_path = Path::new(&file_path);
    if !is_within_safe_dir(target_path)? {
        return Err("Access denied: Cannot write files outside the allowed directory".into());
    }
    // Ensure parent directory exists
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    fs::write(target_path, file_content).map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(format!("Successfully wrote to file: {}", file_path))
}

// Define the struct that matches the TypeScript interface
#[derive(serde::Serialize)]
struct FileInfo {
    name: String,
    path: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![list_files, write_file, read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
