use serde_json::json;
use serde_json::Value;

/// Builds the `browsingContext.getTree` command
pub fn get_tree_command(command_id: u32) -> Value {
    json!({
        "id": command_id,
        "method": "browsingContext.getTree",
        "params": {}
    })
}

/// Builds the `browsingContext.navigate` command
pub fn navigate_command(command_id: u32, context_id: &str, url: &str) -> Value {
    json!({
        "id": command_id,
        "method": "browsingContext.navigate",
        "params": {
            "context": context_id,
            "url": url
        }
    })
}
