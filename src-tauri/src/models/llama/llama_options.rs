use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum LlamaOptions {
    V32_3BInstruct,
    V32_1BInstruct,
    SmolLM2_135MInstruct,
    SmolLM2_360MInstruct,
    SmolLM2_1BInstruct,
    TinyLlama1_1BChat,
}

impl LlamaOptions {
    pub fn get_model_name(&self) -> String {
        match self {
            LlamaOptions::V32_3BInstruct => "meta-llama/Llama-3.2-3B-Instruct".to_string(),
            LlamaOptions::V32_1BInstruct => "meta-llama/Llama-3.2-1B-Instruct".to_string(),
            LlamaOptions::SmolLM2_135MInstruct => "HuggingFaceTB/SmolLM2-135M-Instruct".to_string(),
            LlamaOptions::SmolLM2_360MInstruct => "HuggingFaceTB/SmolLM2-360M-Instruct".to_string(),
            LlamaOptions::SmolLM2_1BInstruct => "HuggingFaceTB/SmolLM2-1.7B-Instruct".to_string(),
            LlamaOptions::TinyLlama1_1BChat => "TinyLlama/TinyLlama-1.1B-Chat-v1.0".to_string(),
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

    pub fn from_pretty_name(name: &str) -> Self {
        match name {
            "Llama-3.2-3B" => Self::V32_3BInstruct,
            "Llama-3.2-1B" => Self::V32_1BInstruct,
            "SmolLM2-135M" => Self::SmolLM2_135MInstruct,
            "SmolLM2-360M" => Self::SmolLM2_360MInstruct,
            "SmolLM2-1.7B" => Self::SmolLM2_1BInstruct,
            "TinyLlama-1.1B" => Self::TinyLlama1_1BChat,
            _ => panic!("Model name not found"),
        }
    }

    pub fn from_model_name_to_pretty_name(name: &str) -> String {
        match name {
            "meta-llama/Llama-3.2-3B-Instruct" => "Llama-3.2-3B".to_string(),
            "meta-llama/Llama-3.2-1B-Instruct" => "Llama-3.2-1B".to_string(),
            "HuggingFaceTB/SmolLM2-135M-Instruct" => "SmolLM2-135M".to_string(),
            "HuggingFaceTB/SmolLM2-360M-Instruct" => "SmolLM2-360M".to_string(),
            "HuggingFaceTB/SmolLM2-1.7B-Instruct" => "SmolLM2-1.7B".to_string(),
            "TinyLlama/TinyLlama-1.1B-Chat-v1.0" => "TinyLlama-1.1B".to_string(),
            _ => panic!("Model name not found"),
        }
    }

    // Returns a Vec of all model names
    pub fn all_model_names() -> Vec<String> {
        Self::iter().map(|model| model.get_model_name()).collect()
    }
}
