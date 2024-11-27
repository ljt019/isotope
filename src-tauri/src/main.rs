// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod models;
mod types;
mod utils;

use tauri::Manager;

use window_shadows::set_shadow;

#[tauri::command]
fn chat(prompt: String, state: tauri::State<models::ModelManager>) -> String {
    let model_manager = state.inner();

    let response = model_manager.chat(prompt);

    response
}

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![chat])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).unwrap();

            let app_handle = app.app_handle();

            // Perform initialization asynchronously
            tauri::async_runtime::spawn(async move {
                let database_conn = setup(app_handle.config().as_ref());

                // Create a model manager with the app handle and database connection
                let model_manager = models::ModelManager::new(app_handle, database_conn);

                // Store the model manager in the managed state
                app.manage(model_manager);

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

fn setup(config: &tauri::Config) -> rusqlite::Connection {
    let connection = database::setup_database(&config);

    let random_number = rand::random::<u32>();

    crate::database::insert_chat(&connection, format!("Chat {}", random_number))
        .expect("Failed to insert chat");

    connection
}
