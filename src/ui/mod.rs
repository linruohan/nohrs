#![cfg(feature = "gui")]

pub mod app;
pub mod assets;
pub mod themes;
pub mod window;

// Public UI entry points that don't pull external UI toolkits yet.
pub use app::NohrsApp;
use gpui::{actions, Action, App, KeyBinding, SharedString};
use gpui_component::{scroll::ScrollbarShow, Root};
use serde::Deserialize;
pub mod app_menus;
pub mod components;
mod title_bar;

pub use components::file_list;
pub use title_bar::AppTitleBar;

actions!(ui, [
    About,
    Open,
    Quit,
    ToggleSearch,
    TestAction,
    Tab,
    TabPrev,
    ShowPanelInfo,
    ToggleListActiveHighlight
]);
#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = ui, no_json)]
pub struct SelectScrollbarShow(ScrollbarShow);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = ui, no_json)]
pub struct SelectLocale(SharedString);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = ui, no_json)]
pub struct SelectFont(usize);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = ui, no_json)]
pub struct SelectRadius(usize);

pub fn init(cx: &mut App) {
    gpui_component::init(cx);
    themes::init(cx);
    cx.bind_keys([
        KeyBinding::new("/", ToggleSearch, None),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-o", Open, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-o", Open, None),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-q", Quit, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("alt-f4", Quit, None),
    ]);

    cx.on_action(|_: &Quit, cx: &mut App| {
        cx.quit();
    });

    cx.on_action(|_: &About, cx: &mut App| {
        if let Some(window) = cx.active_window().and_then(|w| w.downcast::<Root>()) {
            cx.defer(move |cx| {
                window
                    .update(cx, |root, window, cx| {
                        root.push_notification(
                            "GPUI Component Storybook\nVersion 0.1.0",
                            window,
                            cx,
                        );
                    })
                    .unwrap();
            });
        }
    });

    cx.activate(true);
}
