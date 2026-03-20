# V3 Enhancement Lab — Scaffolding Kickoff

> **Branch:** `version/Enhancement-lab-`  
> **Based on:** `main` (Control Room V2)  
> **Created:** 2026-03-20

---

## Purpose

This branch is the working space for all **V3 next-generation** development.  
It is intentionally kept separate from `main` so that experimental work does not
destabilise the stable V2 release.

---

## What Lives Here

| Path | Description |
|------|-------------|
| `docs/v3/ARCHITECTURE.md` | Full V3 architecture blueprint (already on main) |
| `docs/v3/SCAFFOLDING.md` | **This file** — V3 development kickoff & task tracker |
| `src/` | V3 UI enhancements will land here |
| `agents/` | New agent types, pool manager, A2A protocol |
| `src-tauri/` | Streaming bridge, gRPC bindings |

---

## Immediate Next Steps

- [ ] **Agent Pool Manager MVP** — implement dynamic registration + load balancing in `agents/core/pool.py`
- [ ] **WebSocket streaming** — replace Tauri `invoke` echo with a streaming SSE bridge
- [ ] **MCP tool manifest** — add `agents/core/mcp.py` exposing tool schemas
- [ ] **Vector store integration** — add `aiosqlite` + `sqlite-vss` for semantic memory
- [ ] **Dashboard UI** — add `src/components/AgentDashboard.tsx` with live metrics
- [ ] **Docker dev environment** — `docker-compose.yml` for local multi-agent testing
- [ ] **500-agent stress test** — `tests/stress/test_pool.py` with `pytest-asyncio`

---

## Branching Conventions (V3)

Create feature branches off `version/Enhancement-lab-`:

```
version/Enhancement-lab-/<feature>
# e.g.
version/Enhancement-lab-/agent-pool-manager
version/Enhancement-lab-/streaming-ui
version/Enhancement-lab-/mcp-tools
```

Merge feature branches back to `version/Enhancement-lab-` via PR.  
When a milestone is stable, open a PR from `version/Enhancement-lab-` → `main`.

---

## Reference

- Full architecture: [`docs/v3/ARCHITECTURE.md`](ARCHITECTURE.md)
- Contributing guide: [`CONTRIBUTING.md`](../../CONTRIBUTING.md)
- Roadmap milestones: see `docs/v3/ARCHITECTURE.md#roadmap-milestones`
