#![warn(clippy::all)]

mod error;

use crate::error::BinderyError;
use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

type Result<T> = std::result::Result<T, BinderyError>;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .text_color(rgb(0xffffff))
            .bg(rgb(0x606060))
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .shadow_lg()
                    .border_b_1()
                    .border_color(rgb(0x202020))
                    .items_center()
                    .gap_2()
                    .child("")
                    .child("File")
                    .child("|")
                    .child("Edit")
                    .child("|")
                    .child("View")
                    .child("|")
                    .child("Help")
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .shadow_lg()
                    .border_b_1()
                    .border_color(rgb(0x202020))
                    .items_center()
                    .gap_2()
                    .child("")
                    .child("Open")
                    .child("|")
                    .child("Save")
                    .child("|")
                    .child("Undo")
                    .child("|")
                    .child("Redo")
                    .child("|")
                    .child("Export")
                    .child("|")
                    .child("Font")
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .size_full()
                    .child(
                        div()
                            .size_full()
                            .bg(rgb(0x303030))
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .bg(rgb(0x606060))
                            .border_l_1()
                            .border_color(rgb(0x202020))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .border_x_6()
                                    .border_color(rgb(0x606060))
                                    .child("Number of pages per signature")
                                    .child("Paper size")
                                    .child("Skipped pages")
                                    .child("Duplicated pages")
                                    .child("Margins")
                            )
                    )
            )
    }
}

pub(crate) fn main() -> Result<()> {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(450.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloWorld {
                    text: "World".into(),
                })
            },
        )
        .map_err(BinderyError::from)
        .unwrap();
        cx.activate(true);
    });
    Ok(())
}
