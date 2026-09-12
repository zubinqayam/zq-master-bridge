# ZQ Master Bridge

## Workflow Systems and Digital Productivity Prototype

ZQ Master Bridge is a local-first desktop software prototype exploring workflow coordination, automation concepts, data dashboards, and AI-assisted productivity.

## Project Overview

The project combines a web interface with a desktop shell and local data capabilities. It is intended for experimentation, architecture research, and public-safe project documentation.

## What It Contains

- React views for workflow and project information
- A Tauri desktop shell for local experimentation
- Rust integration points for desktop capabilities
- SQLite-backed local state and schema files
- Optional Python sidecar tooling
- Documentation and scripts for local development and packaging experiments

## Current Status

**Prototype** — active technical exploration and architecture documentation.

This repository may contain incomplete features, experimental integrations, and local-only assumptions. It is not presented as production-ready software.

## Architecture

The project includes a React and TypeScript frontend, a Rust and Tauri desktop layer, SQLite persistence, and an optional Python sidecar. The main code areas are `src/`, `src-tauri/`, `agents/`, `database/`, and `docs/`.

## Tech Stack

Verified from the repository and package manifest:

- React 19
- TypeScript
- Vite
- Tauri 2
- Rust
- SQLite
- Python sidecar tooling
- GitHub Actions workflows

## Local Development

Prerequisites listed by the repository include Node.js, Rust, and Python. Install JavaScript dependencies first:

```bash
npm install
```

Start the web development server:

```bash
npm run dev
```

Start the desktop development shell when the Tauri toolchain is configured:

```bash
npm run tauri:dev
```

Optional documented commands include:

```bash
npm run sidecar:build
npm run tauri:build
npm run android:init
npm run android:build
```

Verify local toolchain requirements before running packaging or mobile commands.

## Verification

- Build status: verify `npm run build` locally before describing a release as usable.
- Test status: review the repository's available tests and workflows before making test claims.
- CI status: workflow files are present; current CI results should be checked on GitHub.
- Known limitations: local integrations, desktop packaging, sidecar tooling, and mobile commands may require platform-specific setup.

## Public Safety and Privacy

This repository is intended for public-safe project documentation only. Do not include restricted organizational information, employer-specific material, private personal information, business-sensitive material, access credentials, private messages, or non-public process details.

Use synthetic or publicly available examples. Do not claim production readiness, customer deployment, commercial use, formal validation, or enterprise readiness unless independently verified.

## Roadmap

- Clarify the public workflow and data model documentation.
- Add reproducible examples using safe, non-sensitive data.
- Record build and verification results for supported platforms.
- Separate experimental integrations from stable documented interfaces.

## Security

See [`.github/SECURITY.md`](.github/SECURITY.md) for the vulnerability reporting policy.

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for contribution guidance and repository standards.

## License

[Apache 2.0](LICENSE) © 2026 Zubin Qayam
