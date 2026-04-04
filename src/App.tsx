import { startTransition, useDeferredValue, useEffect, useRef, useState } from "react";
import type { CSSProperties, FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type ViewName = "workspace" | "files" | "analytics" | "operations";

interface Conversation {
  id: number;
  title: string;
  created_at: string;
  updated_at: string;
}

interface Message {
  id: number;
  conversation_id: number;
  role: string;
  content: string;
  created_at: string;
}

interface Agent {
  id: number;
  name: string;
  label: string;
  description: string;
  category: string;
  enabled: boolean;
  module_id: string | null;
}

interface Task {
  id: number;
  conversation_id: number | null;
  agent_name: string;
  title: string;
  status: "pending" | "running" | "done" | "failed";
  result: string | null;
  error: string | null;
  created_at: string;
  updated_at: string;
}

interface LogEntry {
  id: number;
  level: "debug" | "info" | "warn" | "error";
  source: string;
  message: string;
  created_at: string;
}

interface WorkspaceItem {
  id: number;
  item_key: string;
  parent_key: string | null;
  kind: string;
  label: string;
  description: string | null;
  item_path: string | null;
  badge: string | null;
  sort_order: number;
}

interface ModuleRecord {
  id: string;
  label: string;
  description: string;
  kind: string;
  phase: string;
  status: string;
  root_path: string | null;
  launch_path: string | null;
  details: string;
}

interface WorkstationSnapshot {
  workspace_name: string;
  department_name: string;
  active_conversation_id: number | null;
  conversations: Conversation[];
  messages: Message[];
  agents: Agent[];
  tasks: Task[];
  logs: LogEntry[];
  workspace_items: WorkspaceItem[];
  file_items: WorkspaceItem[];
  modules: ModuleRecord[];
}

const NAV_ITEMS: Array<{ id: ViewName; label: string; short: string }> = [
  { id: "workspace", label: "Workstation", short: "WS" },
  { id: "files", label: "Files", short: "FI" },
  { id: "analytics", label: "Analytics", short: "AN" },
  { id: "operations", label: "Operations", short: "OP" },
];

const ACTION_PRESETS = [
  "Upload patient data for weekly intake.",
  "Schedule consultation for General Surgery.",
  "Generate department report for this week.",
  "Show medical guidelines relevant to the active case.",
];

function formatTime(value: string) {
  return new Date(value).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
}

function branch(items: WorkspaceItem[], parentKey: string | null) {
  return items.filter((item) => item.parent_key === parentKey);
}

function TreeBlock({
  items,
  parentKey,
  activeKey,
  onSelect,
}: {
  items: WorkspaceItem[];
  parentKey: string | null;
  activeKey?: string;
  onSelect?: (item: WorkspaceItem) => void;
}) {
  return (
    <>
      {branch(items, parentKey).map((item) => (
        <div key={item.item_key} className="tree-node">
          <button
            className={`tree-item ${activeKey === item.item_key ? "active" : ""} kind-${item.kind}`}
            onClick={() => onSelect?.(item)}
            type="button"
          >
            <span>{item.label}</span>
            {item.badge ? <span className="tree-badge">{item.badge}</span> : null}
          </button>
          {branch(items, item.item_key).length > 0 ? (
            <div className="tree-children">
              <TreeBlock items={items} parentKey={item.item_key} activeKey={activeKey} onSelect={onSelect} />
            </div>
          ) : null}
        </div>
      ))}
    </>
  );
}

function AnalyticsChart() {
  return (
    <svg className="line-chart" viewBox="0 0 420 240" preserveAspectRatio="none" aria-hidden="true">
      <defs>
        <linearGradient id="lineFill" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="rgba(82, 191, 255, 0.38)" />
          <stop offset="100%" stopColor="rgba(82, 191, 255, 0)" />
        </linearGradient>
      </defs>
      <path d="M0 210 L60 176 L100 194 L150 132 L190 92 L225 150 L270 118 L305 144 L345 74 L380 98 L420 62 L420 240 L0 240 Z" fill="url(#lineFill)" />
      <path d="M0 210 L60 176 L100 194 L150 132 L190 92 L225 150 L270 118 L305 144 L345 74 L380 98 L420 62" />
    </svg>
  );
}

export default function App() {
  const [view, setView] = useState<ViewName>("workspace");
  const [snapshot, setSnapshot] = useState<WorkstationSnapshot | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [activeConversationId, setActiveConversationId] = useState<number | null>(null);
  const [activeTreeKey, setActiveTreeKey] = useState("dept.general_surgery");
  const [composer, setComposer] = useState("");
  const [fileSearch, setFileSearch] = useState("");
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const deferredSearch = useDeferredValue(fileSearch);
  const bottomRef = useRef<HTMLDivElement>(null);

  async function loadSnapshot(preferredConversationId?: number | null) {
    const next = await invoke<WorkstationSnapshot>("get_workstation_snapshot");
    startTransition(() => {
      setSnapshot(next);
      const selected =
        preferredConversationId ??
        activeConversationId ??
        next.active_conversation_id ??
        next.conversations[0]?.id ??
        null;
      setActiveConversationId(selected);
      setMessages(selected === next.active_conversation_id ? next.messages : []);
    });
  }

  async function loadMessages(conversationId: number) {
    const next = await invoke<Message[]>("get_messages", { conversation_id: conversationId });
    setMessages(next);
  }

  useEffect(() => {
    loadSnapshot().catch((error) => setNotice(String(error)));
  }, []);

  useEffect(() => {
    if (activeConversationId) {
      loadMessages(activeConversationId).catch((error) => setNotice(String(error)));
    }
  }, [activeConversationId]);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  async function submitMessage(event?: FormEvent) {
    event?.preventDefault();
    if (!composer.trim() || !activeConversationId) return;
    setBusy(true);
    setNotice(null);
    const payload = composer.trim();
    setComposer("");

    try {
      await invoke("add_message", { conversation_id: activeConversationId, role: "user", content: payload });
      const reply = await invoke<string>("dispatch_agent", {
        agent_name: "inam_brain",
        task: payload,
        conversation_id: activeConversationId,
      });
      await invoke("add_message", {
        conversation_id: activeConversationId,
        role: "assistant",
        content: reply,
      });
      await loadSnapshot(activeConversationId);
      await loadMessages(activeConversationId);
    } catch (error) {
      setNotice(String(error));
    } finally {
      setBusy(false);
    }
  }

  async function createConversation() {
    try {
      const id = await invoke<number>("create_conversation", {
        title: `${snapshot?.workspace_name ?? "Saada"} / ${snapshot?.department_name ?? "New Session"}`,
      });
      setView("workspace");
      await loadSnapshot(id);
    } catch (error) {
      setNotice(String(error));
    }
  }

  async function toggleAgent(agent: Agent) {
    try {
      await invoke("toggle_agent", { agent_id: agent.id, enabled: !agent.enabled });
      await loadSnapshot(activeConversationId);
    } catch (error) {
      setNotice(String(error));
    }
  }

  async function openModule(moduleId: string) {
    try {
      const result = await invoke<string>("open_external_module", { module_id: moduleId });
      setNotice(result);
    } catch (error) {
      setNotice(String(error));
    }
  }

  if (!snapshot) {
    return <div className="loading-screen">Preparing workstation shell...</div>;
  }

  const phaseOneModules = snapshot.modules.filter((module) => module.phase === "phase_1");
  const deferredModules = snapshot.modules.filter((module) => module.phase !== "phase_1");
  const filteredFiles = snapshot.file_items.filter((item) =>
    !deferredSearch || item.kind !== "file" || item.label.toLowerCase().includes(deferredSearch.toLowerCase()),
  );

  return (
    <div className="workspace-shell">
      <aside className="nav-rail">
        <div className="brand-mark">ZQ</div>
        {NAV_ITEMS.map((item) => (
          <button
            key={item.id}
            className={`rail-button ${view === item.id ? "active" : ""}`}
            onClick={() => setView(item.id)}
            type="button"
          >
            {item.short}
          </button>
        ))}
      </aside>

      <aside className="sidebar-panel">
        <div className="profile-card">
          <div className="profile-avatar">ZA</div>
          <div>
            <h1>Dr. Al Batinah</h1>
            <p>{snapshot.workspace_name} clinical network</p>
          </div>
        </div>

        <nav className="sidebar-nav">
          {NAV_ITEMS.map((item) => (
            <button
              key={item.id}
              className={`sidebar-link ${view === item.id ? "active" : ""}`}
              onClick={() => setView(item.id)}
              type="button"
            >
              {item.label}
            </button>
          ))}
        </nav>

        <section className="sidebar-section">
          <div className="section-head">
            <span>{snapshot.workspace_name}</span>
            <span className="status-dot" />
          </div>
          <TreeBlock
            items={snapshot.workspace_items}
            parentKey="workspace.saada"
            activeKey={activeTreeKey}
            onSelect={(item) => {
              setActiveTreeKey(item.item_key);
              if (item.kind === "view") {
                setView(item.item_key === "view.home" ? "workspace" : (item.label.toLowerCase() as ViewName));
              }
              if (item.kind === "folder" || item.kind === "file") setView("files");
            }}
          />
        </section>

        <section className="sidebar-section compact">
          <div className="section-head"><span>Phase 1 Modules</span></div>
          {phaseOneModules.map((module) => (
            <button key={module.id} className="module-chip" onClick={() => openModule(module.id)} type="button">
              <span>{module.label}</span>
              <small>{module.status}</small>
            </button>
          ))}
        </section>
      </aside>

      <main className="main-stage">
        <header className="stage-header">
          <div>
            <p className="eyebrow">Advanced Medical AI Workstation V1</p>
            <h2>{snapshot.workspace_name} / {snapshot.department_name}</h2>
          </div>
          <div className="header-actions">
            <button className="ghost-button" onClick={createConversation} type="button">New Session</button>
            <button className="ghost-button" onClick={() => loadSnapshot(activeConversationId)} type="button">Refresh</button>
          </div>
        </header>

        {notice ? <div className="notice-bar">{notice}</div> : null}
        <div className="stage-body">
          {view === "workspace" ? (
            <section className="content-grid">
              <div className="panel thread-panel">
                <div className="message-stack">
                  {messages.map((message) => (
                    <article key={message.id} className={`message-card ${message.role}`}>
                      <div className="message-meta">
                        <strong>{message.role === "assistant" ? "Inam Brain" : message.role === "user" ? "You" : "System"}</strong>
                        <span>{formatTime(message.created_at)}</span>
                      </div>
                      <p>{message.content}</p>
                    </article>
                  ))}
                  <div ref={bottomRef} />
                </div>

                <div className="action-row">
                  {ACTION_PRESETS.map((preset) => (
                    <button key={preset} className="action-pill" onClick={() => setComposer(preset)} type="button">
                      {preset}
                    </button>
                  ))}
                </div>

                <form className="composer" onSubmit={submitMessage}>
                  <button className="composer-icon" onClick={() => setView("files")} type="button">+</button>
                  <input
                    value={composer}
                    onChange={(event) => setComposer(event.target.value)}
                    placeholder="Type a detailed clinical request or routing instruction..."
                    disabled={busy}
                  />
                  <button className="send-button" disabled={busy || !composer.trim()} type="submit">
                    Send
                  </button>
                </form>
              </div>

              <aside className="panel side-panel">
                <div className="summary-box">
                  <h3>Module Access</h3>
                  {phaseOneModules.map((module) => (
                    <div key={module.id} className="summary-row">
                      <div>
                        <strong>{module.label}</strong>
                        <p>{module.description}</p>
                      </div>
                      <button
                        className="mini-button"
                        disabled={module.status !== "installed"}
                        onClick={() => openModule(module.id)}
                        type="button"
                      >
                        Open
                      </button>
                    </div>
                  ))}
                </div>

                <div className="summary-box">
                  <h3>Taskbox and Deferred Systems</h3>
                  {snapshot.modules.map((module) => (
                    <div key={module.id} className="status-row">
                      <span>{module.label}</span>
                      <small>{module.phase.replace("_", " ")} / {module.status}</small>
                    </div>
                  ))}
                </div>
              </aside>
            </section>
          ) : null}
          {view === "files" ? (
            <section className="content-grid files-grid">
              <div className="panel file-panel">
                <div className="panel-head">
                  <h3>Clinical File Workspace</h3>
                  <input
                    className="search-input"
                    value={fileSearch}
                    onChange={(event) => setFileSearch(event.target.value)}
                    placeholder="Search files or folders..."
                  />
                </div>
                <div className="tree-scroll">
                  <TreeBlock
                    items={filteredFiles}
                    parentKey="dept.root"
                    activeKey={activeTreeKey}
                    onSelect={(item) => setActiveTreeKey(item.item_key)}
                  />
                </div>
              </div>

              <div className="panel collaboration-panel">
                <div className="collab-head">
                  <div>
                    <h3>AI Collaboration</h3>
                    <p>Active assistant: Inam Brain</p>
                  </div>
                  <span className="live-tag">Active</span>
                </div>

                <div className="collab-thread">
                  {messages.slice(-3).map((message) => (
                    <article key={message.id} className={`message-card ${message.role}`}>
                      <div className="message-meta">
                        <strong>{message.role === "assistant" ? "Inam Brain" : "Dr. Al Batinah"}</strong>
                        <span>{formatTime(message.created_at)}</span>
                      </div>
                      <p>{message.content}</p>
                    </article>
                  ))}
                </div>

                <div className="file-actions">
                  {["Upload Patient Data", "Schedule Consultation", "Generate Report", "View Medical Guidelines"].map((action) => (
                    <button key={action} className="utility-button" onClick={() => setComposer(`${action}.`)} type="button">
                      {action}
                    </button>
                  ))}
                </div>

                <form className="composer compact" onSubmit={submitMessage}>
                  <input
                    value={composer}
                    onChange={(event) => setComposer(event.target.value)}
                    placeholder="Type a message or query..."
                    disabled={busy}
                  />
                  <button className="send-button" disabled={busy || !composer.trim()} type="submit">
                    Send
                  </button>
                </form>
              </div>
            </section>
          ) : null}

          {view === "analytics" ? (
            <section className="analytics-layout">
              <div className="panel analytics-main">
                <div className="panel-head">
                  <div>
                    <h3>Patient Intake Trends</h3>
                    <p>Last 30 days</p>
                  </div>
                  <span className="insight-tag">AI Powered Insights</span>
                </div>
                <AnalyticsChart />
              </div>

              <div className="analytics-side">
                <div className="panel capacity-panel">
                  {[
                    ["ER Capacity", "85%"],
                    ["ICU Capacity", "70%"],
                    ["Ward Capacity", "45%"],
                  ].map(([label, value]) => (
                    <div key={label} className="capacity-row">
                      <div className="capacity-ring" style={{ "--fill": value } as CSSProperties} />
                      <div>
                        <strong>{label}</strong>
                        <p>{value}</p>
                      </div>
                    </div>
                  ))}
                </div>

                <div className="stats-grid">
                  {[
                    ["Total Patients", "1,245"],
                    ["Recovery Rate", "92%"],
                    ["Average Stay", "4.5 Days"],
                    ["Critical Cases", "15"],
                  ].map(([label, value]) => (
                    <div key={label} className="panel stat-card">
                      <p>{label}</p>
                      <strong>{value}</strong>
                    </div>
                  ))}
                </div>

                <div className="panel demographic-panel">
                  <h3>Patient Demographics</h3>
                  {[
                    ["0-18", 36],
                    ["19-34", 82],
                    ["35-50", 66],
                    ["51-65", 44],
                    ["65+", 28],
                  ].map(([label, width]) => (
                    <div key={label} className="bar-row">
                      <span>{label}</span>
                      <div><i style={{ width: `${width}%` }} /></div>
                    </div>
                  ))}
                </div>
              </div>
            </section>
          ) : null}

          {view === "operations" ? (
            <section className="operations-layout">
              <div className="panel">
                <div className="panel-head">
                  <h3>Module Registry</h3>
                </div>
                <div className="module-grid">
                  {snapshot.modules.map((module) => (
                    <article key={module.id} className="module-card">
                      <div className="module-top">
                        <div>
                          <strong>{module.label}</strong>
                          <p>{module.description}</p>
                        </div>
                        <span className={`module-status ${module.status}`}>{module.status}</span>
                      </div>
                      <small>{module.phase.replace("_", " ")} / {module.kind}</small>
                      <p>{module.details}</p>
                      <button
                        className="mini-button"
                        disabled={module.status !== "installed"}
                        onClick={() => openModule(module.id)}
                        type="button"
                      >
                        Open Module
                      </button>
                    </article>
                  ))}
                </div>
              </div>

              <div className="ops-columns">
                <div className="panel">
                  <div className="panel-head"><h3>Agents and Coordinator Links</h3></div>
                  {snapshot.agents.map((agent) => (
                    <div key={agent.id} className="agent-row">
                      <div>
                        <strong>{agent.label}</strong>
                        <p>{agent.description}</p>
                      </div>
                      <button className="mini-button" onClick={() => toggleAgent(agent)} type="button">
                        {agent.enabled ? "Disable" : "Enable"}
                      </button>
                    </div>
                  ))}
                </div>

                <div className="panel">
                  <div className="panel-head"><h3>Taskbox Queue</h3></div>
                  <div className="table-list">
                    {snapshot.tasks.map((task) => (
                      <div key={task.id} className="table-row">
                        <div>
                          <strong>{task.title}</strong>
                          <p>{task.agent_name}</p>
                        </div>
                        <span className={`module-status ${task.status}`}>{task.status}</span>
                      </div>
                    ))}
                  </div>
                </div>

                <div className="panel">
                  <div className="panel-head"><h3>Coordinator Logs</h3></div>
                  <div className="table-list">
                    {snapshot.logs.map((log) => (
                      <div key={log.id} className="table-row">
                        <div>
                          <strong>{log.source}</strong>
                          <p>{log.message}</p>
                        </div>
                        <span className={`module-status ${log.level}`}>{log.level}</span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>

              <div className="panel deferred-panel">
                <div className="panel-head"><h3>Deferred Modules</h3></div>
                {deferredModules.map((module) => (
                  <div key={module.id} className="status-row">
                    <span>{module.label}</span>
                    <small>{module.status} / {module.details}</small>
                  </div>
                ))}
              </div>
            </section>
          ) : null}
        </div>
      </main>
    </div>
  );
}
