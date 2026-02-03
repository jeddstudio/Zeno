use gpui::{Context, IntoElement, Render, Window, div, prelude::*, px, rgb};

pub struct Workspace;

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_grow()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w(px(280.))
                            .bg(rgb(0x151515))
                            .p_4()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child("Sidebar"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0xb0b0b0))
                                    .child("Phase 0 placeholder"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_grow()
                            .bg(rgb(0x1e1e1e))
                            .p_6()
                            .gap_3()
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child("Zeno"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xb0b0b0))
                                    .child("Editor Area (Phase 0 placeholder)"),
                            ),
                    ),
            )
            .child(
                div()
                    .h(px(180.))
                    .bg(rgb(0x0f0f0f))
                    .p_3()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Terminal Panel"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xb0b0b0))
                            .child("Phase 0 placeholder (collapsed by default later)"),
                    ),
            )
    }
}
