// ZQ Master Bridge — Tauri backend entry point.
// Exposes Tauri commands consumed by the React 19 frontend.

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![chat, agent_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
