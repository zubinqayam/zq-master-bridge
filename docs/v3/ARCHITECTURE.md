# ZQ Master Bridge — V3 Architecture

> **Status:** Planning Phase  
> **Branch:** `version/Enhancement-lab-`  
> **Target:** 500+ parallel agents, swarm orchestration, Dynamo OS integration

---

## Overview

ZQ Master Bridge V3 extends the V2 Control Room with a massively parallel, event-driven agent swarm capable of running 500+ concurrent agents across distributed workspaces.

---

## Goals

| # | Goal | Priority |
|---|------|----------|
| 1 | 500+ parallel agents with sub-100 ms dispatch latency | P0 |
| 2 | Swarm + hierarchical routing (MCP + A2A protocols) | P0 |
| 3 | Dynamo OS workspace integration | P1 |
| 4 | Real-time streaming responses (SSE / WebSocket) | P1 |
| 5 | Agent health monitoring dashboard | P2 |
| 6 | Auto-scaling container orchestration (Docker / Podman) | P2 |
| 7 | Persistent memory and long-context support | P2 |
| 8 | Cross-platform mobile (Android / iOS via Tauri) | P3 |

---

## Architecture Layers

```
┌─────────────────────────────────────────────────────┐
│                  React 19 UI (V3)                   │
│   Streaming chat · Agent dashboard · Live metrics   │
└────────────────────┬────────────────────────────────┘
                     │  Tauri IPC / WebSocket
┌────────────────────▼────────────────────────────────┐
│             Rust Tauri Core (V3)                    │
│   Command router · Stream bridge · Auth layer       │
└────────────────────┬────────────────────────────────┘
                     │  gRPC / HTTP/2
┌────────────────────▼────────────────────────────────┐
│         Python Agent Orchestrator (V3)              │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │              Agent Pool Manager             │   │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ …    │   │
│  │  │ Agent 1 │ │ Agent 2 │ │ Agent N │ 500+  │   │
│  │  └────┬────┘ └────┬────┘ └────┬────┘      │   │
│  └───────┴───────────┴───────────┴────────────┘   │
│                     │                               │
│         Event Bus (Redis Streams / SQLite WAL)      │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────┐
│              Storage Layer (V3)                     │
│  SQLite (local) · Vector DB (RAG) · Redis cache     │
└─────────────────────────────────────────────────────┘
```

---

## Agent Pool Manager

The V3 Agent Pool Manager replaces the simple V2 registry with a dynamic system:

- **Dynamic registration** — agents self-register via discovery endpoint.
- **Load balancing** — round-robin, least-connections, and capability-based routing.
- **Circuit breaker** — failed agents are quarantined and restarted.
- **Observability** — Prometheus metrics emitted per agent.

### Configuration (planned)

```toml
[pool]
max_agents = 512
min_agents = 4
scale_up_threshold = 0.8   # CPU/queue utilisation
scale_down_threshold = 0.2
heartbeat_interval_s = 5
```

---

## MCP + A2A Protocols

V3 adopts the **Model Context Protocol (MCP)** for tool calling and **Agent-to-Agent (A2A)** protocol for inter-agent communication:

- Agents expose MCP-compatible tool manifests.
- The router dispatches tool calls using A2A message envelopes.
- Session context is maintained in the SQLite `conversations` table and optionally in a vector store for semantic retrieval.

---

## Dynamo OS Integration

Dynamo OS provides isolated, reproducible execution environments for each agent:

- Each agent runs in a Dynamo workspace sandbox.
- File I/O is scoped to the workspace; no cross-agent filesystem access.
- Network access is policy-controlled (allow-list per agent type).

---

## Streaming Responses

V2 returns full responses synchronously. V3 introduces end-to-end streaming:

1. Frontend opens a WebSocket to the Tauri backend.
2. Tauri proxies a Server-Sent Events (SSE) stream from the Python orchestrator.
3. The orchestrator streams LLM tokens as they are generated.
4. The UI renders tokens incrementally (React Suspense + streaming).

---

## Roadmap Milestones

| Milestone | Target | Status |
|-----------|--------|--------|
| M1 — Agent Pool MVP (16 agents) | Q2 2026 | 🔲 Planned |
| M2 — MCP tool calling integration | Q3 2026 | 🔲 Planned |
| M3 — Streaming responses E2E | Q3 2026 | 🔲 Planned |
| M4 — 500-agent stress test | Q4 2026 | 🔲 Planned |
| M5 — Dynamo OS workspace sandbox | Q4 2026 | 🔲 Planned |
| M6 — Mobile (Android/iOS) | Q1 2027 | 🔲 Planned |

---

## Contributing to V3

All V3 work happens on the `version/Enhancement-lab-` branch.  
See [CONTRIBUTING.md](../../CONTRIBUTING.md) for setup instructions.

Open a Feature Request issue tagged `v3` before starting large changes.
