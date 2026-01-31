#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use gpui::Application;
use gpui_component_assets::Assets;
use nohrs::{
    core::telemetry::logging::init_logging,
    ui,
    ui::{create_new_window, NohrsApp},
};

fn main() {
    // Placeholder entry point for gpui-based app.
    // TODO: Replace with actual gpui app initialization and run loop once gpui is pinned.
    init_logging();
    let app = Application::new().with_assets(Assets);
    app.run(move |cx| {
        ui::init(cx);
        cx.activate(true);

        create_new_window("Nohrs", move |window, cx| NohrsApp::view(window, cx), cx);
    });
}
