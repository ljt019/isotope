pub mod inference_params_manager;
use serde::{Deserialize, Serialize};

pub use inference_params_manager::InferenceParamsManager;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InferenceParams {
    pub model: String,
    pub prompt: String,
    pub temperature: f64,
    pub top_p: Option<f64>,
    pub top_k: Option<usize>,
    pub max_tokens: usize,
    pub seed: u64,
    pub repeat_penalty: f32,
    pub repeat_last_n: usize,
}

impl Default for InferenceParams {
    fn default() -> Self {
        InferenceParams {
            model: "V32_3BInstruct".to_string(),
            prompt: "".to_string(),
            temperature: 0.2,
            top_p: Some(0.9),
            top_k: Some(50),
            max_tokens: 2048,
            seed: 42,
            repeat_penalty: 1.1,
            repeat_last_n: 128,
        }
    }
}
