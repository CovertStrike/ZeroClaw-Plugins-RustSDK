pub mod memory;
pub mod tools;
pub mod messaging;
pub mod context;

use extism_pdk::*;
use serde::{Deserialize, Serialize};

/// Input passed to the plugin when a message is received.
#[derive(Debug, Deserialize)]
pub struct MessageInput {
    pub message: String,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Output returned by the plugin after processing a message.
#[derive(Debug, Serialize)]
pub struct MessageOutput {
    pub response: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Main entry point called by ZeroClaw when a message is received.
#[plugin_fn]
pub fn on_message(Json(input): Json<MessageInput>) -> FnResult<Json<MessageOutput>> {
    // Example: echo the message back with context info
    let session = context::session().ok();
    let user = context::user_identity().ok();

    let response = format!(
        "Received: '{}' from {} on {}",
        input.message,
        user.map(|u| u.display_name).unwrap_or_else(|| "unknown".into()),
        session.map(|s| s.channel_name).unwrap_or_else(|| "unknown".into()),
    );

    Ok(Json(MessageOutput {
        response,
        error: None,
    }))
}

/// Called when the plugin is first loaded.
#[plugin_fn]
pub fn on_load() -> FnResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "status": "loaded",
        "name": "zeroclaw-plugin-sdk"
    })))
}
