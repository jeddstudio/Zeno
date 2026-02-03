use gpui::{
    App, Context, FocusHandle, Focusable, IntoElement, Render, Window, div, prelude::*, px, rgb,
};

use crate::editor::EditorState;

pub struct EditorView {
    focus_handle: FocusHandle,
    editor: EditorState,
}

impl EditorView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            editor: EditorState::new(),
        }
    }

    fn display_lines(&self) -> Vec<String> {
        let cursor = self.editor.cursor();
        let text = self.editor.text();

        let mut display = String::with_capacity(text.len() + 1);
        display.push_str(&text[..cursor]);
        display.push('|');
        display.push_str(&text[cursor..]);

        display.split('\n').map(|line| line.to_owned()).collect()
    }
}

impl Focusable for EditorView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for EditorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("ZenoEditor")
            .track_focus(&self.focus_handle(cx))
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .p_4()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .text_size(px(14.))
            .children(
                self.display_lines()
                    .into_iter()
                    .map(|line| div().child(line)),
            )
    }
}
