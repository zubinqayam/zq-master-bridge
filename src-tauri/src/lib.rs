#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use serde_json::{json, Value};
use std::{fs, path::{Path, PathBuf}};
use tauri::{command, Manager, State};

#[cfg(target_os = "windows")]
use std::process::Command;

const SCHEMA_VERSION: i64 = 2;
const WORKSPACE_NAME: &str = "Saada";
const DEPARTMENT_NAME: &str = "General Surgery";

struct AppState {
    db_path: PathBuf,
}

#[derive(Serialize)]
struct Conversation {
    id: i64,
    title: String,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize)]
struct Message {
    id: i64,
    conversation_id: i64,
    role: String,
    content: String,
    created_at: String,
}

#[derive(Serialize)]
struct Agent {
    id: i64,
    name: String,
    label: String,
    description: String,
    category: String,
    enabled: bool,
    module_id: Option<String>,
}

#[derive(Serialize)]
struct Task {
    id: i64,
    conversation_id: Option<i64>,
    agent_name: String,
    title: String,
    status: String,
    result: Option<String>,
    error: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize)]
struct LogEntry {
    id: i64,
    level: String,
    source: String,
    message: String,
    created_at: String,
}

#[derive(Serialize)]
struct WorkspaceItem {
    id: i64,
    item_key: String,
    parent_key: Option<String>,
    kind: String,
    label: String,
    description: Option<String>,
    item_path: Option<String>,
    badge: Option<String>,
    sort_order: i64,
}

#[derive(Serialize)]
struct ModuleRecord {
    id: String,
    label: String,
    description: String,
    kind: String,
    phase: String,
    status: String,
    root_path: Option<String>,
    launch_path: Option<String>,
    details: String,
}

#[derive(Serialize)]
struct WorkstationSnapshot {
    workspace_name: String,
    department_name: String,
    active_conversation_id: Option<i64>,
    conversations: Vec<Conversation>,
    messages: Vec<Message>,
    agents: Vec<Agent>,
    tasks: Vec<Task>,
    logs: Vec<LogEntry>,
    workspace_items: Vec<WorkspaceItem>,
    file_items: Vec<WorkspaceItem>,
    modules: Vec<ModuleRecord>,
}

fn open_db(path: &Path) -> Result<Connection, String> {
    let conn = Connection::open(path).map_err(|err| err.to_string())?;
    conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")
        .map_err(|err| err.to_string())?;
    Ok(conn)
}

fn count_rows(conn: &Connection, table: &str) -> Result<i64, String> {
    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| row.get(0))
        .map_err(|err| err.to_string())
}

fn initialize_database(db_path: &Path) -> Result<(), String> {
    let conn = open_db(db_path)?;
    let version = conn
        .query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
        .map_err(|err| err.to_string())?;

    if version < SCHEMA_VERSION {
        conn.execute_batch(include_str!("../../database/schema.sql"))
            .map_err(|err| err.to_string())?;
        conn.pragma_update(None, "user_version", SCHEMA_VERSION)
            .map_err(|err| err.to_string())?;
    }

    seed_workspace(&conn)?;
    seed_agents(&conn)?;
    seed_conversations(&conn)?;
    seed_tasks(&conn)?;
    sync_modules(&conn)?;
    log_event(&conn, "info", "bootstrap", "Workstation ready")?;
    Ok(())
}

