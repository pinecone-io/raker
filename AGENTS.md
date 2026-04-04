# Repository Guidelines

Raker - CLI product to help developers curate and manage contextual intelligence.

This repository hosts a CLI product to help developers curate and manage contextual intelligence from any machine - local or remote - into Pinecone's Autocontext infrastructure.

## Project Structure & Module Organization

- `cli/`: Rust CLI client application.
- `scripts/`: Dev automation (`build.sh`, `install.sh`, `clean.sh`).

## Build, Test, and Development Commands

- Build Rust: `cargo build --release` (creates binaries in `target/release/`).
- Run tests: `cargo test --verbose`.
- Formatting: `cargo fmt` (check with `cargo fmt --check`).
- Linting: `cargo clippy -- -D warnings`.

## Contributor Workflow Rules

- Keep changes minimal and consistent with existing patterns.
- Ensure all tests and linting pass before submitting changes.

## Coding Style & Naming Conventions

- Rust 2021, 4-space indent, `snake_case` for modules, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for consts.
- Format: `cargo fmt` (check with `cargo fmt --check`).
- Lint: `cargo clippy -- -D warnings` (fix or justify warnings).

## Testing Guidelines

- When tests are added, prefer small unit tests near code; name tests after behavior (e.g., `handles_invalid_token`).

## Commit & Pull Request Guidelines

- Conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`, `perf:`, `style:`.
- PRs must include: summary, test plan, breaking changes (if any), and linked issues.
- Before pushing: `cargo fmt --check`, `cargo clippy`, `cargo build --release`, and `cargo test`.
- **Etiquette: NEVER use emojis, NEVER reference AI assistants (Claude, Claude Code, etc.), NEVER add "Generated with" footers**; imperative subject (<50 chars) with details in body when needed.
- Branch naming: `type/short-description` (e.g., `feat/cli-auth`).

Note on commit message formatting:

- Do not include literal escape sequences like `\n` in commit subjects or bodies.
- Use actual newlines for paragraphs/bullets. If amending via scripts, verify the resulting message with `git log -1`.

## Session-Specific Instructions

- **NEVER run builds (`cargo build`, etc.) automatically.** The maintainer will build manually. Only run `cargo fmt --check`, `cargo clippy`, and `cargo test` as pre-commit checks.
- Coordinate actions: wait for explicit maintainer instruction before running long/destructive ops, publishing, or committing.
- Commit policy: never reference AI/assistants; no emojis; write professional, imperative, conventional commits. Commit after every meaningful change without waiting for the user to ask.
- Pre-commit checklist: `cargo fmt --check`, `cargo clippy`, `cargo test`.