# Strategos

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/ChrisClements1987/strategos)
[![Version](https://img.shields.io/badge/version-0.1.0-orange.svg)](https://github.com/ChrisClements1987/strategos/releases)

> A fast, local-first portfolio management tool for power users.

## Vision

To empower individual strategists, founders, and product managers with a private, fast, and highly flexible tool for managing complex business portfolios and roadmaps.

## Core Pillars

- **Privacy First** – All data is 100% local. The application is a self-contained executable that works entirely offline.
- **Blazing Fast** – A native-like user experience built on a high-performance stack (Tauri + Svelte) that launches instantly and responds immediately.
- **Deep Flexibility** – A data model designed to handle real-world complexity: multiple portfolios, nested products (modules), service packages, and complex licence bundles.
- **Power-User Centric** – A "command-line feel" in a GUI. Built for productivity with a command palette, extensive keyboard shortcuts, and robust data portability.

## Tech Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Application Shell** | [Tauri](https://tauri.app/) | Cross-platform desktop framework (Windows, macOS, Linux) |
| **Backend** | [Rust](https://www.rust-lang.org/) | High-performance, secure backend for file system and database operations |
| **Frontend** | [Svelte](https://svelte.dev/) | Reactive UI framework compiled to optimized JavaScript |
| **Database** | [SQLite](https://www.sqlite.org/) | Local, single-file relational database |
| **Styling** | CSS/SCSS | Custom styling with modern CSS |

## Features (v0.1)

- ✅ Portfolio Management (CRUD)
- ✅ Product Management (CRUD)
- ✅ Feature Management (CRUD)
- ✅ Licence Management (CRUD)
- ✅ Client/Persona Management (CRUD)
- ✅ Requirement Management (CRUD)
- 🚧 Dynamic Phase (Roadmap) View
- 🚧 Dynamic Product View
- 🚧 Dynamic Licence View
- 🚧 Dynamic Client View

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [npm](https://www.npmjs.com/) or [pnpm](https://pnpm.io/)

### Installation

```bash
# Clone the repository
git clone https://github.com/ChrisClements1987/strategos.git
cd strategos

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Development

```bash
# Run frontend dev server
npm run dev

# Run Svelte checks
npm run check

# Run in watch mode
npm run check:watch
```

## Project Structure

```
strategos/
├── src/                    # Svelte frontend
│   ├── lib/               # Components, stores, utilities
│   ├── routes/            # SvelteKit routes
│   └── assets/            # Static assets
├── src-tauri/             # Rust backend
│   ├── src/               # Rust source code
│   └── icons/             # Application icons
├── docs/                  # Documentation
├── tests/                 # E2E and integration tests
└── scripts/               # Build and deployment scripts
```

## Contributing

We follow a strict **Test-Driven Development (TDD)** workflow. Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our development process and how to submit pull requests.

### Quick Start for Contributors

1. Fork the repository
2. Create a feature branch from `develop`: `git checkout -b feature/F-X.X-description`
3. Write tests first (TDD)
4. Implement the feature
5. Ensure all tests pass and code is formatted
6. Submit a PR against `develop`

## Roadmap

- **v0.1** – Core Engine + Basic Views (Current)
- **v0.2** – Product Composition + Advanced Attributes
- **v1.0** – Power-User UX + Data Portability
- **v2.0** – Advanced Views + Plugin System

See the [full backlog](https://github.com/ChrisClements1987/strategos/issues) for detailed features.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Built with ❤️ using [Tauri](https://tauri.app/), [Svelte](https://svelte.dev/), and [Rust](https://www.rust-lang.org/).

---

**Status:** 🚧 v0.1 in active development
