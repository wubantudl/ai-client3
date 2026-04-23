pub mod llm;
pub mod mcp;
pub mod skills;
pub mod commands;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_client=info,tauri=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
