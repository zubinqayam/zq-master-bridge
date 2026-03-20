# ZQ Master Bridge

[![Build & Release](https://github.com/zubinqayam/zq-master-bridge/actions/workflows/release.yml/badge.svg)](https://github.com/zubinqayam/zq-master-bridge/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

> **Control Room V2** — A local-first, privacy-preserving AI assistant with a ChatGPT-style UI, Rust/Tauri desktop backend, Python agent sidecar, and a V3 roadmap for 500+ parallel agents.

---

## ✨ Features

| Layer | Technology | Notes |
|-------|-----------|-------|
| UI | React 19 + TypeScript + Vite | ChatGPT-style chat interface |
| Desktop | Rust + Tauri 2 | Native Windows / macOS / Linux |
| Agents | Python 3.11+ asyncio | Pluggable agent router |
| Database | SQLite (WAL mode) | Conversations, tasks, audit log |
| CI/CD | GitHub Actions | Build + release for all platforms |

---

## 🚀 Quick Start

### Prerequisites

| Tool | Version |
|------|---------|
| [Node.js](https://nodejs.org) | ≥ 20 |
| [Rust](https://rustup.rs) | stable (≥ 1.77) |
| [Python](https://python.org) | ≥ 3.11 |

> **Linux users:** install WebKit system libraries first:
> ```bash
> sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential libssl-dev
> ```

### Steps

```bash
# 1. Clone
git clone https://github.com/zubinqayam/zq-master-bridge.git
cd zq-master-bridge

# 2. Environment
cp .env.example .env
# Edit .env with your values (API keys, etc.)

# 3. Install Node dependencies
npm install

# 4. Initialise the database
sqlite3 zq.db < database/schema.sql

# 5. Start the desktop app (Vite dev server + Tauri)
npm run tauri:dev

# 6. (Optional) Start Python agent sidecar in a separate terminal
python -m agents.core.router
```

---

## 🏗️ Project Structure

```
zq-master-bridge/
├── src/                      # React 19 UI
│   ├── App.tsx               # Main chat component
│   ├── main.tsx              # React entry point
│   └── index.css             # Global styles
├── src-tauri/                # Rust Tauri backend
│   ├── src/main.rs           # Tauri commands (chat, agent_status)
│   ├── tauri.conf.json       # Tauri configuration
│   └── Cargo.toml            # Rust dependencies
├── agents/                   # Python agent sidecar
│   └── core/router.py        # Async agent router
├── database/
│   └── schema.sql            # SQLite schema
├── docs/
│   └── v3/ARCHITECTURE.md    # V3 roadmap (500+ agents)
├── .github/
│   ├── workflows/release.yml # CI/CD — build + release
│   ├── SECURITY.md           # Vulnerability reporting
│   ├── dependabot.yml        # Automated dependency updates
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── ISSUE_TEMPLATE/       # Bug report + feature request
├── .vscode/                  # VSCode tasks + extensions
├── .env.example              # Environment variable template
├── CONTRIBUTING.md           # Contributor guide
├── CHANGELOG.md              # Release notes
└── LICENSE                   # Apache 2.0
```

---

## 🔨 Build for Production

```bash
# Build the Tauri desktop app (outputs to src-tauri/target/release/bundle/)
npm run tauri:build
```

Platform bundles generated:

| Platform | Output |
|----------|--------|
| Windows | `.exe` (NSIS installer) |
| macOS | `.app` / `.dmg` |
| Linux | `.AppImage` / `.deb` |

---

## 🤖 V3 Roadmap (500+ Agents)

See [`docs/v3/ARCHITECTURE.md`](docs/v3/ARCHITECTURE.md) for the full V3 blueprint.

V3 development happens on the `version/Enhancement-lab-` branch.

---

## 🔒 Security

See [`.github/SECURITY.md`](.github/SECURITY.md) for the vulnerability reporting policy.

---

## 🤝 Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the development guide, coding standards, and PR process.

---

## 📄 License

[Apache 2.0](LICENSE) © 2026 Zubin Qayam
