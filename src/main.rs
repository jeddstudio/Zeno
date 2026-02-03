mod editor;
mod ui;

use gpui::AppContext;
use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, px, size};

use ui::Workspace;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Workspace),
        )
        .unwrap();
        cx.activate(true);
    });
}
