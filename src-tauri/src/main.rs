// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod model;
mod model_manager;
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

            // Perform initialization asynchronously
            tauri::async_runtime::spawn(async move {
                setup();

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

fn setup() {
    println!("Initializing...");
    std::thread::sleep(std::time::Duration::from_secs(4)); // Simulate a delay
    println!("Done initializing.");
}
