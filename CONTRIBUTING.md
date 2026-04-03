# Contributing to ZQ Master Bridge

Thank you for taking the time to contribute! 🎉  
Please read the sections below before opening an issue or submitting a pull request.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [How to Report a Bug](#how-to-report-a-bug)
3. [How to Request a Feature](#how-to-request-a-feature)
4. [Development Setup](#development-setup)
5. [Branching Strategy](#branching-strategy)
6. [Coding Standards](#coding-standards)
7. [Commit Messages](#commit-messages)
8. [Pull Request Process](#pull-request-process)

---

## Code of Conduct

Be respectful and inclusive. Harassment of any kind will not be tolerated.

---

## How to Report a Bug

Use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.md). Include steps to reproduce, expected vs actual behaviour, and your environment details.

---

## How to Request a Feature

Use the [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.md) and describe the problem you're trying to solve.

---

## Development Setup

### Prerequisites

| Tool | Minimum Version |
| ---- | --------------- |
| Node.js | 20 |
| Rust | 1.77 (stable) |
| Python | 3.11 |
| SQLite | 3.35 |

### Quick Start

```bash
# 1. Clone
git clone https://github.com/zubinqayam/zq-master-bridge.git
cd zq-master-bridge

# 2. Copy env template
cp .env.example .env
# Edit .env with your values — never commit this file!

# 3. Install Node dependencies
npm install

# 4. Apply SQLite schema
sqlite3 zq.db < database/schema.sql

# 5. Start dev server (Vite + Tauri)
npm run tauri:dev

# 6. (Optional) Start Python agent router
python -m agents.core.router
```

Windows is the only supported packaged release target. Android APKs are supported as a local build workflow when the Android SDK is installed.

### Local Release Validation

```bash
# Frontend production build
npm run build

# Rust backend validation
cd src-tauri && cargo check

# Python sidecar bundle
npm run sidecar:build

# Windows installer build
npm run tauri:build -- --bundles nsis
```

### Local Android Validation

```bash
# One-time setup on a machine with Android SDK + Java
npm run android:init

# Build local Android APKs
npm run android:build
```

---

## Branching Strategy

| Branch | Purpose |
| ------ | ------- |
| `main` | Stable, production-ready |
| `version/Enhancement-lab-*` | V3 / next-gen feature work |
| `fix/<short-description>` | Bug fixes |
| `feat/<short-description>` | New features |

Always branch off `main` and open a PR back to `main`.

---

## Coding Standards

### TypeScript / React
- Follow the existing code style (no additional ESLint rules needed for PRs).
- Prefer functional components with hooks.
- No `any` types unless unavoidable (and add a comment explaining why).

### Rust
- Run `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings` and fix all warnings.

### Python
- Run `ruff check agents/` before committing.
- Type-annotate all public functions.

---

## Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add streaming response support
fix: prevent crash on empty message
docs: update README build instructions
chore: bump tauri to 2.1.0
```

---

## Pull Request Process

1. Fork the repository and create a feature branch.
2. Make your changes and ensure all checks pass locally.
3. Open a PR against `main` using the [PR template](.github/PULL_REQUEST_TEMPLATE.md).
4. At least one maintainer review is required before merging.
5. Squash-merge is preferred for feature branches.

---

Thank you for contributing! 🚀
