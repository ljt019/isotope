use super::InferenceParams;
use log::debug;
use serde_json::Value as JsonValue;
use tauri::Wry;
use tauri_plugin_store::{Store, StoreBuilder};

pub struct InferenceParamsManager {
    inference_params: InferenceParams,
    store: Store<Wry>,
}

impl InferenceParamsManager {
    pub fn new(app_handle: tauri::AppHandle<Wry>) -> Self {
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
        let params = self.get_inference_params();
        let selected_model = params.model;
        debug!("Selected model: {:?}", selected_model);

        let model_option = crate::models::llama::llama_options::LlamaOptions::from_model_name(
            selected_model.as_str(),
        );

        model_option.map(|m| m.get_model_name())
    }

    pub fn get_inference_params_with_prompt(&self, prompt: String) -> InferenceParams {
        let mut inference_params = self.inference_params.clone();
        inference_params.prompt = prompt;
        inference_params
    }

    /// Set the model to use for inference
    pub fn set_model(&mut self, model: crate::models::llama::llama_options::LlamaOptions) {
        let model_name = model.get_model_name();
        self.store
            .insert(
                "selected_model".to_string(),
                serde_json::to_value(&model_name).expect("Failed to serialize model"),
            )
            .expect("Failed to insert new model into store");

        self.inference_params.model = model_name;
    }

    pub fn set_inference_params(&mut self, params: InferenceParams) {
        self.store
            .insert(
                "generation_params".to_string(),
                serde_json::to_value(&params).expect("Failed to serialize params"),
            )
            .expect("Failed to insert params into store");

        self.inference_params = params;
    }
}

fn create_store(app_handle: tauri::AppHandle<Wry>) -> Store<Wry> {
    let data_dir = tauri::api::path::app_data_dir(&app_handle.config())
        .expect("Failed to get app data directory");
    debug!("Data dir: {:?}", data_dir);

    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");
    let store_path = data_dir.join("isotope_store.bin");
    debug!("Store path: {:?}", store_path);

    let mut store = StoreBuilder::new(app_handle, store_path).build();

    if let Err(e) = store.load() {
        debug!("Failed to load store: {:?}, creating new store", e);
        initialize_store(&mut store).expect("Failed to initialize store");
    }

    store
}

fn initialize_store(store: &mut Store<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    let default_model = "meta-llama/Llama-3.2-1B-Instruct";

    store.insert(
        "selected_model".to_string(),
        serde_json::to_value(default_model)?,
    )?;

    store.insert(
        "generation_params".to_string(),
        get_default_generation_params(),
    )?;

    store.save()?;
    Ok(())
}

fn get_params() -> InferenceParams {
    InferenceParams::default()
}

fn get_default_generation_params() -> JsonValue {
    serde_json::to_value(InferenceParams::default()).expect("Failed to serialize default params")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[derive(Default)]
    struct MockStore {
        data: Mutex<HashMap<String, JsonValue>>,
    }

    impl MockStore {
        fn insert(&self, key: String, value: JsonValue) {
            self.data.lock().unwrap().insert(key, value);
        }

        fn get(&self, key: &str) -> Option<JsonValue> {
            self.data.lock().unwrap().get(key).cloned()
        }
    }

    struct TestInferenceParamsManager {
        inference_params: InferenceParams,
        store: MockStore,
    }

    impl TestInferenceParamsManager {
        fn new() -> Self {
            Self {
                inference_params: get_params(),
                store: MockStore::default(),
            }
        }

        fn get_current_model_value(&self) -> Option<String> {
            Some(self.inference_params.model.clone())
        }

        fn set_model(&mut self, model: crate::models::llama::llama_options::LlamaOptions) {
            let model_name = model.get_model_name();
            self.store.insert(
                "selected_model".to_string(),
                serde_json::to_value(model_name.clone()).unwrap(),
            );
            self.inference_params.model = model_name;
        }
    }

    #[test]
    fn test_set_model() {
        let mut manager = TestInferenceParamsManager::new();

        let initial_model = manager.get_current_model_value();
        assert_eq!(initial_model, Some("V32_3BInstruct".to_string()));

        let new_model = crate::models::llama::llama_options::LlamaOptions::from_model_name(
            "HuggingFaceTB/SmolLM2-135M-Instruct",
        )
        .expect("Failed to create model");
        manager.set_model(new_model);

        let updated_model = manager.get_current_model_value();
        assert_eq!(
            updated_model,
            Some("HuggingFaceTB/SmolLM2-135M-Instruct".to_string())
        );
    }
}
