// ZQ Master Bridge — Tauri backend entry point.
// Exposes Tauri commands consumed by the React 19 frontend.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct Conversation {
    id: i64,
    title: String,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: i64,
    conversation_id: i64,
    role: String,
    content: String,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Agent {
    id: i64,
    name: String,
    description: Option<String>,
    enabled: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: i64,
    agent_name: String,
    payload: String,
    status: String,
    result: Option<String>,
    error: Option<String>,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Log {
    id: i64,
    level: String,
    source: String,
    message: String,
    created_at: String,
}

struct AppState {
    db: Mutex<Connection>,
}

// ---------------------------------------------------------------------------
// Database initialization
// ---------------------------------------------------------------------------

fn init_db() -> SqlResult<Connection> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path)?;

    // Apply schema
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS conversations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL DEFAULT 'New Conversation',
            created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
            updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
        );

        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            conversation_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
            role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
            content TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
        );

        CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages (conversation_id, created_at);

        CREATE TABLE IF NOT EXISTS agents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            enabled INTEGER NOT NULL DEFAULT 1,
            config_json TEXT NOT NULL DEFAULT '{}',
            created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
        );

        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            agent_name TEXT NOT NULL,
            payload TEXT NOT NULL DEFAULT '{}',
            status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'running', 'done', 'failed')),
            result TEXT,
            error TEXT,
            created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
            updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
        );

        CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks (status, created_at);

        CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level TEXT NOT NULL DEFAULT 'info' CHECK (level IN ('debug', 'info', 'warn', 'error')),
            source TEXT NOT NULL,
            message TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
        );

        INSERT OR IGNORE INTO agents (name, description) VALUES
            ('echo', 'Echo agent — returns input unchanged'),
            ('summarize', 'Placeholder summarizer agent');
        "#,
    )?;

    Ok(conn)
}

fn get_db_path() -> PathBuf {
    let mut path = tauri::api::path::app_data_dir(&tauri::Config::default())
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("zq.db");
    path
}

// ---------------------------------------------------------------------------
// Tauri Commands
// ---------------------------------------------------------------------------

#[tauri::command]
fn get_conversations(state: State<AppState>) -> Result<Vec<Conversation>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db
        .prepare("SELECT id, title, created_at FROM conversations ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<SqlResult<Vec<_>>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn create_conversation(state: State<AppState>, title: String) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.execute("INSERT INTO conversations (title) VALUES (?1)", params![title])
        .map_err(|e| e.to_string())?;
    Ok(db.last_insert_rowid())
}

#[tauri::command]
fn get_messages(state: State<AppState>, conversation_id: i64) -> Result<Vec<Message>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db
        .prepare(
            "SELECT id, conversation_id, role, content, created_at FROM messages WHERE conversation_id = ?1 ORDER BY created_at ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([conversation_id], |row| {
            Ok(Message {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<SqlResult<Vec<_>>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn add_message(
    state: State<AppState>,
    conversation_id: i64,
    role: String,
    content: String,
) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.execute(
        "INSERT INTO messages (conversation_id, role, content) VALUES (?1, ?2, ?3)",
        params![conversation_id, role, content],
    )
    .map_err(|e| e.to_string())?;
    Ok(db.last_insert_rowid())
}

#[tauri::command]
fn get_agents(state: State<AppState>) -> Result<Vec<Agent>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db
        .prepare("SELECT id, name, description, enabled FROM agents ORDER BY name ASC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Agent {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                enabled: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<SqlResult<Vec<_>>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_agent(state: State<AppState>, agent_id: i64, enabled: i32) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.execute(
        "UPDATE agents SET enabled = ?1 WHERE id = ?2",
        params![enabled, agent_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db
        .prepare(
            "SELECT id, agent_name, payload, status, result, error, created_at FROM tasks ORDER BY created_at DESC LIMIT 100",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                agent_name: row.get(1)?,
                payload: row.get(2)?,
                status: row.get(3)?,
                result: row.get(4)?,
                error: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<SqlResult<Vec<_>>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_logs(state: State<AppState>, limit: i64) -> Result<Vec<Log>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db
        .prepare(
            "SELECT id, level, source, message, created_at FROM logs ORDER BY created_at DESC LIMIT ?1",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([limit], |row| {
            Ok(Log {
                id: row.get(0)?,
                level: row.get(1)?,
                source: row.get(2)?,
                message: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<SqlResult<Vec<_>>>().map_err(|e| e.to_string())
}

#[tauri::command]
async fn dispatch_agent(
    state: State<'_, AppState>,
    agent_name: String,
    task: String,
) -> Result<String, String> {
    // Log task creation
    {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        db.execute(
            "INSERT INTO tasks (agent_name, payload, status) VALUES (?1, ?2, 'running')",
            params![agent_name, task],
        )
        .map_err(|e| e.to_string())?;
    }

    // Placeholder: call Python sidecar via HTTP (localhost:8765)
    // In production, replace with real HTTP client call to agents/main.py
    let response = format!("[Echo from {}]: {}", agent_name, task);

    // Update task status
    {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        db.execute(
            "UPDATE tasks SET status = 'done', result = ?1 WHERE agent_name = ?2 AND status = 'running'",
            params![response, agent_name],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(response)
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let db = init_db().expect("Failed to initialize database");

    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(db),
        })
        .invoke_handler(tauri::generate_handler![
            get_conversations,
            create_conversation,
            get_messages,
            add_message,
            get_agents,
            toggle_agent,
            get_tasks,
            get_logs,
            dispatch_agent,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
