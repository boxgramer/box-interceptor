// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    std::env::set_var("GDK_BACKEND", "x11");
    std::env::set_var("WRY_BACKEND", "x11");
    box_interceptor_lib::run()
}
