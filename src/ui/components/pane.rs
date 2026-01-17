#![cfg(feature = "gui")]

use gpui::{div, prelude::*, px, App, IntoElement};
use gpui_component::ActiveTheme;
/// Non-functional tab bar (placeholder)
pub fn tab_bar(cx: &mut App) -> impl IntoElement {
    div()
        .flex()
        .gap_2()
        .p_2()
        .border_1()
        .border_color(cx.theme().accent)
        .bg(cx.theme().background)
        .text_color(cx.theme().accent_foreground)
        .child(div().px_2().py_1().border_1().border_color(cx.theme().accent).child("Tab 1"))
        .child(div().px_2().py_1().border_1().border_color(cx.theme().accent).child("Tab 2"))
}

/// Split container with a vertical resize bar (non-functional placeholder)
pub fn split_container<L: IntoElement, R: IntoElement>(
    left: L,
    right: R,
    cx: &mut App,
) -> impl IntoElement {
    div()
        .flex()
        .gap_1()
        .child(div().border_1().border_color(cx.theme().accent).child(left))
        .child(
            // Resize bar placeholder
            div().w(px(4.0)).bg(cx.theme().accent),
        )
        .child(div().border_1().border_color(cx.theme().accent).child(right))
}
