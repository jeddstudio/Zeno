# Phase 6 Spec: V1 Polishing & Export

## Goal
Mature Zeno for its first official release: a proper theme system, a final performance pass, and Markdown export capabilities.

---

## 6.1 — Theme System (Light / Dark)

### Architecture
- Define a `ZenoTheme` struct that holds **all** colour tokens used across the app:
  - Backgrounds (app, sidebar, terminal, editor)
  - Text (primary, muted, syntax categories)
  - UI accents (selection, cursor, link, code background)
- Store the active theme in **app-level state** so every view reads from a single source.
- Ship two built-in themes: **Dark** (the current Nord-inspired palette) and **Light**.
- Action: `ToggleTheme` bound to `Cmd+Shift+T`.

### Migration
Refactor all hardcoded `rgb(…)` calls in `theme.rs`, `editor.rs`, `workspace.rs`, and any other views added in Phases 2–5 to read colour tokens from the active theme instead.

### Atomic Steps
- [ ] **6.1a** — Define `ZenoTheme` struct + populate the Dark theme with current colour values
- [ ] **6.1b** — Refactor all existing views to read colours from the active theme
- [ ] **6.1c** — Add the Light theme + `ToggleTheme` action + keybinding

---

## 6.2 — Performance Audit

### Targets
| Metric | Target |
|:---|:---|
| Typing latency (input → repaint) | < 16 ms on a 10 k-line document |
| Workspace file-tree scan | < 1 s for a 10 k-file directory |
| Backlink scan (Phase 5) | Must not block the UI thread |

### Methodology
1. **Instrument** key paths: measure wall-clock time from input event receipt to the end of `paint` for the editor.
2. **Profile** with `cargo flamegraph` (or equivalent) if a hotspot is suspected.
3. **Fix** any blocking I/O on the main thread (file reads, directory walks, grep scans).
4. **Re-measure** after each fix to confirm the target is met.

### Atomic Steps
- [ ] **6.2a** — Add timing instrumentation to the editor input → paint loop
- [ ] **6.2b** — Audit file-tree walker and backlink scanner; ensure both run on background tasks
- [ ] **6.2c** — Resolve any issues found; re-measure and confirm targets are met

---

## 6.3 — Markdown Export (HTML / PDF)

### HTML export
- Use `pulldown-cmark` to convert the active buffer's Markdown to an HTML string.
- Action: `ExportToHtml` — writes an `.html` file into the same directory as the source `.md` file (e.g. `note.md` → `note.html`).
- The generated HTML should be self-contained (inline a minimal CSS reset so it renders cleanly in any browser).

### PDF export
- Feed the exported HTML into the **macOS system print dialog** (`NSPrintInfo`) to produce a PDF.
- Fallback: if a CLI tool like `wkhtmltopdf` is available on the system, offer it as an alternative.

### Atomic Steps
- [ ] **6.3a** — Add `pulldown-cmark` dependency; implement HTML export action
- [ ] **6.3b** — Wire PDF export via the macOS system print dialog
- [ ] **6.3c** — Add Export keybindings (`Cmd+Shift+E` for HTML) and verify output

---

## V1 Definition of Done

All items in `specs/master-plan.md` are checked. The released app:

- Opens a local workspace folder and displays a navigable file tree.
- Edits Markdown with WYSIWYG-ish inline rendering (markers conceal on cursor movement).
- Syntax-highlights, renders inline images, and navigates Wiki-links with backlink discovery.
- Includes a fully functional integrated terminal (PTY).
- Exports notes to HTML and PDF.
- Ships with Dark and Light themes.
- Meets the latency and responsiveness targets defined in 6.2.
