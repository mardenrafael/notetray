// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    let quit = CustomMenuItem::new("quit", "Sair");
    let new_item = CustomMenuItem::new("open", "Abrir NoteTray");

    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(new_item);

    let tray = SystemTray::new().with_menu(tray_menu);

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .on_system_tray_event(
            |app: &tauri::AppHandle, event: SystemTrayEvent| match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "open" => {
                        let window = app.get_window("main").unwrap();
                        let _ = window.move_window(Position::TopRight);
                        window.show().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            },
        )
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .system_tray(tray)
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_, _| {});
}