fn seed_workspace(conn: &Connection) -> Result<(), String> {
    if count_rows(conn, "workspace_items")? > 0 {
        return Ok(());
    }

    let items = [
        ("workspace.saada", None, "workspace", "Saada", "Clinical workspace root", None, Some("online"), 0),
        ("view.home", Some("workspace.saada"), "view", "Home", "Primary workstation view", None, None, 10),
        ("view.files", Some("workspace.saada"), "view", "Files", "Clinical file workspace", None, None, 20),
        ("view.analytics", Some("workspace.saada"), "view", "Analytics", "Operational insights", None, None, 30),
        ("view.operations", Some("workspace.saada"), "view", "Operations", "Tasks, modules, and logs", None, None, 40),
        ("dept.root", Some("workspace.saada"), "section", "Departments", "Department tree", None, None, 50),
        ("dept.general_surgery", Some("dept.root"), "department", "General Surgery", "Active workstation context", None, Some("active"), 60),
        ("dept.general_surgery.reports", Some("dept.general_surgery"), "folder", "Reports", "Patient and service reports", None, None, 70),
        ("dept.general_surgery.schedules", Some("dept.general_surgery"), "folder", "Schedules", "Procedure schedules", None, None, 80),
        ("dept.orthopedics", Some("dept.root"), "department", "Orthopedics", "Department workspace", None, None, 90),
        ("dept.urology", Some("dept.root"), "department", "Urology", "Department workspace", None, None, 100),
        ("file.patient_lab", Some("dept.general_surgery.reports"), "file", "Patient_A_Lab_Results.pdf", "Recent pathology report", Some("Patient Reports/2024/May/Patient_A_Lab_Results.pdf"), Some("pdf"), 110),
        ("file.surgical_notes", Some("dept.general_surgery.reports"), "file", "Surgical_Notes_B.docx", "Procedure summary", Some("Patient Reports/2024/May/Surgical_Notes_B.docx"), Some("docx"), 120),
        ("file.consults", Some("dept.general_surgery.schedules"), "file", "General_Surgery_Consultations.xlsx", "Consultation schedule", Some("Schedules/General_Surgery_Consultations.xlsx"), Some("xlsx"), 130),
        ("module.group", Some("workspace.saada"), "section", "Modules", "External ZQ systems", None, None, 140),
    ];

    for (item_key, parent_key, kind, label, description, item_path, badge, sort_order) in items {
        conn.execute(
            "INSERT OR IGNORE INTO workspace_items
             (item_key, parent_key, kind, label, description, item_path, badge, sort_order)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![item_key, parent_key, kind, label, description, item_path, badge, sort_order],
        )
        .map_err(|err| err.to_string())?;
    }

    Ok(())
}

fn seed_agents(conn: &Connection) -> Result<(), String> {
    let agents = [
        ("inam_brain", "Inam Brain", "Primary clinical collaborator for workstation requests.", "assistant", 1, Option::<&str>::None),
        ("coordinator_bridge", "Coordinator Bridge", "Launch and route work into ZQ Coordinator.", "bridge", 1, Some("coordinator")),
        ("taskbox_runner", "Taskbox Runner", "Launch and monitor INNM Taskbox workflows.", "operations", 1, Some("taskbox")),
    ];

    for (name, label, description, category, enabled, module_id) in agents {
        conn.execute(
            "INSERT OR IGNORE INTO agents (name, label, description, category, enabled, module_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![name, label, description, category, enabled, module_id],
        )
        .map_err(|err| err.to_string())?;
    }

    Ok(())
}

fn seed_conversations(conn: &Connection) -> Result<(), String> {
    if count_rows(conn, "conversations")? > 0 {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO conversations (title) VALUES (?1)",
        params![format!("{WORKSPACE_NAME} / {DEPARTMENT_NAME}")],
    )
    .map_err(|err| err.to_string())?;

    let conversation_id = conn.last_insert_rowid();
    let messages = [
        ("assistant", "Welcome to the Saada / General Surgery workstation. The Woods storage matrix is synchronized. How can I assist you with your department data today?"),
        ("user", "I need to upload and map the new patient reports for this week."),
        ("assistant", "I can stage the files workspace, prepare a task in Taskbox, and open Coordinator if cross-system routing is needed."),
    ];

    for (role, content) in messages {
        conn.execute(
            "INSERT INTO messages (conversation_id, role, content) VALUES (?1, ?2, ?3)",
            params![conversation_id, role, content],
        )
        .map_err(|err| err.to_string())?;
    }

    Ok(())
}

fn seed_tasks(conn: &Connection) -> Result<(), String> {
    if count_rows(conn, "tasks")? > 0 {
        return Ok(());
    }

    let conversation_id: Option<i64> = conn
        .query_row(
            "SELECT id FROM conversations ORDER BY id LIMIT 1",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|err| err.to_string())?;

    let tasks = [
        (
            conversation_id,
            "inam_brain",
            "Review uploaded patient reports",
            "{\"priority\":\"high\"}",
            "done",
            Some("Prepared report intake checklist and file staging guidance."),
            Option::<&str>::None,
        ),
        (
            conversation_id,
            "taskbox_runner",
            "Prepare weekly import queue",
            "{\"queue\":\"general-surgery\"}",
            "running",
            Some("Taskbox is ready to accept new intake items."),
            Option::<&str>::None,
        ),
    ];

    for (conv_id, agent_name, title, payload, status, result, error) in tasks {
        conn.execute(
            "INSERT INTO tasks (conversation_id, agent_name, title, payload, status, result, error)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![conv_id, agent_name, title, payload, status, result, error],
        )
        .map_err(|err| err.to_string())?;
    }

    Ok(())
}

