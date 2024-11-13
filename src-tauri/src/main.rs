// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod model;
mod model_manager;
mod utils;

use tauri::async_runtime::spawn;
use tauri::Manager;

#[tauri::command]
async fn chat(app_handle: tauri::AppHandle, message: String) -> Result<(), String> {
    // Clone necessary components to move into the async task
    let app_handle = app_handle.clone();

    let model_manager = model_manager::ModelManager::init(app_handle.clone())
        .map_err(|e| format!("Failed to initialize model manager: {}", e))?;

    // Get the selected model name
    let model_name = model_manager.get_selected_model_name();

    // Spawn a new asynchronous task for handling the chat
    spawn(async move {
        // Initialize the model
        let model = match model::Model::new(model_name.as_deref()).await {
            Ok(m) => m,
            Err(e) => {
                let _ = app_handle.emit_all("chat-error", format!("Failed to load model: {}", e));
                return;
            }
        };

        // Generate tokens and emit events
        if let Err(e) = model.chat_stream(&message, &app_handle).await {
            let _ = app_handle.emit_all("chat-error", format!("Generation error: {}", e));
        }
    });

    Ok(())
}

#[tauri::command]
async fn set_model(app_handle: tauri::AppHandle, model_name: String) -> Result<(), String> {
    let mut model_manager = model_manager::ModelManager::init(app_handle.clone())
        .map_err(|e| format!("Failed to initialize model manager: {}", e))?;

    model_manager
        .set_selected_model(
            model_manager::ModelOptions::from_model_name(&model_name).expect("Invalid model name"),
        )
        .map_err(|e| format!("Failed to set model: {}", e))?;

    Ok(())
}

#[tauri::command]
fn get_selected_model(app_handle: tauri::AppHandle) -> Result<String, String> {
    let model_manager = model_manager::ModelManager::init(app_handle.clone())
        .map_err(|e| format!("Failed to initialize model manager: {}", e))?;

    Ok(model_manager
        .get_selected_model_name()
        .expect("No model selected"))
}

#[tauri::command]
fn get_model_options() -> Result<Vec<String>, String> {
    Ok(model_manager::ModelOptions::all_model_names())
}

use window_shadows::set_shadow;

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            chat,
            set_model,
            get_model_options,
            get_selected_model
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(window, true).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
