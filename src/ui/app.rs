use gpui::{div, prelude::*, px, AnyElement, App, Context, Entity, IntoElement, Render, Window};
use gpui_component::{input::InputState, orange_200, ActiveTheme, Icon, Root};
use tracing::info;

use crate::{
    pages::{
        explorer::ExplorerPage, extensions::ExtensionsPage, git::GitPage, s3::S3Page,
        search::SearchPage, settings::SettingsPage, PageKind,
    },
    ui::components::layout::{
        footer::{footer, FooterProps},
        unified_toolbar::{AccountMenuAction, AccountMenuCommand},
    },
};

pub struct NohrsApp {
    current_page: PageKind,
    // Page entities
    explorer: Entity<ExplorerPage>,
    search: Entity<SearchPage>,
    git: Entity<GitPage>,
    s3: Entity<S3Page>,
    extensions: Entity<ExtensionsPage>,
    settings: Entity<SettingsPage>,
}
impl NohrsApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| InputState::new(window, cx));

        // Create page instances
        let explorer = cx.new(|cx| ExplorerPage::new(search_input.clone(), cx.focus_handle()));
        let search = cx.new(|_cx| SearchPage::new());
        let git = cx.new(|_cx| GitPage::new());
        let s3 = cx.new(|_cx| S3Page::new());
        let extensions = cx.new(|_cx| ExtensionsPage::new());
        let settings = cx.new(|_cx| SettingsPage::new());

        Self { current_page: PageKind::Explorer, explorer, search, git, s3, extensions, settings }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn set_page(&mut self, page: PageKind, cx: &mut Context<Self>) {
        if self.current_page != page {
            self.current_page = page;
            cx.notify();
        }
    }

    fn handle_account_action(
        &mut self,
        action: &AccountMenuAction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match action.command {
            AccountMenuCommand::ProfileSummary => {
                window.prevent_default();
            },
            AccountMenuCommand::Settings => self.set_page(PageKind::Settings, cx),
            AccountMenuCommand::Extensions => self.set_page(PageKind::Extensions, cx),
            AccountMenuCommand::Keymap => {
                // TODO: 实现快捷键设置功能
                info!("Keymap settings not yet implemented");
                window.prevent_default();
            },
            AccountMenuCommand::Themes => {
                // 切换到设置页面，用户可以在那里选择主题
                self.set_page(PageKind::Settings, cx);
            },
            AccountMenuCommand::IconThemes => {
                // TODO: 实现图标主题功能
                info!("Icon themes not yet implemented");
                window.prevent_default();
            },
            AccountMenuCommand::SignOut => {
                info!("Sign out requested");
            },
        }
    }

    fn render_navigation(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let active_page = self.current_page;

        div()
            .w(px(64.0))
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .bg(cx.theme().background)
            .border_r_1()
            .border_color(cx.theme().border)
            .py(px(16.0))
            .child(
                // Page navigation buttons
                div().flex().flex_col().items_center().gap_2().children(
                    PageKind::all().into_iter().map(|page| {
                        let is_active = active_page == page;
                        self.navigation_button(page, is_active, cx)
                    }),
                ),
            )
    }

    fn navigation_button(
        &self,
        page: PageKind,
        active: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id(("nav-btn", page as usize))
            .w(px(48.0))
            .h(px(48.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(8.0))
            .cursor_pointer()
            .when(active, |this| this.bg(cx.theme().background).shadow_sm())
            .when(!active, |this| this.hover(|style| style.bg(cx.theme().muted_foreground)))
            .on_click(cx.listener(move |view, _event, _window, cx| {
                view.set_page(page, cx);
            }))
            .child(Icon::new(page.icon_name()).size_5().text_color(if active {
                orange_200()
            } else {
                cx.theme().primary
            }))
    }

    fn render_active_page(&self, _window: &mut Window, _cx: &mut Context<Self>) -> AnyElement {
        match self.current_page {
            PageKind::Explorer => self.explorer.clone().into_any_element(),
            PageKind::Search => self.search.clone().into_any_element(),
            PageKind::Git => self.git.clone().into_any_element(),
            PageKind::S3 => self.s3.clone().into_any_element(),
            PageKind::Extensions => self.extensions.clone().into_any_element(),
            PageKind::Settings => self.settings.clone().into_any_element(),
        }
    }
}

impl Render for NohrsApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().background)
            .relative()
            .on_action(cx.listener(Self::handle_account_action))
            .child(
                // Main content: toolbar + page
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .min_h(px(0.0))
                    .child(
                        // Left navigation toolbar
                        self.render_navigation(cx),
                    )
                    .child(
                        // Main content area - render active page
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .min_w(px(0.0))
                            .child(self.render_active_page(window, cx)),
                    ),
            )
            .child(
                // Footer status bar
                footer(FooterProps::default(), cx),
            )
            .children(sheet_layer)
            .children(dialog_layer)
            .children(notification_layer)
    }
}