fn log_event(conn: &Connection, level: &str, source: &str, message: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO logs (level, source, message) VALUES (?1, ?2, ?3)",
        params![level, source, message],
    )
    .map_err(|err| err.to_string())?;
    Ok(())
}

fn sync_modules(conn: &Connection) -> Result<(), String> {
    let root = PathBuf::from(r"C:\Users\zubin\ZQ_COORDINATOR");
    let taskbox_root = PathBuf::from(r"C:\ZQ_Taskbox");
    let coordinator_launch = first_existing_path(&[
        root.join(r"DELIVERABLES\2026-03-13\windows\zq-connect-windows.exe"),
        root.join(r"DELIVERABLES\2026-03-13\windows\ZQ Connect Enhanced_0.2.0_x64-setup.exe"),
    ]);
    let taskbox_launch = first_existing_path(&[
        taskbox_root.join(r"dist\INNM_Taskbox.exe"),
        taskbox_root.join(r"dist\INNM_Taskbox\INNM_Taskbox.exe"),
        taskbox_root.join("run.ps1"),
    ]);
    let feedback_path = taskbox_root.join("zq_feedback.py");
    let innm_path = taskbox_root.join("innm_controller.py");

    let modules = vec![
        module_row(
            "coordinator",
            "ZQ Coordinator",
            "External orchestration workspace and routing surface.",
            "desktop_app",
            "phase_1",
            if root.exists() && coordinator_launch.is_some() { "installed" } else { "missing" },
            Some(root.clone()),
            coordinator_launch,
            "Mandatory phase 1 integration.",
        ),
        module_row(
            "taskbox",
            "INNM Taskbox",
            "Task intake, routing, and desktop workflow runner.",
            "desktop_app",
            "phase_1",
            if taskbox_root.exists() && taskbox_launch.is_some() { "installed" } else { "missing" },
            Some(taskbox_root.clone()),
            taskbox_launch,
            "Mandatory phase 1 integration.",
        ),
        module_row(
            "feedback_bot",
            "ZQ Feedback Bot",
            "Deferred phase 2 feedback workflow component discovered in Taskbox.",
            "python_component",
            "phase_2",
            if feedback_path.exists() { "detected" } else { "planned" },
            Some(taskbox_root.clone()),
            if feedback_path.exists() { Some(feedback_path) } else { None },
            "Deferred until coordinator shell is stable.",
        ),
        module_row(
            "innm_wosds",
            "INNM / Woods",
            "Deferred phase 2 INNM-WOSDS controller component.",
            "python_component",
            "phase_2",
            if innm_path.exists() { "detected" } else { "planned" },
            Some(taskbox_root),
            if innm_path.exists() { Some(innm_path) } else { None },
            "Deferred until workstation shell is complete.",
        ),
        module_row(
            "keyhole",
            "Keyhole",
            "Reserved module slot for the Keyhole system.",
            "planned",
            "phase_2",
            "planned",
            None,
            None,
            "Path not yet assigned in this repository.",
        ),
    ];

    for module in modules {
        conn.execute(
            "INSERT INTO modules (id, label, description, kind, phase, status, root_path, launch_path, details)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT(id) DO UPDATE SET
                label = excluded.label,
                description = excluded.description,
                kind = excluded.kind,
                phase = excluded.phase,
                status = excluded.status,
                root_path = excluded.root_path,
                launch_path = excluded.launch_path,
                details = excluded.details",
            params![
                module.id,
                module.label,
                module.description,
                module.kind,
                module.phase,
                module.status,
                module.root_path,
                module.launch_path,
                module.details
            ],
        )
        .map_err(|err| err.to_string())?;
    }

    Ok(())
}

fn first_existing_path(paths: &[PathBuf]) -> Option<PathBuf> {
    paths.iter().find(|path| path.exists()).cloned()
}

fn module_row(
    id: &str,
    label: &str,
    description: &str,
    kind: &str,
    phase: &str,
    status: &str,
    root_path: Option<PathBuf>,
    launch_path: Option<PathBuf>,
    details: &str,
) -> ModuleRecord {
    ModuleRecord {
        id: id.to_string(),
        label: label.to_string(),
        description: description.to_string(),
        kind: kind.to_string(),
        phase: phase.to_string(),
        status: status.to_string(),
        root_path: root_path.map(|path| path.display().to_string()),
        launch_path: launch_path.map(|path| path.display().to_string()),
        details: details.to_string(),
    }
}

