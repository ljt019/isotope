pub mod inference_params_manager;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InferenceParams {
    model: String,
    pub prompt: String,
    temperature: f64,
    top_p: f64,
    top_k: usize,
    max_tokens: usize,
    seed: u64,
    repeat_penalty: f32,
    repeat_last_n: usize,
}

impl Default for InferenceParams {
    fn default() -> Self {
        InferenceParams {
            model: "V32_3BInstruct".to_string(),
            prompt: "".to_string(),
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            max_tokens: 2048,
            seed: 0,
            repeat_penalty: 1.1,
            repeat_last_n: 1,
        }
    }
}
