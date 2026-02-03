use std::ops::Range;

use gpui::{
    App, Bounds, Context, CursorStyle, Element, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, FocusHandle, Focusable, GlobalElementId, LayoutId, MouseButton,
    MouseDownEvent, PaintQuad, Pixels, Point, ShapedLine, SharedString, Style, TextRun,
    UTF16Selection, UnderlineStyle, Window, actions, div, fill, hsla, point, prelude::*, px,
    relative, rgb, rgba,
};

use crate::editor::EditorState;

actions!(
    zeno_editor,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Newline
    ]
);

pub struct EditorView {
    focus_handle: FocusHandle,
    editor: EditorState,
    placeholder: SharedString,
    marked_range: Option<Range<usize>>,
    last_bounds: Option<Bounds<Pixels>>,
    last_line_height: Option<Pixels>,
    last_line_starts: Option<Vec<usize>>,
    last_lines: Option<Vec<ShapedLine>>,
}

impl EditorView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            editor: EditorState::new(),
            placeholder: "Type here…".into(),
            marked_range: None,
            last_bounds: None,
            last_line_height: None,
            last_line_starts: None,
            last_lines: None,
        }
    }

    fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        self.editor.backspace();
        cx.notify();
    }

    fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        self.editor.delete_forward();
        cx.notify();
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        self.editor.move_left(false);
        cx.notify();
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        self.editor.move_right(false);
        cx.notify();
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.editor.move_left(true);
        cx.notify();
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.editor.move_right(true);
        cx.notify();
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.editor.select_all();
        cx.notify();
    }

    fn newline(&mut self, _: &Newline, _: &mut Window, cx: &mut Context<Self>) {
        self.editor.insert_str("\n");
        cx.notify();
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle(cx));
        let offset = self.index_for_mouse_position(event.position);
        if event.modifiers.shift {
            self.editor.set_selection(self.editor.anchor(), offset);
        } else {
            self.editor.set_cursor(offset);
        }
        cx.notify();
    }

    fn offset_to_utf16(&self, utf8_offset: usize) -> usize {
        let offset = utf8_offset.min(self.editor.text().len());
        self.editor.text()[..offset].encode_utf16().count()
    }

    fn offset_from_utf16(&self, utf16_offset: usize) -> usize {
        let text = self.editor.text();
        let mut count = 0usize;
        for (byte_idx, ch) in text.char_indices() {
            if count >= utf16_offset {
                return byte_idx;
            }
            count += ch.len_utf16();
        }
        text.len()
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.editor.text().is_empty() {
            return 0;
        }

        let Some(bounds) = self.last_bounds else {
            return 0;
        };
        let Some(line_height) = self.last_line_height else {
            return 0;
        };
        let Some(line_starts) = self.last_line_starts.as_ref() else {
            return 0;
        };
        let Some(lines) = self.last_lines.as_ref() else {
            return 0;
        };
        let Some(local) = bounds.localize(&position) else {
            return 0;
        };
        if lines.is_empty() {
            return 0;
        }

        let mut line_index = (local.y / line_height).floor() as isize;
        if line_index < 0 {
            line_index = 0;
        }
        let line_index = (line_index as usize).min(lines.len() - 1);

        let line = &lines[line_index];
        let col = line.index_for_x(local.x).unwrap_or(line.text.len());
        (line_starts[line_index] + col).min(self.editor.text().len())
    }
}

impl Focusable for EditorView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for EditorView {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.editor.text()[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.editor.selection_range()),
            reversed: self.editor.selection_reversed(),
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|r| self.range_from_utf16(r))
            .or(self.marked_range.clone())
            .unwrap_or_else(|| self.editor.selection_range());
        self.editor.replace_range(range, new_text);
        self.marked_range = None;
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|r| self.range_from_utf16(r))
            .or(self.marked_range.clone())
            .unwrap_or_else(|| self.editor.selection_range());

        self.editor.replace_range(range.clone(), new_text);
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else {
            self.marked_range = None;
        }

        if let Some(new_selected_range_utf16) = new_selected_range_utf16.as_ref() {
            let new_selected_range = self.range_from_utf16(new_selected_range_utf16);
            let anchor = (range.start + new_selected_range.start).min(self.editor.text().len());
            let cursor = (range.start + new_selected_range.end).min(self.editor.text().len());
            self.editor.set_selection(anchor, cursor);
        }

        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let Some(line_starts) = self.last_line_starts.as_ref() else {
            return None;
        };
        let Some(lines) = self.last_lines.as_ref() else {
            return None;
        };
        let Some(line_height) = self.last_line_height else {
            return None;
        };
        if lines.is_empty() {
            return None;
        }

        let range = self.range_from_utf16(&range_utf16);
        let (line_index, &line_start) = line_starts
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, &start)| start <= range.start)?;
        let line = lines.get(line_index)?;

        let local_start = (range.start - line_start).min(line.text.len());
        let local_end = (range.end - line_start).min(line.text.len());
        let x0 = line.x_for_index(local_start);
        let x1 = line.x_for_index(local_end);
        let top = bounds.top() + line_height * line_index as f32;
        let bottom = top + line_height;

        Some(Bounds::from_corners(
            point(bounds.left() + x0, top),
            point(bounds.left() + x1, bottom),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        Some(self.offset_to_utf16(self.index_for_mouse_position(point)))
    }
}

struct EditorElement {
    editor: Entity<EditorView>,
}

struct PrepaintState {
    lines: Vec<ShapedLine>,
    line_starts: Vec<usize>,
    selection_quads: Vec<PaintQuad>,
    cursor: Option<PaintQuad>,
    line_height: Pixels,
}

