use gpui::{Context, IntoElement, Render, Window, div, prelude::*, rgb};

pub struct Workspace;

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .p_6()
            .gap_3()
            .child(div().text_2xl().font_weight(gpui::FontWeight::BOLD).child("Zeno"))
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xb0b0b0))
                    .child("Workspace root view (panels pending)."),
            )
    }
}

