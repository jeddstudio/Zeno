# Project Direction (2026-02-03)

This document exists to prevent a recurring misunderstanding:

## Core decision

`ref/` is **reference-only** (for reading / research / AI assistance). Zeno must **not** depend on code inside `ref/` to build or run.

Concretely:
- Do **not** add `path = "ref/..."` dependencies in `Cargo.toml`.
- Do **not** require `ref/` to exist for `cargo build` / `cargo run`.
- Treat Zed/Zettlr as **inspiration and study material**, not as linked code.

## Why (licensing + future flexibility)

Zed contains crates under different licenses. Some are permissive (e.g. Apache-2.0), but many core editor/terminal/workspace crates are GPL-3.0-or-later (and some parts may be AGPL).

To keep future options open (including commercialization paths), Zeno avoids directly linking or copy/pasting GPL/AGPL code from reference projects.

This does **not** prevent:
- Using permissive dependencies (e.g. Apache/MIT/BSD) in Zeno.
- Re-implementing ideas/behaviors from Zed/Zettlr based on public documentation and observed behavior (clean-room style).
- Keeping `ref/` locally for study.

## Repo expectations

- `ref/` may be deleted at any time without breaking the build.
- If a future change introduces a dependency on `ref/`, treat it as a regression.

