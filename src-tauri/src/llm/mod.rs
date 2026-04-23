use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use directories::ProjectDirs;
use tracing::info;
use futures_util::StreamExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub id: String,
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    #[serde(rename = "type")]
    pub model_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub models: Vec<ModelConfig>,
    pub active_model: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            models: vec![
                ModelConfig {
                    id: "deepseek-chat".to_string(),
                    name: "DeepSeek V3".to_string(),
                    base_url: "https://api.deepseek.com/v1".to_string(),
                    api_key: String::new(),
                    model_type: "online".to_string(),
                },
                ModelConfig {
                    id: "moonshot-v1-8k".to_string(),
                    name: "Kimi".to_string(),
                    base_url: "https://api.moonshot.cn/v1".to_string(),
                    api_key: String::new(),
                    model_type: "online".to_string(),
                },
                ModelConfig {
                    id: "glm-4".to_string(),
                    name: "智谱 GLM-4".to_string(),
                    base_url: "https://open.bigmodel.cn/api/paas/v4".to_string(),
                    api_key: String::new(),
                    model_type: "online".to_string(),
                },
                ModelConfig {
                    id: "ollama-llama3".to_string(),
                    name: "Ollama Llama3".to_string(),
                    base_url: "http://localhost:11434/v1".to_string(),
                    api_key: String::new(),
                    model_type: "local".to_string(),
                },
            ],
            active_model: Some("deepseek-chat".to_string()),
        }
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "ai-client", "AI Client") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)?;
        Ok(config_dir.join("config.json"))
    } else {
        Err(anyhow::anyhow!("Could not determine config directory"))
    }
}

pub fn load_config() -> Result<AppConfig> {
    let config_path = get_config_path()?;
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        info!("Loaded config");
        Ok(config)
    } else {
        let config = AppConfig::default();
        save_config(&config)?;
        info!("Created default config");
        Ok(config)
    }
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let config_path = get_config_path()?;
    let content = serde_json::to_string_pretty(config)?;
    fs::write(&config_path, content)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ChatMessageResponse,
}

#[derive(Debug, Deserialize)]
pub struct ChatMessageResponse {
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct StreamResponse {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
}

#[derive(Debug, Deserialize)]
struct StreamDelta {
    content: Option<String>,
}

pub async fn chat(config: &ModelConfig, messages: Vec<ChatMessage>) -> Result<ChatResponse> {
    let client = reqwest::Client::new();
    let request_body = ChatRequest {
        model: config.id.clone(),
        messages,
        stream: false,
    };

    let response = client
        .post(format!("{}/chat/completions", config.base_url))
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("API request failed: {}", error_text));
    }

    Ok(response.json().await?)
}

pub async fn chat_stream<F>(config: &ModelConfig, messages: Vec<ChatMessage>, callback: F) -> Result<()>
where
    F: Fn(String) + Send + 'static,
{
    let client = reqwest::Client::new();
    let request_body = ChatRequest {
        model: config.id.clone(),
        messages,
        stream: true,
    };

    let response = client
        .post(format!("{}/chat/completions", config.base_url))
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("API request failed: {}", error_text));
    }

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    
    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                for byte in bytes {
                    let ch = byte as char;
                    if ch == '\n' {
                        if buffer.starts_with("data: ") {
                            let data = &buffer[6..];
                            if data.trim() != "[DONE]" && !data.is_empty() {
                                if let Ok(event) = serde_json::from_str::<StreamResponse>(data) {
                                    if let Some(content) = event.choices.get(0)
                                        .and_then(|c| c.delta.content.as_ref()) 
                                    {
                                        callback(content.clone());
                                    }
                                }
                            }
                        }
                        buffer.clear();
                    } else {
                        buffer.push(ch);
                    }
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Stream error: {}", e));
            }
        }
    }

    Ok(())
}
