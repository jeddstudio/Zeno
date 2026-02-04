# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Build & Iterate

```sh
cargo check          # fast type-check (use while iterating)
cargo run            # build + launch
cargo test           # unit + integration tests
cargo fmt            # format (run before committing)
cargo clippy --all-targets --all-features   # lint
```

Target platform is **macOS**. Edition is **2024**.

---

## Architecture Overview

Zeno is a GPUI-based desktop Markdown editor. The crate is a single binary (`zeno-editor`). Code is organised into three logical layers:

```
src/
├── main.rs              # GPUI Application bootstrap + key-binding table
├── ui/
│   ├── workspace.rs     # Top-level layout: sidebar | editor | terminal panel
│   ├── editor.rs        # EditorView (entity) + EditorElement (custom GPUI Element)
│   └── theme.rs         # Colour constants (dark theme)
├── editor/
│   └── state.rs         # EditorState — pure text buffer + cursor + selection
└── markdown/
    └── highlight.rs     # Tree-sitter → HighlightSpan (byte-range list)
```

### Layer responsibilities

| Layer | Owns | Does NOT own |
|---|---|---|
| `src/editor/` | Buffer text, cursor, anchor, all mutation logic | Any rendering, any GPUI types |
| `src/markdown/` | Parsing source → highlight spans | Colours, TextRuns, rendering |
| `src/ui/editor.rs` | Mapping spans → colours, line layout, painting, input events | Buffer mutations (delegates to EditorState) |

---

## Key Patterns

### EditorView vs EditorElement — the two-struct split

`EditorView` is a **gpui Entity** (owns state, handles actions/events, implements `Render`).
`EditorElement` is a **gpui Element** (the low-level paint primitive returned by `Render`).

This split exists because the GPUI `Element` trait has a strict lifecycle (`request_layout` → `prepaint` → `paint`) and cannot own arbitrary state across frames. `EditorView` holds the durable state; `EditorElement` holds only an `Entity<EditorView>` handle and reads/writes through it.

### Cached layout state for hit-testing

Mouse-to-offset conversion (`index_for_mouse_position`) needs the *previous frame's* layout data (bounds, line metrics, shaped lines). These are written back into `EditorView` at the end of `paint`:

```
prepaint  →  shape lines, compute quads
paint     →  draw, then store lines/bounds/line_starts into EditorView
next mouse event  →  read cached layout from EditorView to hit-test
```

Fields involved: `last_bounds`, `last_line_height`, `last_line_starts`, `last_lines`.

### UTF-8 ↔ UTF-16 bridging

macOS IME (and the GPUI `EntityInputHandler` trait) works in **UTF-16** offsets. All offsets inside `EditorState` are **UTF-8 byte** offsets. `EditorView` has a pair of helpers (`offset_to_utf16` / `offset_from_utf16`) that convert at every boundary. If you add a new `EntityInputHandler` method, follow the same pattern: convert the incoming UTF-16 range immediately, work in UTF-8 internally, and convert back on output.

### Highlight → TextRun pipeline

1. `highlight_markdown(&str)` → `Vec<HighlightSpan>` (byte ranges + `HighlightKind`)
2. Per line in `prepaint`, `runs_for_line()` clips spans to the current line, maps `HighlightKind` → colour via `color_for_highlight_kind`, and fills gaps with the base (unstyled) run.
3. The resulting `Vec<TextRun>` is handed to `window.text_system().shape_line(…)`.

Highlights are re-computed eagerly after every mutation (`update_highlights`). This is fine for small documents; revisit if performance degrades at scale.

---

## Important Constraints

### `ref/` is read-only
A `ref/` directory (if present) contains Zed/Zettlr source for *study only*. Never add a `path = "ref/…"` dependency or copy GPL/AGPL code from it. See `docs/PROJECT_DIRECTION.md`.

### macOS `core-text` pin
`Cargo.toml` pins `core-text = "=21.0.0"` on macOS. This prevents `core-graphics` version conflicts that surface as **E0308** errors deep in `zed-font-kit`. Do not remove or bump this pin. See `docs/TROUBLESHOOTING.md`.

### `EditorState` is a plain struct
It is **not** a gpui Entity. It lives inside `EditorView` as a plain owned field. Do not wrap it in `cx.new(…)` or give it a separate `Entity` handle.

---

## Where to add new code

| What you're adding | Where |
|---|---|
| New UI view / component | `src/ui/<name>.rs`, re-export from `src/ui/mod.rs` |
| Editor behaviour / buffer logic | `src/editor/` |
| Markdown parsing / analysis | `src/markdown/` |
| New key binding | `main.rs` → `cx.bind_keys(…)` table |
| New action | `actions!(…)` macro in the owning view, handler on that view |
| Colour / theme value | `src/ui/theme.rs` |

Unit tests go in `#[cfg(test)] mod tests` inside the same file. Integration tests (cross-module behaviour) go in `tests/`.
