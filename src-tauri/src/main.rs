// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{
    CustomMenuItem, Manager, Runtime, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

#[derive(Serialize, Deserialize)]
struct NotePayload<'a> {
    name: &'a str,
}

#[tauri::command]
async fn create_note<R: Runtime>(
    handle: tauri::AppHandle<R>,
    note_payload: NotePayload<'_>,
) -> Result<(), ()> {
    match tauri::WindowBuilder::new(
        &handle,
        note_payload.name,
        tauri::WindowUrl::App("/note/".into()),
    )
    .title(note_payload.name)
    .always_on_top(true)
    .focused(true)
    .inner_size(400.0, 300.0)
    .resizable(false)
    .build()
    {
        Ok(_) => todo!("Adicionar ao noteManager"),
        Err(_) => todo!("Tratar erro de forma correta"),
    }

    // Ok(())
}

fn main() {
    let quit = CustomMenuItem::new("quit", "Sair");
    let new_item = CustomMenuItem::new("open", "Abrir NoteTray");

    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(new_item);

    let tray = SystemTray::new().with_menu(tray_menu);

    let _ = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_note])
        .on_system_tray_event(
            |app: &tauri::AppHandle, event: SystemTrayEvent| match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "open" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            },
        )
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let window_label = event.window().label();

                if window_label == "main" {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
            }
            _ => {}
        })
        .system_tray(tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