fn conversation_list(conn: &Connection) -> Result<Vec<Conversation>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, title, created_at, updated_at
             FROM conversations
             ORDER BY updated_at DESC, id DESC",
        )
        .map_err(|err| err.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })
        .map_err(|err| err.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())
}

fn message_list(conn: &Connection, conversation_id: i64) -> Result<Vec<Message>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, conversation_id, role, content, created_at
             FROM messages
             WHERE conversation_id = ?1
             ORDER BY id ASC",
        )
        .map_err(|err| err.to_string())?;

    let rows = stmt
        .query_map(params![conversation_id], |row| {
            Ok(Message {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|err| err.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())
}

fn agent_list(conn: &Connection) -> Result<Vec<Agent>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, label, description, category, enabled, module_id
             FROM agents
             ORDER BY category, label",
        )
        .map_err(|err| err.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Agent {
                id: row.get(0)?,
                name: row.get(1)?,
                label: row.get(2)?,
                description: row.get(3)?,
                category: row.get(4)?,
                enabled: row.get::<_, i64>(5)? == 1,
                module_id: row.get(6)?,
            })
        })
        .map_err(|err| err.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())
}

fn task_list(conn: &Connection) -> Result<Vec<Task>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, conversation_id, agent_name, title, status, result, error, created_at, updated_at
             FROM tasks
             ORDER BY id DESC
             LIMIT 30",
        )
        .map_err(|err| err.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                agent_name: row.get(2)?,
                title: row.get(3)?,
                status: row.get(4)?,
                result: row.get(5)?,
                error: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })
        .map_err(|err| err.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())
}

fn log_list(conn: &Connection, limit: i64) -> Result<Vec<LogEntry>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, level, source, message, created_at
             FROM logs
             ORDER BY id DESC
             LIMIT ?1",
        )
        .map_err(|err| err.to_string())?;

    let rows = stmt
        .query_map(params![limit], |row| {
            Ok(LogEntry {
                id: row.get(0)?,
                level: row.get(1)?,
                source: row.get(2)?,
                message: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|err| err.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())
}

fn workspace_item_list(conn: &Connection, file_only: bool) -> Result<Vec<WorkspaceItem>, String> {
    let sql = if file_only {
        "SELECT id, item_key, parent_key, kind, label, description, item_path, badge, sort_order
         FROM workspace_items
         WHERE kind IN ('section', 'department', 'folder', 'file')
         ORDER BY sort_order, label"
    } else {
        "SELECT id, item_key, parent_key, kind, label, description, item_path, badge, sort_order
         FROM workspace_items
         ORDER BY sort_order, label"
    };

    let mut stmt = conn.prepare(sql).map_err(|err| err.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(WorkspaceItem {
                id: row.get(0)?,
                item_key: row.get(1)?,
                parent_key: row.get(2)?,
                kind: row.get(3)?,
                label: row.get(4)?,
                description: row.get(5)?,
                item_path: row.get(6)?,
                badge: row.get(7)?,
                sort_order: row.get(8)?,
            })
        })
        .map_err(|err| err.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())
}

fn module_list(conn: &Connection) -> Result<Vec<ModuleRecord>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, label, description, kind, phase, status, root_path, launch_path, details
             FROM modules
             ORDER BY phase, label",
        )
        .map_err(|err| err.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ModuleRecord {
                id: row.get(0)?,
                label: row.get(1)?,
                description: row.get(2)?,
                kind: row.get(3)?,
                phase: row.get(4)?,
                status: row.get(5)?,
                root_path: row.get(6)?,
                launch_path: row.get(7)?,
                details: row.get(8)?,
            })
        })
        .map_err(|err| err.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())
}

fn assistant_reply(conn: &Connection, task: &str) -> Result<String, String> {
    let modules = module_list(conn)?;
    let coordinator = modules.iter().find(|module| module.id == "coordinator");
    let taskbox = modules.iter().find(|module| module.id == "taskbox");
    let task_lower = task.to_ascii_lowercase();
    let has_file_context = task_lower.contains("file")
        || task_lower.contains("report")
        || task_lower.contains("upload");

    let mut actions = vec!["I can keep this inside the Saada / General Surgery workstation.".to_string()];

    if has_file_context {
        actions.push("The Files view is ready with the current report folders and staged intake documents.".to_string());
    }

    if coordinator.is_some_and(|module| module.status == "installed") {
        actions.push("Coordinator is installed and can be opened for cross-system routing.".to_string());
    } else {
        actions.push("Coordinator is not installed on this machine, so routing will stay local.".to_string());
    }

    if taskbox.is_some_and(|module| module.status == "installed") {
        actions.push("Taskbox is available if you want this request converted into an operational queue item.".to_string());
    } else {
        actions.push("Taskbox is missing, so I will keep this as a workstation-only task.".to_string());
    }

    Ok(format!("{}\n\n{}", task.trim(), actions.join(" ")))
}

