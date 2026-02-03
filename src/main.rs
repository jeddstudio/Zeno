use gpui::{
    App, Application, Bounds, Context, IntoElement, Render, Window, WindowBounds, WindowOptions,
    div, prelude::*, px, rgb, size,
};

struct ZenoRoot;

impl Render for ZenoRoot {
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
                    .child("Bootstrapped GPUI window (editor UI wiring pending)."),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ZenoRoot),
        )
        .unwrap();
        cx.activate(true);
    });
}

