#![cfg(feature = "gui")]

use crate::ui::theme::theme;
use gpui::{div, prelude::*, px, rgb, Context, IntoElement, Pixels, Render, WindowControlArea};
use tracing::info;

pub const UNIFIED_TOOLBAR_HEIGHT: Pixels = px(44.0);
const SAMPLE_BUTTON_ID: &str = "unified-toolbar-sample-button";

#[derive(Clone)]
pub struct UnifiedToolbarProps {
    pub sample_button_label: String,
}

impl Default for UnifiedToolbarProps {
    fn default() -> Self {
        Self {
            sample_button_label: "サンプル".to_string(),
        }
    }
}

pub fn unified_toolbar<V: Render>(
    props: UnifiedToolbarProps,
    cx: &mut Context<V>,
) -> impl IntoElement {
    let UnifiedToolbarProps {
        sample_button_label,
    } = props;

    let drag_region = div()
        .id("unified-toolbar-drag-region")
        .flex_1()
        .h_full()
        .window_control_area(WindowControlArea::Drag);

    div()
        .id("unified-toolbar")
        .h(UNIFIED_TOOLBAR_HEIGHT)
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .px(px(16.0))
        .bg(rgb(theme::BG))
        .border_b_1()
        .border_color(rgb(theme::BORDER))
        .child(drag_region)
        .child(
            div()
                .id(SAMPLE_BUTTON_ID)
                .px(px(12.0))
                .py(px(6.0))
                .rounded(px(8.0))
                .bg(rgb(theme::ACCENT))
                .text_color(rgb(theme::WHITE))
                .text_sm()
                .cursor_pointer()
                .on_click(cx.listener(|_view, _event, _window, _cx| {
                    info!("sample toolbar button clicked");
                }))
                .hover(|style| style.bg(rgb(theme::ACCENT_HOVER)))
                .child(sample_button_label),
        )
}
