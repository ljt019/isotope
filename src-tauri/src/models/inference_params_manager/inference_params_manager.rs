use super::InferenceParams;
use tauri_plugin_store::JsonValue;
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
        // Get and deserialize the selected model into a String
        let params = self.get_inference_params();

        // Get model option from inference params
        let selected_model = params.model.clone();

        println!("Selected model: {:?}", selected_model);

        // Get Model Option from string
        let model_option =
            crate::models::llama::llama_options::LlamaOptions::from_model_option_string(
                selected_model.as_str(),
            );

        let model_value = model_option
            .expect("Failed to get current model value")
            .get_model_name();

        Some(model_value)
    }

    pub fn get_inference_params_with_prompt(&self, prompt: String) -> InferenceParams {
        let mut inference_params = self.inference_params.clone();

        // Update the inference params with the prompt
        inference_params.prompt = prompt;

        // Return the updated inference params
        inference_params
    }

    pub fn set_model(&mut self, model: crate::models::llama::llama_options::LlamaOptions) {
        // Update the store with the new model
        self.store
            .insert(
                "selected_model".to_string(),
                serde_json::to_value(model.get_model_name()).expect("Failed to serialize model"),
            )
            .expect("Failed to insert new model into store");
    }

    pub fn set_inference_params(&mut self, params: InferenceParams) {
        // Update the store with the new inference params
        self.store
            .insert(
                "generation_params".to_string(),
                serde_json::to_value(&params).unwrap(),
            )
            .expect("Failed to insert new inference params into store");

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
    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    let store_path = data_dir.join("isotope_store.bin");
    println!("Store path: {:?}", store_path);

    let mut store = StoreBuilder::new(app_handle.clone(), store_path).build();

    // Try to load existing store, create new one if failed
    if let Err(e) = store.load() {
        println!("Failed to load store: {:?}, creating new store", e);

        // Initialize with default model
        let default_model = crate::models::llama::llama_options::LlamaOptions::default_model();
        store
            .insert(
                "selected_model".to_string(),
                serde_json::to_value(default_model).expect("Failed to serialize default model"),
            )
            .expect("Failed to insert default model into store");

        // Initialize with default generation params
        store
            .insert(
                "generation_params".to_string(),
                get_default_generation_params(),
            )
            .expect("Failed to insert default generation params into store");

        // Save the new store
        store.save().expect("Failed to save new store");
    }

    store
}

fn get_params() -> InferenceParams {
    let params = InferenceParams::default();

    params
}

fn get_default_generation_params() -> JsonValue {
    let params = InferenceParams::default();

    serde_json::to_value(params).expect("Failed to serialize default generation params")
}
