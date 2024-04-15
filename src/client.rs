use reqwest::blocking::Client;
use serde_json::{json, Value};

use std::env;
use base64::Engine;
use base64::engine::general_purpose;
use dotenv::dotenv;

pub struct Anthropic {
    api_key: String,
    api_url: String,
    api_version: String,
    pub messages: Messages,
}

impl Anthropic {
    pub fn new() -> Self {
        dotenv().ok();
        let config = AnthropicEnvConfig;
        let api_key = config.get_api_key();
        let api_url = config.get_api_url();
        let api_version = config.get_api_version();
        Anthropic {
            api_key,
            api_url,
            api_version,
            messages: Messages,
        }
    }

    pub fn pack_vision_content(image_url: String, image_media_type: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>>{

        let client = Client::new();
        let response = client.get(image_url).send()?;
        let image_array_buffer = response.bytes()?;
        let image_data = general_purpose::STANDARD.encode(image_array_buffer);

        let content = vec![
            json!({
             "type": "image",
             "source": {
                "type": "base64",
                "media_type": image_media_type,
                "data": image_data,
            },
        })
        ];
        let messages = vec![
            json!({"role": "user", "content": content}),
        ];
        Ok(messages)
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    pub fn get_api_url(&self) -> &str {
        &self.api_url
    }

    pub fn get_api_version(&self) -> &str {
        &self.api_version
    }
}

pub struct Messages;

impl Messages {
    pub fn create(
        &self,
        anthropic_client: &Anthropic,
        model: &str,
        max_tokens: usize,
        messages: &[Value],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();

        let response = client
            .post(anthropic_client.get_api_url())
            .header("x-api-key", anthropic_client.get_api_key())
            .header("anthropic-version", anthropic_client.get_api_version())
            .header("content-type", "application/json")
            .json(&json!({
                "model": model,
                "max_tokens": max_tokens,
                "messages": messages
            }))
            .send()?;

        response.text().map_err(Into::into)
    }
}


trait AnthropicConfig {
    fn get_api_key(&self) -> String;
    fn get_api_url(&self) -> String;
    fn get_api_version(&self) -> String;
}
struct AnthropicEnvConfig;

impl AnthropicConfig for AnthropicEnvConfig {

    fn get_api_key(&self) -> String {
        env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set")
    }

    fn get_api_url(&self) -> String {
        env::var("ANTHROPIC_API_URL").expect("ANTHROPIC_API_URL must be set")
    }

    fn get_api_version(&self) -> String {
        env::var("ANTHROPIC_API_VERSION").expect("ANTHROPIC_API_VERSION must be set")
    }
}