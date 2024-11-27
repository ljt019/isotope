use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

pub enum ModelOptions {
    ModelOption,
}

impl ModelOptions {
    pub fn get_model_type(model_id: String) -> ModelOptions {
        // Get the model option from the model id
        let model_option: Option<ModelOptions> = ModelOption::from_model_name(model_id.as_str());

        // Return the model option
        match model_option {
            Some(model) => ModelOptions::ModelOption(model),
            None => ModelOptions::ModelOption(ModelOption::V32_3BInstruct),
        }
    }
}

trait ModelOption {
    fn get_model_name(&self) -> String;
    fn from_model_name(model_name: &str) -> Option<Self>;
    fn all_model_names() -> Vec<Sring>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
enum LlamaOptions {
    V32_3BInstruct,
    V32_1BInstruct,
    SmolLM2_135MInstruct,
    SmolLM2_360MInstruct,
    SmolLM2_1BInstruct,
    TinyLlama1_1BChat,
}

impl ModelOption for LlamaOptions {
    fn get_model_name(&self) -> String {
        match self {
            ModelOptions::V32_3BInstruct => "meta-llama/Llama-3.2-3B-Instruct".to_string(),
            ModelOptions::V32_1BInstruct => "meta-llama/Llama-3.2-1B-Instruct".to_string(),
            ModelOptions::SmolLM2_135MInstruct => "HuggingFaceTB/SmolLM2-135M-Instruct".to_string(),
            ModelOptions::SmolLM2_360MInstruct => "HuggingFaceTB/SmolLM2-360M-Instruct".to_string(),
            ModelOptions::SmolLM2_1BInstruct => "HuggingFaceTB/SmolLM2-1.7B-Instruct".to_string(),
            ModelOptions::TinyLlama1_1BChat => "TinyLlama/TinyLlama-1.1B-Chat-v1.0".to_string(),
        }
    }

    fn from_model_name(name: &str) -> Option<Self> {
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

    // Returns a Vec of all model names
    fn all_model_names() -> Vec<String> {
        Self::iter().map(|model| model.get_model_name()).collect()
    }
}
