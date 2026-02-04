# Phase 4 Spec: Integrated Terminal

## Goal
Embed a PTY-based terminal in a bottom panel so users can run shell commands — including AI agents like Claude Code — directly inside Zeno without leaving the editor.

---

## 4.1 — Embedded Terminal Panel (PTY)

### Backend
- Use a PTY crate (research `portable-pty` vs alternatives before starting — see Open Questions).
- Spawn a default shell session (`$SHELL`) inside the PTY.
- Stream PTY output into a Zeno-owned terminal view via a GPUI background task.
- Forward keyboard input from the focused terminal panel to the PTY stdin.

### Panel UI
- Persistent bottom panel — the same region currently used by the placeholder.
- Toggle visibility: `Ctrl+`` ` (backtick). Remember last-used height.
- Resizable: drag the top border to adjust panel height.
- Follows the app's dark theme colours (Nord-inspired palette).

### Rendering engine
Implement a custom `TerminalElement` following the same pattern as `EditorElement`:

```
request_layout  →  claim full panel bounds
prepaint        →  parse PTY output into styled lines (SGR escape sequences)
paint           →  draw lines + blinking cursor
```

- Start with simple line-based rendering.
- Optimise via batching / dirty-region tracking if scrolling performance degrades.
- Blinking cursor: toggle visibility on a fixed interval via a GPUI timer.

### Atomic Steps
- [ ] **4.1a** — Add PTY crate dependency; spawn a shell session; stream output
- [ ] **4.1b** — Implement `TerminalPanel` view + `Ctrl+`` ` toggle keybinding
- [ ] **4.1c** — Implement `TerminalElement` with basic line rendering + SGR colour support
- [ ] **4.1d** — Wire keyboard input forwarding + blinking cursor
- [ ] **4.1e** — Add panel resize handle (drag top border)

---

## 4.2 — Terminal ↔ Editor Context Bridge

### Goal
Let the terminal session know which file is currently open, so shell commands and AI agents can act on it without the user specifying a path.

### Mechanism
- Set an environment variable `ZENO_FILE` in the PTY session to the absolute path of the active editor's file.
- Update `ZENO_FILE` whenever the active tab changes.
- (Stretch goal) Provide a `/zeno` shell helper script that can read or write the active buffer content directly.

### Atomic Steps
- [ ] **4.2a** — Inject `ZENO_FILE` into the PTY env; update on active-tab change
- [ ] **4.2b** — *(Stretch)* Implement a basic `/zeno` helper command

---

## Open Questions
- **Which PTY crate?** Research `portable-pty` vs alternatives for macOS stability and async compatibility with GPUI. Decision must be made before 4.1a.
- **Scrollback buffer size?** Start with 10 k lines; make it configurable later.
- **SGR colour support?** Essential for `claude code` and colourised shell output. Prioritise basic 256-colour and true-colour (24-bit) support in 4.1c.
