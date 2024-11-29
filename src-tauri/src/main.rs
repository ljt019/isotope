// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod models;
mod utils;

use crate::database::pool::create_pool;
use log::{debug, error};
use tauri::Manager;
use tokio::sync::Mutex;
use window_shadows::set_shadow;

#[tauri::command]
async fn chat(
    prompt: String,
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, Mutex<models::model_manager::ModelManager>>,
) -> Result<String, String> {
    debug!("Acquiring model manager lock for chat");
    let mut model_manager = state.lock().await;
    debug!("Lock acquired successfully");

    let response = model_manager
        .chat(prompt, app_handle.clone())
        .await
        .map_err(|e| {
            error!("Chat error: {}", e);
            e.to_string()
        })?;

    Ok(response)
}

#[tauri::command]
async fn set_model(
    state: tauri::State<'_, Mutex<models::model_manager::ModelManager>>,
    model_selection: String,
) -> Result<(), String> {
    debug!("Acquiring model manager lock for model selection");
    let mut model_manager = state.lock().await;
    debug!("Lock acquired successfully");

    let model =
        models::llama::llama_options::LlamaOptions::from_model_name(model_selection.as_str())
            .expect("Invalid model selection");

    model_manager.set_model(model).await;
    Ok(())
}

#[tauri::command]
fn get_model_options() -> Result<Vec<String>, String> {
    debug!("Fetching available model options");
    let llama_options = crate::models::llama::llama_options::LlamaOptions::all_model_names();
    Ok(llama_options)
}

#[tauri::command]
async fn get_selected_model(
    state: tauri::State<'_, Mutex<models::model_manager::ModelManager>>,
) -> Result<String, String> {
    debug!("Acquiring model manager lock to get selected model");
    let model_manager = state.lock().await;
    debug!("Lock acquired successfully");

    let selected_model = model_manager.get_current_model().await;
    Ok(selected_model)
}

#[tauri::command]
async fn get_current_chat(
    state: tauri::State<'_, Mutex<models::model_manager::ModelManager>>,
) -> Result<Vec<models::chat_manager::Message>, String> {
    debug!("Acquiring model manager lock to get chat history");
    let model_manager = state.lock().await;
    debug!("Lock acquired successfully");

    let current_chat = model_manager.get_current_chat().await;

    let messages = current_chat.messages.clone();

    Ok(messages)
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            debug!("Initializing application");

            // Create async runtime for setup
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

            // Initialize ModelManager state first - await the Future
            let app_data_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("Failed to get app data directory");

            let pool =
                create_pool(&app_data_dir.join("chat.db")).expect("Database initialization failed");

            // Await the ModelManager creation
            let model_manager = rt
                .block_on(async {
                    models::model_manager::ModelManager::new(app.app_handle(), pool).await
                })
                .expect("Failed to create ModelManager");
            debug!("ModelManager created");

            // Now manage the resolved ModelManager
            app.manage(Mutex::new(model_manager));
            debug!("ModelManager state managed");

            // Setup window
            let window = app
                .get_window("main")
                .ok_or_else(|| "Failed to get main window")?;
            set_shadow(&window, true).map_err(|e| format!("Failed to set window shadow: {}", e))?;

            load_main_app(&window);
            debug!("Window setup completed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            chat,
            get_model_options,
            set_model,
            get_selected_model,
            get_current_chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn load_main_app(window: &tauri::Window) {
    window
        .eval("window.location.replace('index.html');")
        .expect("Failed to load main application");
}
