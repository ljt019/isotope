use crate::model;
use crate::model_manager;
use tauri::Manager;

#[tauri::command]
pub async fn chat(app_handle: tauri::AppHandle, message: String) -> Result<(), String> {
    // Clone necessary components to move into the async task
    let app_handle = app_handle.clone();

    let model_manager = model_manager::ModelManager::init(app_handle.clone())
        .map_err(|e| format!("Failed to initialize model manager: {}", e))?;

    // Get the selected model name
    let model_name = model_manager.get_selected_model_name();

    // Spawn a new asynchronous task for handling the chat
    tauri::async_runtime::spawn(async move {
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
pub async fn set_model(app_handle: tauri::AppHandle, model_name: String) -> Result<(), String> {
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
pub fn get_selected_model(app_handle: tauri::AppHandle) -> Result<String, String> {
    let model_manager = model_manager::ModelManager::init(app_handle.clone())
        .map_err(|e| format!("Failed to initialize model manager: {}", e))?;

    Ok(model_manager
        .get_selected_model_name()
        .expect("No model selected"))
}

#[tauri::command]
pub fn get_model_options() -> Result<Vec<String>, String> {
    Ok(model_manager::ModelOptions::all_model_names())
}
