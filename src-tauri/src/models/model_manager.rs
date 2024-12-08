use crate::database::pool::DbPool;
use crate::database::Chat;
use crate::models::chat_manager::ChatManager;
use crate::models::inference_params_manager::InferenceParamsManager;
use crate::models::llama::llama_models::LlamaModel as Model;
use log::info;

use super::llama::llama_options::LlamaOptions;

/// Manages the active model, chat history, and inference parameters
pub struct ModelManager {
    model: Model,
    inference_params_manager: InferenceParamsManager,
    chat_manager: ChatManager,
}

/// General implementation of ModelManager
impl ModelManager {
    /// Create a new ModelManager with the given model and database connection
    pub async fn new(app_handle: tauri::AppHandle, pool: DbPool) -> rusqlite::Result<Self> {
        let inference_params_manager = InferenceParamsManager::new(app_handle);
        let chat_manager = ChatManager::new(pool)?;

        // Create model based on selected model or default
        let model = match inference_params_manager.get_current_model_value() {
            Some(model_name) => Model::new(Some(&model_name))
                .await
                .expect("Failed to load model"),
            None => Model::new(None).await.expect("Failed to load model"),
        };

        Ok(Self {
            model,
            chat_manager,
            inference_params_manager,
        })
    }

    /// Chat with the model, with the given prompt
    pub async fn chat(
        &mut self,
        prompt: String,
        app_handle: tauri::AppHandle,
    ) -> Result<String, rusqlite::Error> {
        let prompt = crate::models::chat_manager::Message {
            role: "user".to_string(),
            content: prompt,
        };

        // Handle database operations
        let full_chat_prompt = self.chat_manager.handle_prompt(prompt)?;

        // Format prompt and get inference params
        let formatted_prompt = self.model.format_prompt(full_chat_prompt);
        let inference_params = self
            .inference_params_manager
            .get_inference_params_with_prompt(formatted_prompt);

        // Generate response
        let response = crate::models::chat_manager::Message {
            role: "assistant".to_string(),
            content: self
                .model
                .inference(inference_params, app_handle)
                .await
                .expect("Failed to generate response"),
        };

        // Save response
        self.chat_manager.handle_response(response.clone())?;

        // Return response
        Ok(response.content)
    }
}

/// Chat management methods
impl ModelManager {
    pub async fn new_chat(&mut self) {
        self.chat_manager
            .new_chat()
            .expect("Failed to create new chat");
    }

    pub async fn get_current_chat(&self) -> &Chat {
        self.chat_manager.get_current_chat()
    }

    pub async fn get_chat_history(&self) -> Vec<Chat> {
        self.chat_manager
            .get_all_chats()
            .expect("Failed to get chat history")
    }
}

/// LLM model management methods
impl ModelManager {
    pub async fn set_model(&mut self, model: LlamaOptions) {
        self.model = Model::new(Some(&model.get_model_name()))
            .await
            .expect("Failed to load model");

        let name = model.get_model_name().to_string();

        self.inference_params_manager.set_model(model);

        info!("Model set to: {}", name);
    }

    pub async fn get_current_model(&self) -> String {
        let current_model_value = self
            .inference_params_manager
            .get_current_model_value()
            .expect("Failed to get current model");

        info!("Current model: {}", current_model_value);

        current_model_value
    }
}
