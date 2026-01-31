#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
fn main() {
    // Placeholder entry point for gpui-based app.
    // TODO: Replace with actual gpui app initialization and run loop once gpui is pinned.
    nohrs::ui::NohrsApp::run();
}
