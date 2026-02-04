# Repository Guidelines

## Project Direction

- `docs/PROJECT_DIRECTION.md` is the source of truth for architectural decisions.
- If you have a local `ref/` directory, treat it as **read-only** study material — Zeno must never depend on it to build or run.

## Project Structure

- `src/main.rs` — GPUI app entry point.
- `src/ui/…` — Views and UI components.
- `src/editor/…` — Editor logic (buffer, cursor, selection).
- `src/markdown/…` — Markdown parsing and highlighting.
- `docs/` — Design and engineering documentation.
- `specs/` — Phase specs and the V1 master plan (`specs/master-plan.md` is the checklist).

## Build & Develop

| Command | Purpose |
|:---|:---|
| `cargo run` | Build and launch |
| `cargo build` | Compile without running |
| `cargo check` | Fast type-check (use while iterating) |
| `cargo test` | Unit + integration tests |
| `cargo fmt` | Format (run before committing) |
| `cargo clippy --all-targets --all-features` | Lint (fix all warnings before PR) |

## Coding Style

- Rust edition: **2024**.
- Formatting: Rustfmt defaults.
- Naming: `snake_case` (modules / functions), `PascalCase` (types), `SCREAMING_SNAKE_CASE` (consts).
- Keep UI code deterministic and side-effect-light; put I/O behind clear boundaries.

## Testing

- Unit tests: colocate with code via `#[cfg(test)] mod tests { … }`.
- Integration tests: add in `tests/` when behaviour crosses module boundaries.
- Test names describe behaviour, not implementation (e.g. `renders_empty_document`).

## Commits & Pull Requests

- Use Conventional Commits subjects: `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`.
- One logical change per commit — never mix refactors with behaviour changes.
- PRs must include: purpose / approach, linked issue (if any), and screenshots for any UI change.
- Pre-PR checklist: `cargo fmt` ✓ `cargo test` ✓ `cargo clippy` ✓

## Security

- Never commit secrets, API keys, or machine-specific file paths.
