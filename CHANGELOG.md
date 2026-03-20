# Changelog

All notable changes to ZQ Master Bridge are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- React 19 ChatGPT-style UI (`src/App.tsx`)
- Rust Tauri 2 backend with `chat` and `agent_status` commands (`src-tauri/src/main.rs`)
- Python async agent router with pluggable handler registry (`agents/core/router.py`)
- SQLite database schema with conversations, messages, agents, tasks, and logs tables
- CI/CD GitHub Actions workflow for Windows, macOS, Linux builds and release packaging
- VSCode workspace tasks and extension recommendations
- Apache 2.0 LICENSE (full text)
- `.env.example` for safe environment variable templating
- `.github/SECURITY.md` for vulnerability reporting
- `.github/dependabot.yml` for automated dependency updates (npm, Cargo, pip, Actions)
- `CONTRIBUTING.md` with development setup and coding standards
- GitHub issue templates (bug report, feature request)
- GitHub pull request template
- `docs/v3/ARCHITECTURE.md` — V3 / 500-agent architecture blueprint
- `version/Enhancement-lab-` branch created for V3 enhancement work

---

## [2.0.0] — 2026-03-20

### Added
- Initial repository setup with Apache 2.0 license.

[Unreleased]: https://github.com/zubinqayam/zq-master-bridge/compare/v2.0.0...HEAD
[2.0.0]: https://github.com/zubinqayam/zq-master-bridge/releases/tag/v2.0.0
