import React, { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Message {
  id: number;
  role: "user" | "assistant" | "system";
  content: string;
  created_at: string;
}

interface Conversation {
  id: number;
  title: string;
  created_at: string;
}

interface Agent {
  id: number;
  name: string;
  description: string | null;
  enabled: number;
}

interface Task {
  id: number;
  agent_name: string;
  status: "pending" | "running" | "done" | "failed";
  created_at: string;
  result: string | null;
  error: string | null;
}

interface Log {
  id: number;
  level: "debug" | "info" | "warn" | "error";
  source: string;
  message: string;
  created_at: string;
}

type TabView = "chat" | "agents" | "tasks" | "logs" | "settings";

export default function App() {
  const [view, setView] = useState<TabView>("chat");
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [currentConvId, setCurrentConvId] = useState<number | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState("");
  const [loading, setLoading] = useState(false);
  const [agents, setAgents] = useState<Agent[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [logs, setLogs] = useState<Log[]>([]);
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    loadConversations();
    loadAgents();
    loadTasks();
    loadLogs();
  }, []);

  useEffect(() => {
    if (currentConvId) {
      loadMessages(currentConvId);
    }
  }, [currentConvId]);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  async function loadConversations() {
    try {
      const convs = await invoke<Conversation[]>("get_conversations");
      setConversations(convs);
      if (convs.length > 0 && !currentConvId) {
        setCurrentConvId(convs[0].id);
      }
    } catch (err) {
      console.error("Failed to load conversations:", err);
    }
  }

  async function loadMessages(conversationId: number) {
    try {
      const msgs = await invoke<Message[]>("get_messages", { conversationId });
      setMessages(msgs);
    } catch (err) {
      console.error("Failed to load messages:", err);
    }
  }

  async function loadAgents() {
    try {
      const agentList = await invoke<Agent[]>("get_agents");
      setAgents(agentList);
    } catch (err) {
      console.error("Failed to load agents:", err);
    }
  }

  async function loadTasks() {
    try {
      const taskList = await invoke<Task[]>("get_tasks");
      setTasks(taskList);
    } catch (err) {
      console.error("Failed to load tasks:", err);
    }
  }

  async function loadLogs() {
    try {
      const logList = await invoke<Log[]>("get_logs", { limit: 100 });
      setLogs(logList);
    } catch (err) {
      console.error("Failed to load logs:", err);
    }
  }

  async function createNewConversation() {
    try {
      const newConvId = await invoke<number>("create_conversation", {
        title: "New Conversation",
      });
      await loadConversations();
      setCurrentConvId(newConvId);
      setMessages([]);
    } catch (err) {
      console.error("Failed to create conversation:", err);
    }
  }

  async function sendMessage() {
    if (!input.trim() || !currentConvId) return;

    const userMessage = input.trim();
    setInput("");
    setLoading(true);

    try {
      // Add user message
      await invoke("add_message", {
        conversationId: currentConvId,
        role: "user",
        content: userMessage,
      });

      // Route task to echo agent (placeholder - replace with real routing logic)
      const response = await invoke<string>("dispatch_agent", {
        agentName: "echo",
        task: userMessage,
      });

      // Add assistant message
      await invoke("add_message", {
        conversationId: currentConvId,
        role: "assistant",
        content: response,
      });

      await loadMessages(currentConvId);
      await loadTasks();
    } catch (err) {
      console.error("Failed to send message:", err);
    } finally {
      setLoading(false);
    }
  }

  async function toggleAgent(agentId: number, enabled: boolean) {
    try {
      await invoke("toggle_agent", { agentId, enabled: enabled ? 1 : 0 });
      await loadAgents();
    } catch (err) {
      console.error("Failed to toggle agent:", err);
    }
  }

  return (
    <div className="app-container">
      {/* Sidebar */}
      <div className="sidebar">
        <div className="sidebar-header">
          <h2>ZQ Master Bridge</h2>
          <p className="version">v2.0.0</p>
        </div>
        <nav className="sidebar-nav">
          <button
            className={view === "chat" ? "active" : ""}
            onClick={() => setView("chat")}
          >
            💬 Chat
          </button>
          <button
            className={view === "agents" ? "active" : ""}
            onClick={() => setView("agents")}
          >
            🤖 Agents
          </button>
          <button
            className={view === "tasks" ? "active" : ""}
            onClick={() => setView("tasks")}
          >
            📋 Tasks
          </button>
          <button
            className={view === "logs" ? "active" : ""}
            onClick={() => setView("logs")}
          >
            📊 Audit Log
          </button>
          <button
            className={view === "settings" ? "active" : ""}
            onClick={() => setView("settings")}
          >
            ⚙️ Settings
          </button>
        </nav>

        {view === "chat" && (
          <div className="conversations-panel">
            <button className="new-conversation-btn" onClick={createNewConversation}>
              + New Conversation
            </button>
            <div className="conversation-list">
              {conversations.map((conv) => (
                <div
                  key={conv.id}
                  className={`conversation-item ${
                    conv.id === currentConvId ? "active" : ""
                  }`}
                  onClick={() => setCurrentConvId(conv.id)}
                >
                  <div className="conversation-title">{conv.title}</div>
                  <div className="conversation-date">
                    {new Date(conv.created_at).toLocaleDateString()}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Main Content */}
      <div className="main-content">
        {view === "chat" && (
          <div className="chat-view">
            <div className="messages-container">
              {messages.map((msg) => (
                <div key={msg.id} className={`message message-${msg.role}`}>
                  <div className="message-role">
                    {msg.role === "user" ? "You" : "Assistant"}
                  </div>
                  <div className="message-content">{msg.content}</div>
                  <div className="message-time">
                    {new Date(msg.created_at).toLocaleTimeString()}
                  </div>
                </div>
              ))}
              <div ref={bottomRef} />
            </div>
            <div className="input-container">
              <input
                type="text"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyPress={(e) => e.key === "Enter" && sendMessage()}
                placeholder="Type your message..."
                disabled={loading || !currentConvId}
              />
              <button
                onClick={sendMessage}
                disabled={loading || !input.trim() || !currentConvId}
              >
                {loading ? "⏳" : "➤"}
              </button>
            </div>
          </div>
        )}

        {view === "agents" && (
          <div className="agents-view">
            <h2>Agent Manager</h2>
            <div className="agents-grid">
              {agents.map((agent) => (
                <div key={agent.id} className="agent-card">
                  <div className="agent-header">
                    <h3>{agent.name}</h3>
                    <label className="toggle-switch">
                      <input
                        type="checkbox"
                        checked={agent.enabled === 1}
                        onChange={(e) => toggleAgent(agent.id, e.target.checked)}
                      />
                      <span className="slider"></span>
                    </label>
                  </div>
                  <p className="agent-description">
                    {agent.description || "No description"}
                  </p>
                  <div className="agent-status">
                    Status: {agent.enabled ? "✅ Enabled" : "❌ Disabled"}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {view === "tasks" && (
          <div className="tasks-view">
            <h2>Task Queue</h2>
            <button onClick={loadTasks} className="refresh-btn">
              🔄 Refresh
            </button>
            <table className="tasks-table">
              <thead>
                <tr>
                  <th>ID</th>
                  <th>Agent</th>
                  <th>Status</th>
                  <th>Created</th>
                  <th>Result</th>
                </tr>
              </thead>
              <tbody>
                {tasks.map((task) => (
                  <tr key={task.id} className={`task-status-${task.status}`}>
                    <td>{task.id}</td>
                    <td>{task.agent_name}</td>
                    <td>
                      <span className={`status-badge status-${task.status}`}>
                        {task.status}
                      </span>
                    </td>
                    <td>{new Date(task.created_at).toLocaleString()}</td>
                    <td className="task-result">
                      {task.error || task.result || "—"}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        {view === "logs" && (
          <div className="logs-view">
            <h2>Audit Log</h2>
            <button onClick={loadLogs} className="refresh-btn">
              🔄 Refresh
            </button>
            <table className="logs-table">
              <thead>
                <tr>
                  <th>Time</th>
                  <th>Level</th>
                  <th>Source</th>
                  <th>Message</th>
                </tr>
              </thead>
              <tbody>
                {logs.map((log) => (
                  <tr key={log.id} className={`log-level-${log.level}`}>
                    <td>{new Date(log.created_at).toLocaleString()}</td>
                    <td>
                      <span className={`log-badge log-${log.level}`}>
                        {log.level.toUpperCase()}
                      </span>
                    </td>
                    <td>{log.source}</td>
                    <td>{log.message}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        {view === "settings" && (
          <div className="settings-view">
            <h2>Settings</h2>
            <div className="settings-section">
              <h3>About</h3>
              <p>
                <strong>ZQ Master Bridge</strong> v2.0.0
              </p>
              <p>Local-first AI assistant with multi-agent orchestration</p>
              <p>
                <a
                  href="https://github.com/zubinqayam/zq-master-bridge"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  View on GitHub
                </a>
              </p>
            </div>
            <div className="settings-section">
              <h3>Database</h3>
              <p>SQLite WAL mode enabled</p>
              <p>Agents: {agents.length}</p>
              <p>Conversations: {conversations.length}</p>
              <p>Tasks: {tasks.length}</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
