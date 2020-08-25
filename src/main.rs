use orbtk::prelude::*;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("nota - an OrbTk based note taking application")
                .position((48.0, 48.0))
                .size(640.0, 360.0)
                .resizeable(true)
                .child(TextBlock::create().text("TextBlock.").build(ctx))
                .build(ctx)
        })
        .run();
}
