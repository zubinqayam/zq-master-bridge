// ZQ Master Bridge — shared Tauri entry point for desktop and mobile.
// Mobile targets compile this crate as a library, while desktop uses main.rs
// to call the same run() entry point.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;

/// Simple echo-style chat handler.
/// Replace with real AI / agent invocation in production.
#[command]
fn chat(message: String) -> String {
    format!("ZQ AI Response: {}", message)
}

/// Return current agent status (placeholder).
#[command]
fn agent_status() -> serde_json::Value {
    serde_json::json!({
        "status": "idle",
        "version": "2.0.0",
        "agents": 0
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![chat, agent_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
