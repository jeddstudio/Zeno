# Phase 2 Spec: Hybrid Rendering (WYSIWYG-ish)

## Goal
Render Markdown elements visually *inside* the editor — concealing syntax markers when the cursor is away, revealing them when the cursor moves onto the same line. This is the core "WYSIWYG-ish" experience that sets Zeno apart from a plain syntax-highlighted editor.

---

## Phase 2.1 — Inline Decorations (Cursor-Proximity Concealment)

### What it does

| Raw Markdown | Cursor away (rendered) | Cursor on line |
|:---|:---|:---|
| `# Heading` | Heading — large, coloured | `# Heading` — syntax colours |
| `**bold**` | **bold** | `**bold**` |
| `*italic*` | *italic* | `*italic*` |
| `` `code` `` | code — with background | `` `code` `` |
| `[text](url)` | text — link colour + underline | `[text](url)` |

### Technical Strategy

#### 1. Decoration records from Tree-sitter

Extend the highlight pass to produce `Decoration` records alongside `HighlightSpan`:

```rust
/// A single decoration: what to conceal and how to style the visible content.
struct Decoration {
    /// Full byte range of the construct (e.g. the entire `**bold**`)
    full_range: Range<usize>,
    /// Byte ranges of syntax markers to hide (e.g. each `**`)
    concealed: Vec<Range<usize>>,
    /// Visual style to apply to the visible (non-concealed) content
    style: DecorationStyle,
}

enum DecorationStyle {
    Heading { level: u8 },   // font-size increase per level
    Bold,                     // font-weight bold
    Italic,                   // font-style italic
    InlineCode,               // background colour
    Link { url: String },     // colour + underline
}
```

Tree-sitter node types to target (via query captures):

| Node type | Conceal | Keep visible |
|:---|:---|:---|
| `heading` | `#` prefix + space | heading text |
| `strong_emphasis` | both `**` delimiters | inner text |
| `emphasis` | both `*` delimiters | inner text |
| `inline_code` | both `` ` `` delimiters | code text |
| `link` | `[`, `](url)` | link text only |

#### 2. Cursor-line gate

In `prepaint`, compute `cursor_line_index` once. For each line:

- **Cursor on this line** → existing path: raw text + syntax-highlight runs (no change).
- **Cursor elsewhere** → decoration path: concealed text + styled runs.

This keeps the editing experience transparent — the moment you move into a line, the raw Markdown reappears.

#### 3. Concealed-text string construction

The display string passed to `shape_line` must **omit** the concealed marker bytes entirely (not just colour them invisibly — they must not occupy layout space).

Algorithm per decorated line:
1. Collect all `Decoration` records that overlap this line.
2. Walk the raw line text byte-by-byte. Skip any byte that falls inside a `concealed` range.
3. Produce a shortened display string and a parallel mapping table (original byte offset → display byte offset).
4. Build `TextRun`s whose `len` values sum to the shortened string length, applying the appropriate style per region.

#### 4. Style → TextRun mapping

| DecorationStyle | TextRun field changes |
|:---|:---|
| `Heading(1)` | `font` size + 4 px |
| `Heading(2)` | `font` size + 2 px |
| `Heading(3+)` | `font` size + 1 px |
| `Bold` | `font` weight → Bold |
| `Italic` | `font` style → Italic |
| `InlineCode` | `background_color = Some(code_bg)` |
| `Link` | `color = link_colour`, `underline = Some(…)` |

### Atomic Steps
- [ ] **2.1a** — Add `Decoration` + `DecorationStyle` types; extend tree-sitter output to emit them
- [ ] **2.1b** — Implement cursor-line gate + concealed-string builder in `prepaint`
- [ ] **2.1c** — Wire styled runs (bold, italic, code, link) into rendering
- [ ] **2.1d** — Add heading font-size decorations

### Open Questions
- Nested styles (e.g. `***bold italic***`)? → defer beyond MVP.
- Blockquote (`>`) concealment? → defer beyond MVP.

---

## Phase 2.2 — Inline Image Rendering

### Goal
Render `![caption](path)` as an actual image inside the editor flow, below the line containing the syntax.

### Technical Strategy

#### 1. Detection
Use Tree-sitter image node captures from the `markdown-inline` grammar:
- Query: `(image (link_destination) @path)`
- Capture both the full image-syntax byte range (for folding/reveal) and the destination path string.

#### 2. Inline block placement
Implement a Zeno-owned "inline block" system:
- An image block renders **directly below** the line containing the `![…](…)` syntax.
- Uses a GPUI image element with aspect-ratio-aware scaling.
- Max-width: constrain to the current editor column width so images never overflow.

#### 3. Path resolution
- **Local paths**: Resolve relative to the currently-open file's parent directory.
- **Remote URLs** (`http`/`https`): Investigate whether GPUI's image element supports remote fetching natively. If not, show a "remote images not yet supported" placeholder.

#### 4. Lifecycle and interaction
- Image blocks are invalidated when the syntax tree re-parses (i.e. after any edit to the image line).
- Clicking the image or placing the cursor on its line reveals the underlying `![…](…)` syntax — reusing the cursor-proximity logic from 2.1.
- Broken or missing local paths: show a placeholder icon with the caption text.

### Atomic Steps
- [ ] **2.2a** — Implement image-path resolution helper (relative to active file)
- [ ] **2.2b** — Implement inline image block rendering

### Open Questions
- Image resize handles? → out of scope for V1.
- Very large images (> 10 MB)? → cap rendered dimensions; consider lazy-loading.
