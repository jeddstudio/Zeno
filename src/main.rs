mod editor;
mod ui;

use gpui::AppContext;
use gpui::{App, Application, Bounds, Focusable, WindowBounds, WindowOptions, px, size};

use ui::Workspace;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);
        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| cx.new(Workspace::new),
            )
            .unwrap();
        window
            .update(cx, |workspace, window, cx| {
                window.focus(&workspace.editor.focus_handle(cx));
                cx.activate(true);
            })
            .unwrap();
    });
}
