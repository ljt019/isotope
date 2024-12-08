use crate::models::{chat_manager::Message, inference_params_manager::InferenceParams};
use crate::utils::hub_load_safetensors;
use anyhow::{Error, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::{LogitsProcessor, Sampling};
use candle_transformers::models::llama::{Cache, Llama, LlamaConfig, LlamaEosToks};
use hf_hub::{api::sync::ApiBuilder, Repo, RepoType};
use log::{debug, info};
use std::sync::Arc;
use tauri::Manager;
use tauri::Runtime;
use tokenizers::Tokenizer;

/// Constants used in the model
const EOS_TOKEN: &str = "<|eot_id|>";
const BOS_TOKEN: &str = "<|begin_of_text|>";
const DEFAULT_MODEL: &str = "meta-llama/Llama-3.2-1B-Instruct";

/// Holds the resources required by the model
struct LlamaModelResources {
    llama: Llama,
    tokenizer: Tokenizer,
    config: candle_transformers::models::llama::Config,
}

impl LlamaModelResources {
    /// Loads the model resources from the Hugging Face Hub
    async fn load(model_id: &str) -> Result<Self> {
        let api = ApiBuilder::new()
            .with_token(Some(
                std::env::var("HF_TOKEN").expect("HF_TOKEN env not set"),
            ))
            .build()?;

        let revision = "main".to_string();

        let api = api.repo(Repo::with_revision(
            model_id.to_string(),
            RepoType::Model,
            revision,
        ));

        let tokenizer_filename = api.get("tokenizer.json")?;
        let config_filename = api.get("config.json")?;

        let config: LlamaConfig = serde_json::from_slice(&std::fs::read(config_filename)?)?;
        let config = config.into_config(false);

        let device = Device::cuda_if_available(0)?;
        let dtype = DType::F32;

        let filename: Vec<std::path::PathBuf>;

        // match the model_id, if its V32_3BInstruct then load the model from the local file system, otherwise load the model from the Hugging Face Hub
        match model_id {
            "meta-llama/Llama-3.2-3B-Instruct" => {
                filename = hub_load_safetensors(&api, "model.safetensors.index.json")?;
            }
            _ => {
                filename = vec![api.get("model.safetensors")?];
            }
        }

        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&filename, dtype, &device)? };
        let llama = Llama::load(vb, &config)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(Error::msg)?;

        Ok(LlamaModelResources {
            llama,
            tokenizer,
            config,
        })
    }
}

/// The main Model struct encapsulating the Llama model and tokenizer
pub struct LlamaModel {
    model_id: String,
    resources: Arc<LlamaModelResources>,
}

impl LlamaModel {
    /// Initializes the Model by loading the specified model.
    /// It handles downloading and caching as necessary.
    pub async fn new(model_id: Option<&str>) -> Result<Self> {
        let model_id = model_id.unwrap_or(DEFAULT_MODEL);
        debug!("Loading model: {}", model_id);
        let resources: LlamaModelResources = LlamaModelResources::load(model_id).await?;
        debug!("Model loaded: {}", model_id);
        Ok(Self {
            model_id: model_id.to_string(),
            resources: Arc::new(resources),
        })
    }

    /// formats the prompt in the required format for the model
    pub fn format_prompt(&self, messages: Vec<Message>) -> String {
        let mut formatted_messages = Vec::new();

        // Formats messages into a single prompt string.
        for msg in messages {
            match msg.role.as_str() {
                "system" => {
                    // Include system messages without special tokens
                    formatted_messages.push(format!("System:\n{}", msg.content));
                }
                "user" => {
                    // Wrap user messages with BOS_TOKEN and EOS_TOKEN
                    formatted_messages.push(format!("{}{}{}", BOS_TOKEN, msg.content, EOS_TOKEN));
                }
                "assistant" => {
                    // Include assistant messages without special tokens
                    formatted_messages.push(format!("Assistant:\n{}", msg.content));
                }
                _ => {
                    // Handle any other roles generically
                    formatted_messages.push(format!("{}:\n{}", msg.role, msg.content));
                }
            }
        }

        // Append "Assistant:" to signal the model to generate a response
        formatted_messages.push("Assistant:".to_string());

        formatted_messages.join("\n\n") // Clear separation between messages
    }

