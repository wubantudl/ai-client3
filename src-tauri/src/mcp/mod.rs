use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::Result;
use directories::ProjectDirs;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServerConfig {
    pub name: String,
    pub transport: String,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPConnectionInfo {
    pub tools: Vec<MCPTool>,
    pub resources: Vec<serde_json::Value>,
    pub prompts: Vec<serde_json::Value>,
}

pub struct MCPServerState {
    pub servers: HashMap<String, MCPServerConfig>,
}

impl Default for MCPServerState {
    fn default() -> Self {
        Self { servers: HashMap::new() }
    }
}

pub fn get_mcp_state() -> Arc<Mutex<MCPServerState>> {
    use std::sync::OnceLock;
    static STATE: OnceLock<Arc<Mutex<MCPServerState>>> = OnceLock::new();
    STATE.get_or_init(|| Arc::new(Mutex::new(MCPServerState::default()))).clone()
}

pub fn get_mcp_config_path() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "ai-client", "AI Client") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)?;
        Ok(config_dir.join("mcp_servers.json"))
    } else {
        Err(anyhow::anyhow!("Could not determine config directory"))
    }
}

pub fn load_mcp_servers() -> Result<Vec<MCPServerConfig>> {
    let config_path = get_mcp_config_path()?;
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        let servers: Vec<MCPServerConfig> = serde_json::from_str(&content)?;
        info!("Loaded {} MCP servers", servers.len());
        Ok(servers)
    } else {
        save_mcp_servers(&vec![])?;
        Ok(vec![])
    }
}

pub fn save_mcp_servers(servers: &Vec<MCPServerConfig>) -> Result<()> {
    let config_path = get_mcp_config_path()?;
    let content = serde_json::to_string_pretty(servers)?;
    fs::write(&config_path, content)?;
    Ok(())
}

pub async fn connect_mcp_server(config: &MCPServerConfig) -> Result<MCPConnectionInfo> {
    info!("Connecting to MCP server: {}", config.name);
    let state = get_mcp_state();
    let mut state = state.lock().await;
    state.servers.insert(config.name.clone(), config.clone());
    Ok(MCPConnectionInfo {
        tools: vec![],
        resources: vec![],
        prompts: vec![],
    })
}

pub async fn disconnect_mcp_server(name: &str) -> Result<()> {
    let state = get_mcp_state();
    let mut state = state.lock().await;
    state.servers.remove(name);
    info!("Disconnected MCP server: {}", name);
    Ok(())
}

pub async fn call_mcp_tool(_server: &str, _tool: &str, _args: serde_json::Value) -> Result<serde_json::Value> {
    Ok(serde_json::json!({"success": true}))
}
