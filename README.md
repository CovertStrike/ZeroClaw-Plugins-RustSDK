# ZeroClaw Rust Plugin SDK

Build WASM plugins for ZeroClaw in Rust.

## Requirements

- Rust toolchain with `wasm32-unknown-unknown` target
- Install target: `rustup target add wasm32-unknown-unknown`

## Building

```bash
./build.sh
```

Output is placed in `release/`:
- `zeroclaw_plugin_sdk.wasm` - the compiled plugin
- `plugin.toml` - plugin manifest

## Plugin Entry Points

### `on_message`

Called when a message is received. Receives JSON input with the message and metadata, returns a response.

```rust
use zeroclaw_plugin_sdk::{MessageInput, MessageOutput};
use extism_pdk::*;

#[plugin_fn]
pub fn on_message(Json(input): Json<MessageInput>) -> FnResult<Json<MessageOutput>> {
    Ok(Json(MessageOutput {
        response: format!("You said: {}", input.message),
        error: None,
    }))
}
```

### `on_load`

Called when the plugin is first loaded. Use for initialization.

```rust
#[plugin_fn]
pub fn on_load() -> FnResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "status": "loaded",
        "name": "my-plugin"
    })))
}
```

## SDK Modules

### Memory

Store and recall persistent data.

```rust
use zeroclaw_plugin_sdk::memory;

// Store a value
memory::store("user:prefs", "{\"theme\": \"dark\"}")?;

// Recall by query
let results = memory::recall("user preferences")?;

// Forget a key
memory::forget("user:prefs")?;
```

### Messaging

Send messages to channels.

```rust
use zeroclaw_plugin_sdk::messaging;

// Get available channels
let channels = messaging::get_channels()?;

// Send a message
messaging::send("slack", "@user", "Hello from the plugin!")?;
```

### Tools

Call ZeroClaw tools.

```rust
use zeroclaw_plugin_sdk::tools;
use serde_json::json;

let result = tools::tool_call("web_search", json!({
    "query": "rust wasm plugins"
}))?;
```

### Context

Access session, user, and agent information.

```rust
use zeroclaw_plugin_sdk::context;

// Get session info (channel, conversation ID, timestamp)
let session = context::session()?;
println!("Channel: {}", session.channel_name);

// Get user who triggered this invocation
let user = context::user_identity()?;
println!("User: {}", user.display_name);

// Get agent configuration
let agent = context::agent_config()?;
println!("Agent: {}", agent.name);
```

## License

MIT OR Apache-2.0
