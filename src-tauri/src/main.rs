// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod model;

use tauri::async_runtime::spawn;
use tauri::Manager;

#[tauri::command]
async fn chat(app_handle: tauri::AppHandle, message: String) -> Result<(), String> {
    // Clone necessary components to move into the async task
    let app_handle = app_handle.clone();

    // Spawn a new asynchronous task for handling the chat
    spawn(async move {
        // Initialize the model
        let model = match model::Model::new(None).await {
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

use window_shadows::set_shadow;

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![chat])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(window, true).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
