pub mod chat_manager;
pub mod inference_params_manager;
pub mod llama;
pub mod model_manager;

pub trait LLM {
    fn inference(&self, params: InferenceParams) -> String;
    fn format_prompt(&self, prompt: Vec<Message>) -> String;
}
