# Phase 1 Spec: Core Editor (Zeno Implementation)

## Goal
Implement a **Zeno-owned** minimal editor core and wire it into the GPUI UI so typing is possible (no file I/O, no Markdown parsing yet).

This phase prioritizes **clarity, testability, and stable interfaces** over feature breadth.

## Assumptions
- Target platform for development is macOS.
- We may iterate UI rendering later; Phase 1.1’s UI can be visually simple as long as behavior is correct.

## Definition of Done

### Phase 1.1 — Zeno-owned text input/editing
1. `cargo run` launches and the editor area accepts keyboard input.
2. Text buffer is owned by Zeno (not borrowed from external editor crates).
3. Cursor is visible and moves with Left/Right arrows.
4. Backspace deletes the character before the cursor.
5. Enter inserts a newline.
6. Editor core logic has unit tests for basic editing operations.

### Phase 1.2 — Markdown syntax highlighting (Tree-sitter)
1. Tree-sitter Markdown integration exists behind a minimal interface.
2. Highlight spans are produced for a buffer.
3. Rendering uses spans (exact visuals can be basic).

## Interface (TAOUP: separate engine from UI)

### Editor Core (engine)
- **Input**: text operations (insert char/string, delete back/forward, move cursor).
- **Output**: buffer text + cursor position in a representation suitable for rendering.
- **Failure modes**: operations at bounds should be no-ops (robustness) and never panic.

### Editor View (UI)
- **Input**: key events from GPUI.
- **Output**: renders buffer + caret in the Editor Area.
- **Failure modes**: unknown keys ignored (silence).

## Modules
- `src/editor/` — editor engine (buffer, cursor, selection later).
- `src/ui/` — GPUI rendering and event wiring.

## Proposed Atomic Steps

### Atomic Step 1.1a — Editor core: buffer + cursor + operations
- [x] Implement editor core + tests
- Add `TextBuffer` and `Cursor` model (selection deferred).
- Add unit tests for:
  - inserting characters
  - backspace at middle and at start
  - cursor left/right bounds
  - newline insertion

Verify:
- `cargo test` passes.

### Atomic Step 1.1b — Editor view: render text + caret
- [x] Render editor view in Workspace
- Render the buffer text in the Workspace’s center panel.
- Render a visible caret (simple placeholder is acceptable).

Verify:
- `cargo run` shows editor area with caret.

### Atomic Step 1.1c — Key input wiring
- [x] Wire key input and text input
- Wire keydown/text input events to editor core:
  - printable characters insert
  - Enter inserts newline
  - Backspace deletes
  - Left/Right moves cursor

Verify:
- Manual: typing updates UI; caret moves; backspace works.

### Atomic Step 1.1d — Selection (optional follow-up)
- [x] Add mouse selection + shift selection
- Shift+Left/Right expands selection
- typing replaces selection

Verify:
- Manual selection behaves as expected; no panics.

### Atomic Step 1.2a — Add Markdown highlighter module (Tree-sitter)
- [x] Add `src/markdown/` module with a minimal API:
  - input: `&str`
  - output: highlight spans in byte ranges
- [x] Add unit test(s) for stable spans on a sample Markdown snippet

Verify:
- `cargo test` passes.

### Atomic Step 1.2b — Wire highlighting into editor rendering
- [ ] Convert highlight spans into `TextRun`s per line (basic colors are fine).
- [ ] Ensure un-highlighted text still renders correctly.

Verify:
- Manual: headings/emphasis/code show distinct colors.
