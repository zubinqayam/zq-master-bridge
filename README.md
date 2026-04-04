# ZQ Master Bridge

[![Build & Release](https://github.com/zubinqayam/zq-master-bridge/actions/workflows/release.yml/badge.svg)](https://github.com/zubinqayam/zq-master-bridge/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

> **Phase 1 workstation shell** — A local-first Tauri application for the Saada / General Surgery workstation with a sample-driven clinical UI, SQLite-backed local state, and first-class integration points for ZQ Coordinator and INNM Taskbox.

---

## ✨ Features

| Layer | Technology | Notes |
| ------- | ----------- | ------- |
| UI | React 19 + TypeScript + Vite | Workstation shell with Home, Files, Analytics, and Operations surfaces |
| Desktop | Rust + Tauri 2 | Windows desktop release target plus Android local builds |
| Data | SQLite (WAL mode) | Conversations, tasks, logs, workspace tree, and module registry |
| Integrations | Local path discovery | ZQ Coordinator and INNM Taskbox phase 1 launch points |
| CI/CD | GitHub Actions | Push/PR checks plus Windows release builds |

---

## 🚀 Quick Start

### Prerequisites

| Tool | Version |
| ------ | --------- |
| [Node.js](https://nodejs.org) | ≥ 20 |
| [Rust](https://rustup.rs) | stable (≥ 1.77) |
| [Python](https://python.org) | ≥ 3.11 |

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

# 4. Start the desktop app (Vite dev server + Tauri)
npm run tauri:dev

# 5. (Optional) Start Python agent sidecar in a separate terminal
python -m agents.core.router
```

> Packaged release support is Windows-only. Local Android APK builds are supported through the Tauri Android toolchain after Android SDK setup.

---

## 🏗️ Project Structure

```
zq-master-bridge/
├── src/                      # React 19 UI
│   ├── App.tsx               # Workstation shell and routed views
│   ├── main.tsx              # React entry point
│   └── index.css             # Global styles
├── src-tauri/                # Rust Tauri backend
│   ├── src/main.rs           # Desktop entry point
│   ├── src/lib.rs            # SQLite bootstrapping + workstation IPC
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
# Build the packaged Python sidecar into src-tauri/resources/
npm run sidecar:build

# Build the Windows NSIS installer
npm run tauri:build -- --bundles nsis
```

Windows release artifact:

| Platform | Output |
|----------|--------|
| Windows | `.exe` (NSIS installer) |

## Workstation Scope

- Phase 1 UI follows the workstation sample direction rather than the earlier tabbed placeholder shell.
- The local database now stores workspace structure, module registry entries, conversations, messages, tasks, and logs.
- `C:\Users\zubin\ZQ_COORDINATOR` and `C:\ZQ_Taskbox` are detected at runtime and exposed in the Operations and Workstation views.
- Feedback Bot, INNM / Woods, and Keyhole are surfaced as deferred module slots so they are visible even when not yet integrated.

## 📱 Local Android APK Build

```bash
# Initialize the Android target once on a machine with Android SDK + Java configured
npm run android:init

# Build installable debug APKs for local device/emulator testing
npm run android:build

# Build release APKs when you need a release artifact
npm run android:build:release
```

Android APK builds are for local/mobile validation and are not part of the official GitHub release contract.

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
