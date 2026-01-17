use gpui::{div, prelude::*, px, AnyElement, Context, Render, Window};
use gpui_component::ActiveTheme;
pub struct GitPage;

impl GitPage {
    pub fn new() -> Self {
        Self
    }
}

impl Render for GitPage {
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
                    .child("📦 Git"),
            )
            .child(
                div()
                    .mt(px(16.0))
                    .text_base()
                    .text_color(cx.theme().primary)
                    .child("Git integration feature to be implemented"),
            )
    }
}

impl crate::pages::Page for GitPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        <Self as Render>::render(self, window, cx).into_any_element()
    }
}
