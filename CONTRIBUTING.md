# Contributing to Strategos

Thank you for your interest in contributing to Project Strategos! This document outlines our development workflows and guidelines.

## Table of Contents

- [How to Contribute](#how-to-contribute)
- [TDD (Test-Driven Development) Workflow](#tdd-test-driven-development-workflow)
- [Development-Including-Documentation Workflow](#development-including-documentation-workflow)
- [Branch Strategy](#branch-strategy)
- [Code Quality Standards](#code-quality-standards)
- [Commit Message Guidelines](#commit-message-guidelines)

## How to Contribute

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/strategos.git
   cd strategos
   ```
3. **Create a feature branch** from `develop`:
   ```bash
   git checkout develop
   git checkout -b feature/F-X.X-description
   ```
4. **Follow the TDD workflow** (see below)
5. **Ensure all tests pass** and code is properly formatted
6. **Submit a Pull Request** against the `develop` branch

## TDD (Test-Driven Development) Workflow

**All new features and bug fixes must be accompanied by tests.** We follow a strict TDD approach to ensure code quality and maintainability.

### The TDD Cycle

1. **Get the Task**
   - Pick an issue from the [GitHub Issues backlog](https://github.com/ChrisClements1987/strategos/issues)
   - Comment on the issue to let others know you're working on it

2. **Create a Feature Branch**
   ```bash
   git checkout develop
   git checkout -b feature/F-X.X-description
   ```
   Use the naming convention: `feature/F-{EPIC}-{FEATURE}-{description}`

3. **Write a Failing Test**
   
   **For Backend Logic (Rust):**
   - Add tests in `src-tauri/src/` (in the same file or a `tests` module)
   - Example:
     ```rust
     #[cfg(test)]
     mod tests {
         use super::*;

         #[test]
         fn test_create_portfolio() {
             let portfolio = Portfolio::new("Test Portfolio");
             assert_eq!(portfolio.name, "Test Portfolio");
         }
     }
     ```
   - Run tests: `cargo test --manifest-path src-tauri/Cargo.toml`

   **For Frontend Components (Svelte):**
   - Add tests in `src/__tests__/` using `vitest` and `@testing-library/svelte`
   - Example:
     ```typescript
     import { render, screen } from '@testing-library/svelte';
     import { describe, it, expect } from 'vitest';
     import PortfolioCard from '../lib/components/PortfolioCard.svelte';

     describe('PortfolioCard', () => {
       it('renders portfolio name', () => {
         render(PortfolioCard, { props: { name: 'Test Portfolio' } });
         expect(screen.getByText('Test Portfolio')).toBeInTheDocument();
       });
     });
     ```
   - Run tests: `npm test`

4. **Run Tests**
   - Confirm the new test **fails**
   - Confirm all existing tests **pass**

5. **Write the Minimal Code**
   - Write the simplest possible code to make the failing test pass
   - Use AI assistants (GitHub Copilot, etc.) to help:
     - *"Here is my failing test [paste test]. Write the minimal code to make this test pass."*

6. **Run Tests Again**
   - Confirm **all tests now pass**
   ```bash
   # Backend
   cargo test --manifest-path src-tauri/Cargo.toml

   # Frontend
   npm test
   ```

7. **Refactor**
   - Clean up the code
   - Ensure it follows project conventions
   - Run linters:
     ```bash
     pre-commit run -a
     ```

8. **Commit**
   ```bash
   git add .
   git commit -m "feat(F-1.1): Add portfolio CRUD logic and tests"
   ```

## Development-Including-Documentation Workflow

**Code is not "done" until it is documented.**

### Documentation Requirements

1. **User-Facing Changes**
   - If your change affects the UI or user interaction, update the user documentation in `/docs/user/`
   - Include screenshots or GIFs for UI changes

2. **Developer-Facing Changes**
   - If you change an API, data model, or core process, update:
     - This `CONTRIBUTING.md`
     - Relevant code comments
     - API documentation in `/docs/developer/api/`
     - Architecture docs in `/docs/developer/architecture/`

3. **PR Checklist**
   - Every Pull Request must check: **"I have updated all relevant documentation"**

## Branch Strategy

We follow **GitFlow**:

- `main` – Production-ready releases only
- `develop` – Integration branch for features
- `feature/F-X.X-description` – Feature branches (from `develop`)
- `hotfix/description` – Emergency fixes (from `main`)

### Workflow

```bash
# Start a new feature
git checkout develop
git pull origin develop
git checkout -b feature/F-1.1-portfolio-crud

# ... develop with TDD ...

# Push and create PR
git push origin feature/F-1.1-portfolio-crud
# Create PR against 'develop' on GitHub
```

## Code Quality Standards

### Pre-Commit Hooks

We use `pre-commit` to enforce code quality. Install hooks:

```bash
pip install pre-commit
pre-commit install
```

Run manually:
```bash
pre-commit run -a
```

### Linting

- **Rust**: `cargo clippy --manifest-path src-tauri/Cargo.toml`
- **JavaScript/Svelte**: `npm run lint` (ESLint)

### Formatting

- **Rust**: `cargo fmt --manifest-path src-tauri/Cargo.toml`
- **JavaScript/Svelte**: `npm run format` (Prettier)

### Type Checking

- **Svelte/TypeScript**: `npm run check`

## Commit Message Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

- `feat` – A new feature
- `fix` – A bug fix
- `docs` – Documentation only
- `style` – Code style changes (formatting, etc.)
- `refactor` – Code refactoring
- `test` – Adding or updating tests
- `chore` – Maintenance tasks

### Scopes (Examples)

- `F-1.1` – Feature/Epic reference
- `portfolio` – Module/component name
- `db` – Database layer
- `ui` – User interface

### Examples

```bash
git commit -m "feat(F-1.1): Add portfolio CRUD operations"
git commit -m "test(portfolio): Add unit tests for Portfolio model"
git commit -m "docs(readme): Update installation instructions"
git commit -m "fix(F-1.1): Correct portfolio deletion logic"
```

## Pull Request Process

1. **Ensure all tests pass**
2. **Run pre-commit hooks**: `pre-commit run -a`
3. **Update documentation**
4. **Fill out the PR template completely**
5. **Link related issues**: `Closes #XX`
6. **Request review** from maintainers
7. **Address feedback** promptly

## Getting Help

- **Questions?** Open a [Discussion](https://github.com/ChrisClements1987/strategos/discussions)
- **Bug Report?** Open an [Issue](https://github.com/ChrisClements1987/strategos/issues)
- **Feature Request?** Check the backlog or open an issue

---

Thank you for contributing to Strategos! 🚀