fn launch_target(path: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let mut command = if path.is_dir() {
            let mut cmd = Command::new("explorer");
            cmd.arg(path);
            cmd
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("ps1"))
        {
            let mut cmd = Command::new("powershell");
            cmd.args(["-ExecutionPolicy", "Bypass", "-File"]).arg(path);
            cmd
        } else {
            let mut cmd = Command::new(path);
            if let Some(parent) = path.parent() {
                cmd.current_dir(parent);
            }
            cmd
        };

        command.spawn().map_err(|err| err.to_string())?;
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = path;
        Err("External module launching is only available on Windows desktop builds.".to_string())
    }
}

#[command]
fn get_workstation_snapshot(state: State<'_, AppState>) -> Result<WorkstationSnapshot, String> {
    let conn = open_db(&state.db_path)?;
    sync_modules(&conn)?;
    let conversations = conversation_list(&conn)?;
    let active_conversation_id = conversations.first().map(|conversation| conversation.id);
    let messages = match active_conversation_id {
        Some(conversation_id) => message_list(&conn, conversation_id)?,
        None => Vec::new(),
    };

    Ok(WorkstationSnapshot {
        workspace_name: WORKSPACE_NAME.to_string(),
        department_name: DEPARTMENT_NAME.to_string(),
        active_conversation_id,
        conversations,
        messages,
        agents: agent_list(&conn)?,
        tasks: task_list(&conn)?,
        logs: log_list(&conn, 20)?,
        workspace_items: workspace_item_list(&conn, false)?,
        file_items: workspace_item_list(&conn, true)?,
        modules: module_list(&conn)?,
    })
}

#[command]
fn get_workspace_tree(state: State<'_, AppState>) -> Result<Vec<WorkspaceItem>, String> {
    let conn = open_db(&state.db_path)?;
    workspace_item_list(&conn, false)
}

#[command]
fn get_module_registry(state: State<'_, AppState>) -> Result<Vec<ModuleRecord>, String> {
    let conn = open_db(&state.db_path)?;
    sync_modules(&conn)?;
    module_list(&conn)
}

#[command]
fn get_file_items(state: State<'_, AppState>) -> Result<Vec<WorkspaceItem>, String> {
    let conn = open_db(&state.db_path)?;
    workspace_item_list(&conn, true)
}

#[command]
fn get_conversations(state: State<'_, AppState>) -> Result<Vec<Conversation>, String> {
    let conn = open_db(&state.db_path)?;
    conversation_list(&conn)
}

#[command]
fn get_messages(conversation_id: i64, state: State<'_, AppState>) -> Result<Vec<Message>, String> {
    let conn = open_db(&state.db_path)?;
    message_list(&conn, conversation_id)
}

#[command]
fn create_conversation(title: String, state: State<'_, AppState>) -> Result<i64, String> {
    let conn = open_db(&state.db_path)?;
    conn.execute(
        "INSERT INTO conversations (title) VALUES (?1)",
        params![title],
    )
    .map_err(|err| err.to_string())?;
    let id = conn.last_insert_rowid();
    log_event(&conn, "info", "conversation", &format!("Created conversation {id}"))?;
    Ok(id)
}

#[command]
fn add_message(
    conversation_id: i64,
    role: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let conn = open_db(&state.db_path)?;
    conn.execute(
        "INSERT INTO messages (conversation_id, role, content) VALUES (?1, ?2, ?3)",
        params![conversation_id, role, content],
    )
    .map_err(|err| err.to_string())?;
    conn.execute(
        "UPDATE conversations
         SET updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
         WHERE id = ?1",
        params![conversation_id],
    )
    .map_err(|err| err.to_string())?;
    Ok(conn.last_insert_rowid())
}

#[command]
fn get_agents(state: State<'_, AppState>) -> Result<Vec<Agent>, String> {
    let conn = open_db(&state.db_path)?;
    agent_list(&conn)
}

