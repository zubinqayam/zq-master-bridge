-- ZQ Master Bridge workstation schema
-- Applied by src-tauri/src/lib.rs when PRAGMA user_version < 2

PRAGMA foreign_keys = OFF;

DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS tasks;
DROP TABLE IF EXISTS logs;
DROP TABLE IF EXISTS agents;
DROP TABLE IF EXISTS workspace_items;
DROP TABLE IF EXISTS modules;
DROP TABLE IF EXISTS conversations;

PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;

CREATE TABLE conversations (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    title      TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE messages (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    role            TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content         TEXT NOT NULL,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_messages_conversation
    ON messages (conversation_id, created_at);

CREATE TABLE agents (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE,
    label       TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    category    TEXT NOT NULL DEFAULT 'assistant',
    enabled     INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    module_id   TEXT,
    created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE tasks (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation_id INTEGER REFERENCES conversations(id) ON DELETE SET NULL,
    agent_name      TEXT NOT NULL,
    title           TEXT NOT NULL,
    payload         TEXT NOT NULL DEFAULT '{}',
    status          TEXT NOT NULL DEFAULT 'pending'
                        CHECK (status IN ('pending', 'running', 'done', 'failed')),
    result          TEXT,
    error           TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_tasks_status
    ON tasks (status, created_at);

CREATE TABLE logs (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    level      TEXT NOT NULL DEFAULT 'info'
                   CHECK (level IN ('debug', 'info', 'warn', 'error')),
    source     TEXT NOT NULL,
    message    TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE workspace_items (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    item_key    TEXT NOT NULL UNIQUE,
    parent_key  TEXT,
    kind        TEXT NOT NULL,
    label       TEXT NOT NULL,
    description TEXT,
    item_path   TEXT,
    badge       TEXT,
    sort_order  INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_workspace_parent_key
    ON workspace_items (parent_key, sort_order, label);

CREATE TABLE modules (
    id          TEXT PRIMARY KEY,
    label       TEXT NOT NULL,
    description TEXT NOT NULL,
    kind        TEXT NOT NULL,
    phase       TEXT NOT NULL,
    status      TEXT NOT NULL,
    root_path   TEXT,
    launch_path TEXT,
    details     TEXT NOT NULL DEFAULT ''
);