    /// Generates a response to the given prompt
    pub async fn inference<R: Runtime>(
        &self,
        inference_params: InferenceParams,
        app_handle: tauri::AppHandle<R>,
    ) -> Result<String, anyhow::Error> {
        // Encode the prompt
        let tokens = self
            .resources
            .tokenizer
            .encode(inference_params.prompt, true)
            .map_err(|e| anyhow::anyhow!("Encoding error: {}", e))?
            .get_ids()
            .to_vec();

        let eos_token_id = self
            .resources
            .tokenizer
            .token_to_id(EOS_TOKEN)
            .map(LlamaEosToks::Single)
            .or_else(|| self.resources.config.eos_token_id.clone());

        let temperature = inference_params.temperature;
        let sampling = if temperature <= 0.0 {
            Sampling::ArgMax
        } else {
            match (inference_params.top_k, inference_params.top_p) {
                (Some(k), Some(p)) => Sampling::TopKThenTopP { k, p, temperature },
                (Some(k), None) => Sampling::TopK { k, temperature },
                (None, Some(p)) => Sampling::TopP { p, temperature },
                (None, None) => Sampling::All { temperature },
            }
        };

        let mut logits_processor = LogitsProcessor::from_sampling(inference_params.seed, sampling);

        let device = Device::cuda_if_available(0)?;
        let dtype = DType::F32;
        let mut cache = Cache::new(true, dtype, &self.resources.config, &device)?;

        let start_gen = std::time::Instant::now();
        let mut token_output = Vec::new();
        let mut tokens = tokens.clone();
        let mut token_generated = 0;

        for _ in 0..inference_params.max_tokens {
            let (context_size, context_index) = if cache.use_kv_cache && token_generated > 0 {
                (1, tokens.len())
            } else {
                (tokens.len(), 0)
            };

            let ctxt = &tokens[tokens.len().saturating_sub(context_size)..];
            let input = Tensor::new(ctxt, &device)?.unsqueeze(0)?;
            let logits = self
                .resources
                .llama
                .forward(&input, context_index, &mut cache)?
                .squeeze(0)?;

            let logits = if inference_params.repeat_penalty != 1.0 {
                let start_at = tokens.len().saturating_sub(inference_params.repeat_last_n);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    inference_params.repeat_penalty,
                    &tokens[start_at..],
                )?
            } else {
                logits
            };

            let next_token = logits_processor.sample(&logits)?;
            token_generated += 1;
            tokens.push(next_token);
            token_output.push(next_token);

            // Decode the token
            let generated_text = self
                .resources
                .tokenizer
                .decode(&[next_token], true)
                .map_err(|e| anyhow::anyhow!("Decoding error: {}", e))?;

            // Split the generated text into individual chars
            let generated_text = generated_text.chars().collect::<Vec<char>>();

            for char in generated_text {
                // Emit each char as a token
                let emit_result = app_handle.emit_all("chat-token", char.to_string());
                if let Err(e) = emit_result {
                    eprintln!("Failed to emit token: {}", e);
                    return Err(anyhow::anyhow!("Failed to emit token: {}", e));
                }

                // Add small delay between characters
                let delay = if char.is_whitespace() {
                    // Longer pause at word boundaries
                    tokio::time::Duration::from_millis(10)
                } else {
                    // Shorter pause between characters
                    tokio::time::Duration::from_millis(5)
                };

                tokio::time::sleep(delay).await;
            }

            /*
            let emit_result = app_handle.emit_all("chat-token", generated_text.clone());
            if let Err(e) = emit_result {
                eprintln!("Failed to emit token: {}", e);
                return Err(anyhow::anyhow!("Failed to emit token: {}", e));
            }
            */

            // Check for end-of-sequence
            if let Some(LlamaEosToks::Single(eos_tok_id)) = eos_token_id {
                if next_token == eos_tok_id {
                    break;
                }
            }
        }

        let generation_time = start_gen.elapsed().as_secs_f64();
        info!(
            "Generated {} tokens in {:.2} seconds",
            token_generated, generation_time
        );

        // Emit a completion event with json containing the generated text, the number of tokens generated, the generation time, and the model id.
        let _ = app_handle
            .emit_all(
                "chat-end",
                serde_json::json!({
                    "text": self
                        .resources
                        .tokenizer
                        .decode(&token_output, true)
                        .map_err(|e| anyhow::anyhow!("Decoding error: {}", e))?,
                    "tokens": token_generated,
                    "time": generation_time,
                    "model": self.model_id,
                }),
            )
            .map_err(|e| anyhow::anyhow!("Failed to emit completion event: {}", e));

        // Combine the tokens into a single string
        let generated_text = self
            .resources
            .tokenizer
            .decode(&token_output, true)
            .map_err(|e| anyhow::anyhow!("Decoding error: {}", e))?;

        Ok(generated_text)
    }
}

impl LlamaModel {}
