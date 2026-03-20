# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 2.x     | ✅ Actively supported |
| < 2.0   | ❌ No longer supported |

## Reporting a Vulnerability

**Please do NOT open a public GitHub issue for security vulnerabilities.**

Report security issues by emailing **zubin.qayam@outlook.com** with:

1. A description of the vulnerability and its potential impact.
2. Steps to reproduce the issue.
3. Any proof-of-concept code (if applicable).
4. Your suggested fix (optional but appreciated).

### What to Expect

- **Acknowledgement** within 48 hours of your report.
- **Status update** within 7 days (confirmed, investigating, or rejected).
- **Patch release** within 30 days for confirmed critical/high vulnerabilities.
- Credit in the `CHANGELOG.md` (with your permission).

### Scope

The following are **in scope**:

- Authentication or privilege escalation bugs.
- Remote code execution vulnerabilities.
- Data leakage / exposure of secrets.
- Tauri IPC command injection.
- SQL injection in the SQLite schema or queries.

The following are **out of scope**:

- Denial-of-service via resource exhaustion (unless trivially exploitable).
- Issues already known and tracked in GitHub Issues.
- Issues in third-party dependencies (report upstream; we will coordinate).

Thank you for helping keep ZQ Master Bridge secure! 🔒
