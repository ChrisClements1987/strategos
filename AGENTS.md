# AGENTS.md - Project Strategos

## Project Status
**Current Phase:** v0.1 Core Engine & Dynamic Views COMPLETE
**Next Step:** Build cross-platform releases and polish UI

## Planned Architecture (v0.1+)
- **Stack:** Tauri (Rust backend) + Svelte (frontend) + SQLite (local DB)
- **Type:** Local-first desktop application (Windows/macOS/Linux)
- **Backend:** `src-tauri/` directory with Rust code
- **Frontend:** Svelte SPA communicating via Tauri JS bridge
- **Database:** Single SQLite `.db` file in user's local app directory

## Commands
- **Backend tests:** `cargo test --manifest-path src-tauri/Cargo.toml` (43 tests passing)
- **Frontend tests:** `npm test` (Vitest)
- **Dev mode:** `npm run tauri dev`
- **Type check:** `npm run check`
- **Linting:** `pre-commit run -a` (Clippy, ESLint, Prettier, rustfmt)
- **Build:** `npm run tauri build`

## Development Workflow
- **TDD Required:** All features must have tests written first (failing test → implementation → refactor)
- **Branching:** GitFlow model with `main` (releases) and `develop` (integration)
- **Feature branches:** `feature/F-X.X-description` from `develop`
- **Documentation:** All code changes must include documentation updates

## Current Actions Available
1. Review planning docs: `programme-management/strategos-kick-off-and-tdd.md`
2. Create GitHub repo following Step 1 in kick-off doc
3. Initialize Tauri project structure after repo creation

## v0.1 Completion Status
**Core Engine (E-1.1):** ✅ COMPLETE
- Portfolio, Product, Feature, Licence, Client, Requirement CRUD
- 43 backend unit tests passing
- Full database schema with relationships

**Dynamic Views (E-2.1):** ✅ COMPLETE
- Products view with portfolio filtering
- Licences view with portfolio filtering
- Clients view with portfolio filtering
- Features view with product filtering
- Requirements view with client filtering

**Remaining for v0.1:**
- Cross-platform builds (E-5.1)
- Documentation polish (E-5.2)
