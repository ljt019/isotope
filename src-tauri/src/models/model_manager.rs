use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::{EnumIter, IntoEnumIterator};
use tauri_plugin_store::StoreBuilder;

/// Represents a single message in a conversation
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Parameters for text generation
#[derive(Debug, Deserialize, Serialize)]
pub struct GenerationParams {
    pub messages: Option<Vec<Message>>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<usize>,
    pub max_tokens: Option<usize>,
    pub seed: Option<u64>,
    pub repeat_penalty: Option<f32>,
    pub repeat_last_n: Option<usize>,
}

// Define a static OnceLock for the default generation parameters
static DEFAULT_GENERATION_PARAMS: std::sync::OnceLock<serde_json::Value> =
    std::sync::OnceLock::new();

fn get_default_generation_params() -> &'static serde_json::Value {
    DEFAULT_GENERATION_PARAMS.get_or_init(|| {
        json!(
            {
                "messages": null,
                "temperature": 0.7,
                "top_p": 0.9,
                "top_k": 40,
                "max_tokens": 2048,
                "seed": null,
                "repeat_penalty": 1.1,
                "repeat_last_n": 1
            }
        )
    })
}

/* DEFAULT_GENERATION_PARAMS
    json!({
        "temperature": 0.7,
        "top_p": 0.9,
        "top_k": 40,
        "max_tokens": 2048,
        "seed": null,
        "repeat_penalty": 1.1
    })
*/

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum ModelOptions {
    V32_3BInstruct,
    V32_1BInstruct,
    SmolLM2_135MInstruct,
    SmolLM2_360MInstruct,
    SmolLM2_1BInstruct,
    TinyLlama1_1BChat,
}

impl ModelOptions {
    pub fn get_model_name(&self) -> String {
        match self {
            ModelOptions::V32_3BInstruct => "meta-llama/Llama-3.2-3B-Instruct".to_string(),
            ModelOptions::V32_1BInstruct => "meta-llama/Llama-3.2-1B-Instruct".to_string(),
            ModelOptions::SmolLM2_135MInstruct => "HuggingFaceTB/SmolLM2-135M-Instruct".to_string(),
            ModelOptions::SmolLM2_360MInstruct => "HuggingFaceTB/SmolLM2-360M-Instruct".to_string(),
            ModelOptions::SmolLM2_1BInstruct => "HuggingFaceTB/SmolLM2-1.7B-Instruct".to_string(),
            ModelOptions::TinyLlama1_1BChat => "TinyLlama/TinyLlama-1.1B-Chat-v1.0".to_string(),
        }
    }

    pub fn from_model_name(name: &str) -> Option<Self> {
        match name {
            "meta-llama/Llama-3.2-3B-Instruct" => Some(Self::V32_3BInstruct),
            "meta-llama/Llama-3.2-1B-Instruct" => Some(Self::V32_1BInstruct),
            "HuggingFaceTB/SmolLM2-135M-Instruct" => Some(Self::SmolLM2_135MInstruct),
            "HuggingFaceTB/SmolLM2-360M-Instruct" => Some(Self::SmolLM2_360MInstruct),
            "HuggingFaceTB/SmolLM2-1.7B-Instruct" => Some(Self::SmolLM2_1BInstruct),
            "TinyLlama/TinyLlama-1.1B-Chat-v1.0" => Some(Self::TinyLlama1_1BChat),
            _ => None,
        }
    }

    // Returns a Vec of all variants
    pub fn all() -> Vec<Self> {
        Self::iter().collect()
    }

    // Returns a Vec of all model names
    pub fn all_model_names() -> Vec<String> {
        Self::iter().map(|model| model.get_model_name()).collect()
    }
}

impl Default for ModelOptions {
    fn default() -> Self {
        Self::V32_1BInstruct
    }
}

pub struct ModelManager {
    store: tauri_plugin_store::Store<tauri::Wry>,
}

impl ModelManager {
    pub fn init(app_handle: tauri::AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        // Get the app data directory
        let data_dir = tauri::api::path::app_data_dir(&app_handle.config())
            .expect("Failed to get app data directory");

        println!("Data dir: {:?}", data_dir);

        // Create directory if it doesn't exist
        std::fs::create_dir_all(&data_dir)?;

        let store_path = data_dir.join("model_manager.bin");
        println!("Store path: {:?}", store_path);

        let mut store = StoreBuilder::new(app_handle, store_path).build();

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

        Ok(Self { store })
    }

    pub fn get_selected_model(&self) -> Option<ModelOptions> {
        let selected_model = self.store.get("selected_model".to_string());

        match selected_model {
            Some(value) => {
                // Attempt to deserialize the stored value into ModelOptions
                serde_json::from_value(value.clone()).ok()
            }
            None => None,
        }
    }

    pub fn set_selected_model(
        &mut self,
        model: ModelOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Serialize the enum directly
        let value = serde_json::to_value(model)?;
        self.store.insert("selected_model".to_string(), value)?;
        self.store.save()?;
        Ok(())
    }

    // Helper method to get the actual model name string
    pub fn get_selected_model_name(&self) -> Option<String> {
        self.get_selected_model()
            .map(|model| model.get_model_name())
    }

    pub fn get_generation_params(&self) -> GenerationParams {
        // Retrieve the stored Value or use the default
        let params_value: serde_json::Value = self
            .store
            .get("generation_params".to_string())
            .cloned()
            .unwrap_or_else(|| serde_json::json!(get_default_generation_params()));

        // Deserialize the Value into GenerationParams
        serde_json::from_value(params_value).expect("Failed to deserialize generation params")
    }

    pub fn set_generation_params(
        &mut self,
        params: GenerationParams,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let value = serde_json::to_value(params)?;
        self.store.insert("generation_params".to_string(), value)?;
        self.store.save()?;
        Ok(())
    }
}
