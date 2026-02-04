# Phase 5 Spec: Zettelkasten Intelligence

## Goal
Implement core Zettelkasten knowledge-management features: Wiki-link detection, click-to-navigate, and a backlinks discovery view.

---

## 5.1 — Wiki-links: Detection + Navigation

### Detection
- Syntax: `[[Target]]` or `[[Target|Display Label]]` (pipe separates target from the visible label).
- Use a Tree-sitter query or regex pass to detect `[[…]]` spans in the buffer.
- Style: distinct colour (e.g. the existing link blue `0x89ddff`) + underline on hover.
- Reuse the decoration layer from Phase 2 — Wiki-links are styled but **not** concealed (the brackets remain visible).

### Click-to-open navigation
1. User clicks a Wiki-link.
2. **Resolve** the target against the workspace:
   - Scan all files under the workspace root.
   - Match by **file stem** (filename without extension).
   - If multiple matches exist, prefer `.md` files.
3. **Found** → open the file in a new tab (or focus the existing tab if already open).
4. **Not found** → show a prompt: *"No file named `Target` found. Create it?"*
   - If yes, create `Target.md` in the workspace root and open it.

### Atomic Steps
- [ ] **5.1a** — Add Wiki-link span detection + link-coloured styling
- [ ] **5.1b** — Implement click handler + file-stem resolution logic
- [ ] **5.1c** — Implement "create if missing" prompt and file creation

---

## 5.2 — Backlinks Discovery

### Goal
When editing a file, show which other files in the workspace link *to* it via Wiki-links.

### Mechanism
1. On file open (and on a short debounce after edits), run a **background scan**:
   - Grep all `.md` files in the workspace for `[[CurrentFileStem]]`.
   - Collect matching file paths + the line numbers where the link appears.
2. Display results in a collapsible sidebar section labelled **"Backlinks"**:
   - Each entry shows the linking file's name and the surrounding line snippet.
3. Clicking a backlink entry opens that file in a tab and scrolls to the relevant line.

### Performance constraint
The scan must run on a **background task** and must never block the UI thread. For workspaces up to ~10 k `.md` files, a simple sequential grep is acceptable; revisit if latency is noticeable.

### Atomic Steps
- [ ] **5.2a** — Implement background backlink scanner (file-stem grep)
- [ ] **5.2b** — Implement Backlinks sidebar panel + click-to-navigate

---

## Open Questions
- **ID-based vs name-based links**: `[[20240202]]` (Zettel ID) vs `[[My Note]]` (human-readable name). Support both — match by stem regardless of format.
- **Zettel ID generation**: A quick command to insert `YYYYMMDDHHMM` at the cursor as a new note ID. Nice-to-have; add as a stretch if time allows in this phase.