#[command]
fn toggle_agent(agent_id: i64, enabled: bool, state: State<'_, AppState>) -> Result<(), String> {
    let conn = open_db(&state.db_path)?;
    conn.execute(
        "UPDATE agents SET enabled = ?2 WHERE id = ?1",
        params![agent_id, if enabled { 1 } else { 0 }],
    )
    .map_err(|err| err.to_string())?;
    log_event(
        &conn,
        "info",
        "agents",
        &format!("Agent {agent_id} set to {}", if enabled { "enabled" } else { "disabled" }),
    )?;
    Ok(())
}

#[command]
fn dispatch_agent(
    agent_name: String,
    task: String,
    conversation_id: Option<i64>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let conn = open_db(&state.db_path)?;
    let enabled: Option<i64> = conn
        .query_row(
            "SELECT enabled FROM agents WHERE name = ?1",
            params![agent_name],
            |row| row.get(0),
        )
        .optional()
        .map_err(|err| err.to_string())?;

    if enabled != Some(1) {
        return Err(format!("Agent {agent_name} is disabled or unavailable."));
    }

    conn.execute(
        "INSERT INTO tasks (conversation_id, agent_name, title, payload, status)
         VALUES (?1, ?2, ?3, ?4, 'running')",
        params![conversation_id, agent_name, format!("Workstation request: {}", task), json!({ "task": task }).to_string()],
    )
    .map_err(|err| err.to_string())?;

    let task_id = conn.last_insert_rowid();
    let reply = assistant_reply(&conn, &task)?;

    conn.execute(
        "UPDATE tasks
         SET status = 'done',
             result = ?2,
             updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
         WHERE id = ?1",
        params![task_id, reply],
    )
    .map_err(|err| err.to_string())?;
    log_event(&conn, "info", "dispatch", &format!("Dispatched {agent_name} task {task_id}"))?;
    Ok(reply)
}

#[command]
fn get_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    let conn = open_db(&state.db_path)?;
    task_list(&conn)
}

#[command]
fn get_logs(limit: Option<i64>, state: State<'_, AppState>) -> Result<Vec<LogEntry>, String> {
    let conn = open_db(&state.db_path)?;
    log_list(&conn, limit.unwrap_or(50))
}

#[command]
fn open_external_module(module_id: String, state: State<'_, AppState>) -> Result<String, String> {
    let conn = open_db(&state.db_path)?;
    sync_modules(&conn)?;
    let module = conn
        .query_row(
            "SELECT label, status, root_path, launch_path
             FROM modules
             WHERE id = ?1",
            params![module_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                ))
            },
        )
        .optional()
        .map_err(|err| err.to_string())?
        .ok_or_else(|| "Module not found.".to_string())?;

    if module.1 == "missing" || module.1 == "planned" {
        return Err(format!("{} is not installed on this machine.", module.0));
    }

    let target = module
        .3
        .or(module.2)
        .ok_or_else(|| format!("{} does not have a launch target.", module.0))?;
    launch_target(Path::new(&target))?;
    log_event(&conn, "info", "modules", &format!("Opened {}", module.0))?;
    Ok(format!("Opening {}", module.0))
}

#[command]
fn chat(message: String, state: State<'_, AppState>) -> Result<String, String> {
    let conn = open_db(&state.db_path)?;
    assistant_reply(&conn, &message)
}

#[command]
fn agent_status(state: State<'_, AppState>) -> Result<Value, String> {
    let conn = open_db(&state.db_path)?;
    let enabled_agents: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM agents WHERE enabled = 1",
            [],
            |row| row.get(0),
        )
        .map_err(|err| err.to_string())?;
    let installed_modules: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM modules WHERE status = 'installed'",
            [],
            |row| row.get(0),
        )
        .map_err(|err| err.to_string())?;

    Ok(json!({
        "status": "ready",
        "version": "2.0.0",
        "workspace": WORKSPACE_NAME,
        "department": DEPARTMENT_NAME,
        "enabled_agents": enabled_agents,
        "installed_modules": installed_modules
    }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to resolve app data dir");
            fs::create_dir_all(&app_dir).expect("failed to create app data dir");
            let db_path = app_dir.join("zq_master_bridge.db");
            initialize_database(&db_path).expect("failed to initialize database");
            app.manage(AppState { db_path });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_workstation_snapshot,
            get_workspace_tree,
            get_module_registry,
            get_file_items,
            get_conversations,
            get_messages,
            create_conversation,
            add_message,
            get_agents,
            toggle_agent,
            dispatch_agent,
            get_tasks,
            get_logs,
            open_external_module,
            chat,
            agent_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
