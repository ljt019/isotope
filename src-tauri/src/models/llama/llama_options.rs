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

    pub fn from_model_option_string(name: &str) -> Option<Self> {
        match name {
            "V32_3BInstruct" => Some(Self::V32_3BInstruct),
            "V32_1BInstruct" => Some(Self::V32_1BInstruct),
            "SmolLM2_135MInstruct" => Some(Self::SmolLM2_135MInstruct),
            "SmolLM2_360MInstruct" => Some(Self::SmolLM2_360MInstruct),
            "SmolLM2_1BInstruct" => Some(Self::SmolLM2_1BInstruct),
            "TinyLlama1_1BChat" => Some(Self::TinyLlama1_1BChat),
            _ => None,
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

    // Returns a Vec of all model names
    pub fn all_model_names() -> Vec<String> {
        Self::iter().map(|model| model.get_model_name()).collect()
    }

    pub fn default_model() -> Self {
        Self::V32_3BInstruct
    }
}
