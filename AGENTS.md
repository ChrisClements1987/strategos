# AGENTS.md - Project Strategos

## Project Status
**Current Phase:** Pre-development planning. Code repository not yet initialized.
**Next Step:** Run repository setup commands from `programme-management/strategos-kick-off-and-tdd.md`

## Planned Architecture (v0.1+)
- **Stack:** Tauri (Rust backend) + Svelte (frontend) + SQLite (local DB)
- **Type:** Local-first desktop application (Windows/macOS/Linux)
- **Backend:** `src-tauri/` directory with Rust code
- **Frontend:** Svelte SPA communicating via Tauri JS bridge
- **Database:** Single SQLite `.db` file in user's local app directory

## Planned Commands (Once Codebase Exists)
- **Backend tests:** `cargo test` (in `src-tauri/`)
- **Frontend tests:** `npm run test` or `vitest` (with `svelte-testing-library`)
- **Linting:** `pre-commit run -a` (runs Clippy, ESLint, Prettier, rustfmt)
- **Build:** TBD (likely `npm run tauri build`)

## Development Workflow
- **TDD Required:** All features must have tests written first (failing test → implementation → refactor)
- **Branching:** GitFlow model with `main` (releases) and `develop` (integration)
- **Feature branches:** `feature/F-X.X-description` from `develop`
- **Documentation:** All code changes must include documentation updates

## Current Actions Available
1. Review planning docs: `programme-management/strategos-kick-off-and-tdd.md`
2. Create GitHub repo following Step 1 in kick-off doc
3. Initialize Tauri project structure after repo creation
