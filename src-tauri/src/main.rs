// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::WindowBuilder;
use tauri_plugin_window_state::Builder as windowStatePlugin;

fn main() {
    let tauri_app = tauri::Builder::default();

    tauri_app
        .plugin(windowStatePlugin::default().build())
        .setup(move |app| {
            let window_builder = WindowBuilder::new(
                app,
                "aplikasi",
                tauri::WindowUrl::App("https://kesamsatan.bapenda-kalselprov.online/".into()),
            )
            .title("KESAMSATAN STAGING")
            .resizable(true)
            .inner_size(1200.0, 900.0)
            .fullscreen(false)
            .resizable(true)
            .decorations(true)
            .skip_taskbar(false)
            .always_on_top(false)
            .initialization_script(include_str!("./inject/event.js"))
            .build()
            .expect("Error while building main window");

            window_builder.show().unwrap();
            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                #[cfg(target_os = "macos")]
                {
                    event.window().minimize().unwrap();
                    event.window().hide().unwrap();
                }

                #[cfg(not(target_os = "macos"))]
                event.window().close().unwrap();

                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