impl IntoElement for EditorElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for EditorElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let editor = self.editor.read(cx);
        let text = editor.editor.text();
        let selection = editor.editor.selection_range();
        let cursor = editor.editor.cursor();
        let style = window.text_style();
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();

        let (display_text, text_color) = if text.is_empty() {
            (editor.placeholder.clone(), hsla(0., 0., 1., 0.5))
        } else {
            (text.to_string().into(), style.color)
        };

        let mut line_starts = vec![0usize];
        for (idx, b) in display_text.as_bytes().iter().enumerate() {
            if *b == b'\n' {
                line_starts.push(idx + 1);
            }
        }
        if line_starts.is_empty() {
            line_starts.push(0);
        }

        let mut lines = Vec::with_capacity(line_starts.len());
        for (i, &start) in line_starts.iter().enumerate() {
            let raw_end = line_starts
                .get(i + 1)
                .copied()
                .unwrap_or(display_text.len());
            let end = if start == raw_end {
                raw_end
            } else if raw_end > 0 && display_text.as_bytes()[raw_end - 1] == b'\n' {
                raw_end - 1
            } else {
                raw_end
            };
            let slice = &display_text[start..end];

            let base_run = TextRun {
                len: slice.len(),
                font: style.font(),
                color: text_color,
                background_color: None,
                underline: None,
                strikethrough: None,
            };

            let runs = if !text.is_empty() {
                if let Some(marked_range) = editor.marked_range.as_ref() {
                    if marked_range.start >= start && marked_range.end <= end {
                        vec![
                            TextRun {
                                len: marked_range.start - start,
                                ..base_run.clone()
                            },
                            TextRun {
                                len: marked_range.end - marked_range.start,
                                underline: Some(UnderlineStyle {
                                    color: Some(base_run.color),
                                    thickness: px(1.0),
                                    wavy: false,
                                }),
                                ..base_run.clone()
                            },
                            TextRun {
                                len: end - marked_range.end,
                                ..base_run
                            },
                        ]
                        .into_iter()
                        .filter(|r| r.len > 0)
                        .collect()
                    } else {
                        vec![base_run]
                    }
                } else {
                    vec![base_run]
                }
            } else {
                vec![base_run]
            };

            lines.push(window.text_system().shape_line(
                slice.to_string().into(),
                font_size,
                &runs,
                None,
            ));
        }

        let mut selection_quads = vec![];
        if !text.is_empty() && !selection.is_empty() {
            for (i, &start) in line_starts.iter().enumerate() {
                let raw_end = line_starts.get(i + 1).copied().unwrap_or(text.len());
                let end = if start == raw_end {
                    raw_end
                } else if raw_end > 0 && text.as_bytes()[raw_end - 1] == b'\n' {
                    raw_end - 1
                } else {
                    raw_end
                };

                let overlap_start = selection.start.max(start);
                let overlap_end = selection.end.min(end);
                if overlap_start >= overlap_end {
                    continue;
                }

                let line = &lines[i];
                let x0 = line.x_for_index(overlap_start - start);
                let x1 = line.x_for_index(overlap_end - start);
                let top = bounds.top() + line_height * i as f32;
                let bottom = top + line_height;
                selection_quads.push(fill(
                    Bounds::from_corners(
                        point(bounds.left() + x0, top),
                        point(bounds.left() + x1, bottom),
                    ),
                    rgba(0x3355ff40),
                ));
            }
        }

        let cursor_quad = if selection.is_empty() {
            let cursor = cursor.min(display_text.len());
            let (line_index, &line_start) = line_starts
                .iter()
                .enumerate()
                .rev()
                .find(|&(_, &start)| start <= cursor)
                .unwrap_or((0, &0));
            let line_index = line_index.min(lines.len().saturating_sub(1));
            let line = &lines[line_index];
            let col = (cursor - line_start).min(line.text.len());
            let x = line.x_for_index(col);
            let top = bounds.top() + line_height * line_index as f32;
            Some(fill(
                Bounds::new(
                    point(bounds.left() + x, top),
                    gpui::size(px(2.), line_height),
                ),
                gpui::blue(),
            ))
        } else {
            None
        };

        PrepaintState {
            lines,
            line_starts,
            selection_quads,
            cursor: cursor_quad,
            line_height,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.editor.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.editor.clone()),
            cx,
        );

        for quad in prepaint.selection_quads.drain(..) {
            window.paint_quad(quad);
        }

        for (i, line) in prepaint.lines.iter().enumerate() {
            let origin = point(
                bounds.left(),
                bounds.top() + prepaint.line_height * i as f32,
            );
            line.paint(origin, prepaint.line_height, window, cx).ok();
        }

        if focus_handle.is_focused(window) {
            if let Some(cursor) = prepaint.cursor.take() {
                window.paint_quad(cursor);
            }
        }

        let lines = prepaint.lines.clone();
        let line_starts = prepaint.line_starts.clone();
        let line_height = prepaint.line_height;
        self.editor.update(cx, |editor, _cx| {
            editor.last_bounds = Some(bounds);
            editor.last_line_height = Some(line_height);
            editor.last_line_starts = Some(line_starts);
            editor.last_lines = Some(lines);
        });
    }
}

impl Render for EditorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("ZenoEditor")
            .track_focus(&self.focus_handle(cx))
            .cursor(CursorStyle::IBeam)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::newline))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .p_4()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .child(EditorElement {
                editor: cx.entity(),
            })
    }
}
