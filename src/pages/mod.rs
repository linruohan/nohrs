use gpui::AnyElement;
use gpui_component::IconName;

pub mod explorer;
pub mod extensions;
pub mod git;
pub mod icon_themes;
pub mod keymap;
pub mod s3;
pub mod search;
pub mod settings;
pub mod themes;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PageKind {
    Explorer,
    Search,
    Git,
    S3,
    Extensions,
    Settings,
    KeyMap,
    Themes,
    IconThemes,
}

impl PageKind {
    pub fn label(&self) -> &'static str {
        match self {
            PageKind::Explorer => "Explorer",
            PageKind::Search => "Search",
            PageKind::Git => "Git",
            PageKind::S3 => "S3",
            PageKind::Extensions => "Extensions",
            PageKind::Settings => "Settings",
            PageKind::KeyMap => "Keymap",
            PageKind::Themes => "Themes",
            PageKind::IconThemes => "Icon Themes",
        }
    }

    pub fn icon_name(&self) -> IconName {
        match self {
            PageKind::Explorer => IconName::Folder,
            PageKind::Search => IconName::Search,
            PageKind::Git => IconName::GitHub,
            PageKind::S3 => IconName::HardDrive,
            PageKind::Extensions => IconName::LayoutDashboard,
            PageKind::Settings => IconName::Settings,
            PageKind::KeyMap => IconName::Keyboard,
            PageKind::Themes => IconName::Palette,
            PageKind::IconThemes => IconName::Drama,
        }
    }

    pub fn all() -> Vec<PageKind> {
        vec![
            PageKind::Explorer,
            PageKind::Search,
            PageKind::Git,
            PageKind::S3,
            PageKind::Extensions,
            PageKind::Settings,
            PageKind::KeyMap,
            PageKind::Themes,
            PageKind::IconThemes,
        ]
    }
}

/// Trait for page rendering
pub trait Page {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> AnyElement
    where
        Self: Sized;
}
