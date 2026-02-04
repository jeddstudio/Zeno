# Phase 3 Spec: Project & File System

## Goal
Transform the sidebar placeholder into a usable project explorer: recursive file tree, multi-file tab management, and basic file operations.

---

## 3.1 — Recursive File Tree Sidebar

### State model

```rust
struct FileTree {
    root: PathBuf,
    nodes: Vec<FileNode>,
}

struct FileNode {
    path:        PathBuf,
    is_dir:      bool,
    is_expanded: bool,          // meaningful for directories only
    children:    Vec<FileNode>,
}
```

### UI behaviour
- Indented rows: chevron (▶ / ▼) for directories, file icon for files.
- Click chevron → toggle expand / collapse; persist state in memory for the session.
- Click filename → open file in editor (delegates to 3.2).
- Default exclusions: hidden files/dirs (`.*`), `target/`, `node_modules/`.

### Atomic Steps
- [ ] **3.1a** — Implement `FileTree` state + recursive directory walker (background task)
- [ ] **3.1b** — Render tree rows with proper indentation + chevron toggle

---

## 3.2 — File Opening, Buffer Model & Tabs

### Buffer model

Each open file is represented by an `OpenBuffer`:

```rust
struct OpenBuffer {
    path:        PathBuf,
    editor:      Entity<EditorView>,   // reuse existing editor component
    is_modified: bool,
}
```

`Workspace` holds a `Vec<OpenBuffer>` and an `active_index: usize`.

### Tab bar
- Rendered in a horizontal strip above the editor area.
- Each tab: filename + a modified-indicator dot when unsaved changes exist.
- Click tab → switch active buffer; the editor area swaps content.
- Close tab (× button) → if modified, show a "save / discard / cancel" prompt; otherwise close silently.

### Sidebar ↔ tab sync
- The sidebar highlights (distinct background) the entry whose path matches the active tab.
- Opening a file that is already in a tab focuses that tab instead of creating a duplicate.

### Atomic Steps
- [ ] **3.2a** — Add `OpenBuffer` model + multi-buffer state to `Workspace`
- [ ] **3.2b** — Implement `TabBar` view component
- [ ] **3.2c** — Wire sidebar click → file read → new tab (or focus existing tab)

---

## 3.3 — Basic File Operations

### Supported operations

| Operation | Trigger | Behaviour |
|:---|:---|:---|
| New file | Button in sidebar header | Creates an empty `.md` file in the selected directory; opens it in a new tab |
| New folder | Button in sidebar header | Creates an empty directory; expands it in the tree |
| Rename | Double-click on a filename | Shows an inline text input pre-filled with the current name; renames on Enter |
| Delete | Right-click context menu or keybinding | Shows a confirmation prompt; closes the file's tab if open |

### Implementation notes
- All I/O via `std::fs`; run on a background task and refresh the `FileTree` on completion.
- Error handling: if an operation fails (e.g. permission denied), show an inline error message in the sidebar — do not panic or crash.
- Rename must also update any open `OpenBuffer` entry that references the old path.

### Atomic Steps
- [ ] **3.3a** — Implement New File / New Folder with sidebar refresh
- [ ] **3.3b** — Implement Rename via inline input
- [ ] **3.3c** — Implement Delete with confirmation prompt
