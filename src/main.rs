mod editor;
mod ui;

use gpui::AppContext;
use gpui::{
    App, Application, Bounds, Focusable, KeyBinding, WindowBounds, WindowOptions, px, size,
};

use ui::Workspace;
use ui::editor::{Backspace, Delete, Left, Newline, Right, SelectAll, SelectLeft, SelectRight};

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("enter", Newline, None),
        ]);

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
