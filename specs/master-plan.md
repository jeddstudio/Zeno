# Zeno V1 Master Plan

This is the tracked (public) master plan for Zeno V1. When all items in this file are completed, Zeno is considered “V1”.

## Phase 0: Foundations (COMPLETED)
- [x] Atomic Step 0.1 — Boot GPUI window
- [x] Atomic Step 0.2 — Introduce `Workspace` root view
- [x] Atomic Step 0.3 — Add panel placeholders (Sidebar / Editor / Terminal)
- [x] Atomic Step 0.4 — Add basic theme constants

## Phase 1: Core Editor (IN PROGRESS)
- [x] Atomic Step 1.1 — Zeno-owned text input/editing (buffer + cursor + selection)
- [ ] Atomic Step 1.2 — Markdown syntax highlighting (Tree-sitter)

## Phase 2: Hybrid Rendering (PLANNED)
- [ ] Atomic Step 2.1 — Inline decorations (conceal Markdown markers based on cursor proximity)
- [ ] Atomic Step 2.2 — Inline image rendering

## Phase 3: Project & File System (PLANNED)
- [ ] Atomic Step 3.1 — Recursive file tree sidebar (expand/collapse)
- [ ] Atomic Step 3.2 — File opening + buffer model + tabs
- [ ] Atomic Step 3.3 — Basic file operations (create/rename/delete)

## Phase 4: Integrated Terminal (PLANNED)
- [ ] Atomic Step 4.1 — Embedded terminal panel (PTY)
- [ ] Atomic Step 4.2 — Terminal ↔ editor context bridge (current file, selection)

## Phase 5: Zettelkasten Intelligence (PLANNED)
- [ ] Atomic Step 5.1 — Wiki-links (`[[ID]]`) detection + navigation
- [ ] Atomic Step 5.2 — Backlinks discovery view

## Phase 6: V1 Polishing (PLANNED)
- [ ] Atomic Step 6.1 — Theme system (light/dark, tokens)
- [ ] Atomic Step 6.2 — Performance audit (typing latency, large directories)
- [ ] Atomic Step 6.3 — Export (HTML/PDF)
