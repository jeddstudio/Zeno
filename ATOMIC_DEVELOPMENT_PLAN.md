# Master Plan: Zeno — The AI-Collaborative Markdown Editor

This document uses the **Atomic Work Method (原子工作法)**. We are building the "Next Generation" of knowledge management: an editor that bridges **Human Writing**, **AI Collaboration**, and **Markdown** using a GPU-accelerated UI (GPUI). The Terminal is not just a tool; it is the interface for AI Agents to act upon your knowledge base.

> **Authoritative checklist**: `specs/master-plan.md` — tick items off there as each atomic step completes.
> **Detailed specs**: `specs/phase-N-*.md` — one file per phase.

---

## 🏗️ Phase 0: The GPUI Foundation
**Goal**: Get a working desktop window using GPUI.

- **Atomic Step 0.1**: Initialize GPUI Project.
  - **Action**: Create a new Rust binary crate and add `gpui` as a dependency. Implement a basic `Application::run`.
  - **Verification**: Run `cargo run` and see a native window appear.
- **Atomic Step 0.2**: Workspace Layout.
  - **Action**: Create a `Workspace` view with a simple division: `Sidebar` (left) and `Editor Area` (right).
  - **Verification**: Visual distinction between the sidebar and the main content area.

---

## 📝 Phase 1: The Core Editor (Zeno Implementation)
**Goal**: A functional, high-performance editor implemented in Zeno.

- **Atomic Step 1.1**: Implement Basic Text Editing.
  - **Action**: Implement a minimal editable text surface (buffer + cursor + selection + input).
  - **Verification**: A text cursor appears, and you can type text into the window.
- **Atomic Step 1.2**: Markdown Highlighting (Tree-sitter).
  - **Action**: Configure the editor to use Tree-sitter for Markdown syntax highlighting.
  - **Verification**: Symbols like `#` and `*` trigger color changes.

---

## 🎨 Phase 2: Hybrid Rendering (WYSIWYG-ish)
**Goal**: Real-time rendering of Markdown elements *inside* the editor, inspired by Zettlr/Obsidian.

- **Atomic Step 2.1**: Inline Decorations for Markdown.
  - **Action**: Implement decorations that can hide Markdown symbols (like `**`) while not focused, and render the text as Bold.
  - **Verification**: `**bold text**` renders as bold text when the cursor is on a different line.
- **Atomic Step 2.2**: Image Inlining.
  - **Action**: Render images directly inside the editor flow (implementation owned by Zeno).
  - **Verification**: `![caption](path)` shows the actual image in the editor.

---

## 📂 Phase 3: Project & File System
**Goal**: Navigate local folders and manage files (Zeno-owned implementation).

- **Atomic Step 3.1**: File Tree Sidebar.
  - **Action**: Implement a recursive file walker and display it in the sidebar.
  - **Verification**: Clicking a file path logs the path to the console.
- **Atomic Step 3.2**: File Opening & Buffer Management.
  - **Action**: Connect sidebar clicks to the editor. Load file content into buffers/tabs.
  - **Verification**: Opening multiple files and switching between tabs works smoothly.
- **Atomic Step 3.3**: File Operations.
  - **Action**: Implement create, rename, and delete file operations from the sidebar.
  - **Verification**: New files appear in the tree; rename and delete work without errors.

---

## 💻 Phase 4: The AI Bridge (Integrated Terminal)
**Goal**: Embed a PTY-based terminal to enable seamless AI collaboration within the vault.

- **Atomic Step 4.1**: Embedded terminal panel.
  - **Action**: Implement an embedded terminal panel using a PTY backend.
  - **Verification**: Run `ls` in the embedded terminal.
- **Atomic Step 4.2**: Context-aware terminal (future).
  - **Action**: Allow the terminal to know which file is currently open in the editor.
  - **Verification**: A command in the terminal can reference `current_file.md`.

---

## 🚀 Phase 5: Zettelkasten Intelligence
**Goal**: Core Zettelkasten features.

- **Atomic Step 5.1**: Wiki-link (`[[ID]]`) detection.
  - **Action**: Detect `[[ID]]` and make it clickable.
  - **Verification**: Clicking `[[20240202]]` attempts to open the corresponding file.
- **Atomic Step 5.2**: Backlinks discovery.
  - **Action**: Scan the workspace for files that link to the current note; display them in the sidebar.
  - **Verification**: Editing a linked-to note shows the backlink entries; clicking one navigates to the source.

---

## 🧹 Phase 6: V1 Polishing & Export
**Goal**: Polish Zeno for its first official release.

- **Atomic Step 6.1**: Theme System.
  - **Action**: Implement a centralised theme manager with Dark and Light themes.
  - **Verification**: `Cmd+Shift+T` toggles between themes; all UI elements update.
- **Atomic Step 6.2**: Performance Audit.
  - **Action**: Measure and fix any typing latency or blocking I/O in file scanning.
  - **Verification**: Typing latency < 16 ms on a 10 k-line document.
- **Atomic Step 6.3**: Markdown Export.
  - **Action**: Export the active buffer to HTML; optionally to PDF via system print.
  - **Verification**: Exported HTML renders correctly in a browser.

