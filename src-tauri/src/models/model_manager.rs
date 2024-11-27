use crate::models::chat_manager::ChatManager;
use crate::models::inference_params_manager::InferenceParamsManager;
use crate::types::LLM;

/// Manages the active model, chat history, and inference parameters
pub struct ModelManager {
    model: Box<dyn LLM>,
    inference_params_manager: InferenceParamsManager,
    chat_manager: ChatManager,
}

impl ModelManager {
    /// Create a new ModelManager with the given model and database connection
    pub fn new(app_handle: tauri::AppHandle, db_conn: rusqlite::Connection) -> Self {
        // Create a new inference params manager and chat manager
        let inference_params_manager = InferenceParamsManager::new(app_handle);
        let chat_manager = ChatManager::new(db_conn);

        // Create a new model based on the selected model or the default model
        match inference_params_manager.get_current_model() {
            Some(model_name) => {
                // Load the model with the name in the inference params
                model = super::llama_models::Model::new(model_name);
            }
            None => {
                // Load the default model if no model is returned
                model = super::llama_models::Model::new(None);
            }
        }

        // Return the new ModelManager
        Self {
            model: Box::new(model),
            chat_manager: chat_manager,
            inference_params_manager,
        }
    }

    /// Chat with the model, with the given prompt
    pub fn chat(&mut self, prompt: String) {
        // Create a message from the user input
        let prompt = crate::types::Message {
            role: "user".to_string(),
            content: prompt,
        };

        // Handle database stuff and get full prompt, including chat history
        let full_chat_prompt = self.chat_manager.handle_prompt(prompt);

        // Format the prompt for the model
        let formatted_prompt = self.model.format_prompt(full_chat_prompt);

        // Get the inference params from the store
        let inference_params = self
            .inference_params_manager
            .get_inference_params_with_prompt(formatted_prompt);

        // Feed the params to the model and get the response
        let response = crate::types::Message {
            role: "assistant".to_string(),
            content: self.model.inference(inference_params),
        };

        // Add the response to the chat history
        self.chat_manager.handle_response(response);
    }
}
