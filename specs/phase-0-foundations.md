# Phase 0 Spec: Foundations (GPUI Skeleton)

## Goal
Establish a stable GPUI application skeleton that can serve as the base for all later phases.

## Definition of Done
1. `cargo run` launches a native window reliably.
2. The root view is structured as a `Workspace` container (even if most panels are placeholders).
3. The layout clearly shows three regions:
   - Left: `Sidebar` placeholder
   - Center: `EditorArea` placeholder
   - Bottom: `TerminalPanel` placeholder (collapsed by default is ok)

## Technical Strategy
- Keep Phase 0 UI **static** (no file IO, no terminal, no editor) to reduce moving parts.
- Use GPUI layout primitives (`div().flex()` etc.) to build the shell.
- Add minimal app-level state only if it is necessary for layout toggles (e.g. show/hide terminal).

## Proposed Atomic Steps
1. **Atomic Step 0.1**: Boot GPUI Window
   - Verify: a window appears.
2. **Atomic Step 0.2**: Introduce `Workspace` view
   - Verify: window content is built by a `Workspace` view.
3. **Atomic Step 0.3**: Add panel placeholders
   - Verify: sidebar/editor/terminal regions are visible (or terminal is togglable).
4. **Atomic Step 0.4**: Add basic theme constants
   - Verify: consistent background/borders/text colors across placeholders.

