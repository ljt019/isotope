// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod database;
mod models;
mod utils;

use commands::*;
use tauri::Manager;

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
            set_shadow(&window, true).unwrap();

            let app_handle = app.app_handle();

            // Perform initialization asynchronously
            tauri::async_runtime::spawn(async move {
                setup(app_handle.config().as_ref());

                load_main_app(&window);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn load_main_app(window: &tauri::Window) {
    window
        .eval("window.location.replace('index.html');")
        .unwrap();
}

fn setup(config: &tauri::Config) {
    let connection = database::setup_database(&config);

    let random_number = rand::random::<u32>();

    crate::database::insert_chat(&connection, format!("Chat {}", random_number))
        .expect("Failed to insert chat");
}
