use gpui::{div, prelude::*, px, AnyElement, Context, Render, Window};
use gpui_component::ActiveTheme;
pub struct S3Page;

impl Default for S3Page {
    fn default() -> Self {
        Self
    }
}

impl S3Page {
    pub fn new() -> Self {
        Self
    }
}

impl Render for S3Page {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(cx.theme().background)
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(cx.theme().primary)
                    .child("☁️ S3"),
            )
            .child(
                div()
                    .mt(px(16.0))
                    .text_base()
                    .text_color(cx.theme().primary)
                    .child("S3 compatible storage integration to be implemented"),
            )
    }
}

impl crate::pages::Page for S3Page {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        <Self as Render>::render(self, window, cx).into_any_element()
    }
}
