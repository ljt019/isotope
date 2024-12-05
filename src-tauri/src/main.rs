// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod models;
mod utils;

use crate::database::pool::create_pool;
use chrono::Local;
use colored::*;
use log::Level;
use log::{debug, error, info};
use tauri::Manager;
use tauri_plugin_log::LogTarget;
use tokio::sync::Mutex;
use window_shadows::set_shadow;

#[tauri::command]
async fn chat(
    prompt: String,
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, Mutex<models::model_manager::ModelManager>>,
) -> Result<String, String> {
    let mut model_manager = state.lock().await;

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
    let mut model_manager = state.lock().await;

    let model =
        models::llama::llama_options::LlamaOptions::from_model_name(model_selection.as_str())
            .expect("Invalid model selection");

    debug!("Setting model to: {}", model.get_model_name());

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
    let model_manager = state.lock().await;

    let selected_model = model_manager.get_current_model().await;
    Ok(selected_model)
}

#[tauri::command]
async fn get_current_chat(
    state: tauri::State<'_, Mutex<models::model_manager::ModelManager>>,
) -> Result<Vec<models::chat_manager::Message>, String> {
    let model_manager = state.lock().await;

    let current_chat = model_manager.get_current_chat().await;

    let messages = current_chat.messages.clone();

    Ok(messages)
}

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout])
                .level(log::LevelFilter::Debug)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .format(|callback, args, record| {
                    let timestamp = Local::now()
                        .format("%Y-%m-%d %H:%M:%S%.3f")
                        .to_string()
                        .dimmed();
                    let level = colorize_level(record.level());
                    let target = record.target().cyan();
                    let message = args.to_string();

                    let formatted_message =
                        format!("{} {} [{}] {}", timestamp, level, target, message);

                    callback.finish(format_args!("{}", formatted_message))
                })
                .filter(blacklist_filter)
                .build(),
        )
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

            // Now manage the resolved ModelManager
            app.manage(Mutex::new(model_manager));

            // Setup window
            let window = app
                .get_window("main")
                .ok_or_else(|| "Failed to get main window")?;
            set_shadow(&window, true).map_err(|e| format!("Failed to set window shadow: {}", e))?;

            load_main_app(&window);
            info!("Application initialized");
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

// Define a blacklist filter function
fn blacklist_filter(metadata: &log::Metadata) -> bool {
    let blacklist_modules = ["hf_hub"];

    // Exclude messages from blacklisted modules
    if blacklist_modules
        .iter()
        .any(|module| metadata.target().starts_with(module))
    {
        false
    } else {
        true
    }
}

/// Maps log levels to their corresponding colored strings.
fn colorize_level(level: Level) -> colored::ColoredString {
    match level {
        Level::Error => "ERROR".red().bold(),
        Level::Warn => "WARN".yellow().bold(),
        Level::Info => "INFO".green().bold(),
        Level::Debug => "DEBUG".magenta().bold(),
        Level::Trace => "TRACE".blue().bold(),
    }
}
