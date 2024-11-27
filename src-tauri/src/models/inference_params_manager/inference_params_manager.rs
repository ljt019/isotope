use super::InferenceParams;
use tauri_plugin_store::StoreBuilder;

pub struct InferenceParamsManager {
    inference_params: InferenceParams,
    store: tauri_plugin_store::Store<tauri::Wry>,
}

impl InferenceParamsManager {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        let store = create_store(app_handle);
        let inference_params = get_params();

        Self {
            inference_params,
            store,
        }
    }

    pub fn get_inference_params(&self) -> InferenceParams {
        self.inference_params.clone()
    }

    pub fn get_current_model_value(&self) -> Option<String> {
        // Get the selected model from the store
        let selected_model = self.store.get("selected_model");

        // Deserialize the selected model
        let derserialized_value = serde_json::from_value(selected_model);

        // Create a model option
        let mut model_option = crate::models::model_options::ModelOptions::new();

        // Get the model option from the deserialized value
        let model_option = crate::models::model_options::ModelOptions::get_model_type(model_option);

        // Return the selected model or None if it doesn't exist
        match derserialized_value {
            Ok(model) => Some(model),
            Err(_) => None,
        }
    }

    pub fn get_inference_params_with_prompt(&self, prompt: String) -> InferenceParams {
        let mut inference_params = self.inference_params.clone();

        // Update the inference params with the prompt
        inference_params.prompt = prompt;

        // Return the updated inference params
        inference_params
    }

    pub fn set_inference_params(&mut self, params: InferenceParams) {
        // Update the store with the new inference params
        todo!();

        // Update the in memory inference params
        self.inference_params = params;
    }
}

fn create_store(app_handle: tauri::AppHandle) -> tauri_plugin_store::Store<tauri::Wry> {
    // Get the app data directory
    let data_dir = tauri::api::path::app_data_dir(&app_handle.config())
        .expect("Failed to get app data directory");

    println!("Data dir: {:?}", data_dir);

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&data_dir)?;

    let store_path = data_dir.join("isotope_store.bin");
    println!("Store path: {:?}", store_path);

    let mut store = StoreBuilder::new(app_handle.clone(), store_path).build();

    // Try to load existing store, create new one if failed
    if let Err(e) = store.load() {
        println!("Failed to load store: {:?}, creating new store", e);

        // Initialize with default model
        let default_model = ModelOptions::default();
        store.insert(
            "selected_model".to_string(),
            serde_json::to_value(default_model)?,
        )?;

        // Initialize with default generation params
        store.insert(
            "generation_params".to_string(),
            get_default_generation_params().clone(),
        )?;

        // Save the new store
        store.save()?;
    }

    store
}

fn get_params() -> InferenceParams {
    todo!();
}
