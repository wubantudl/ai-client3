use crate::llm::{self, ChatMessage, ModelConfig};
use tauri::Emitter;

#[tauri::command]
pub async fn chat(model_id: String, messages: Vec<ChatMessage>) -> Result<String, String> {
    let config = llm::load_config().map_err(|e| e.to_string())?;
    
    let model_config = config.models.iter()
        .find(|m| m.id == model_id)
        .ok_or_else(|| format!("Model not found: {}", model_id))?;

    match llm::chat(model_config, messages).await {
        Ok(response) => Ok(response.choices[0].message.content.clone()),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub async fn chat_stream(
    window: tauri::Window,
    model_id: String, 
    messages: Vec<ChatMessage>
) -> Result<(), String> {
    let config = llm::load_config().map_err(|e| e.to_string())?;
    
    let model_config = config.models.iter()
        .find(|m| m.id == model_id)
        .ok_or_else(|| format!("Model not found: {}", model_id))?;

    let window_clone = window.clone();
    
    match llm::chat_stream(model_config, messages, move |chunk| {
        let _ = window_clone.emit("stream-chunk", serde_json::json!({ "content": chunk }));
    }).await {
        Ok(_) => {
            let _ = window.emit("stream-done", ());
            Ok(())
        }
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub fn get_models() -> Result<Vec<ModelConfig>, String> {
    let config = llm::load_config().map_err(|e| e.to_string())?;
    Ok(config.models)
}

#[tauri::command]
pub fn save_model_config(models: Vec<ModelConfig>) -> Result<(), String> {
    let mut config = llm::load_config().map_err(|e| e.to_string())?;
    config.models = models;
    llm::save_config(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_model_config() -> Result<llm::AppConfig, String> {
    llm::load_config().map_err(|e| e.to_string())
}
