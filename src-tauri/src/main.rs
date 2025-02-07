// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::tray::TrayIconBuilder;

fn main() {
    pane_view_lib::run();

    // tauri::Builder::default()
    //     .setup(|app| {
    //         let tray_icon = TrayIconBuilder::new()
    //             .icon(app.default_window_icon().unwrap().clone())
    //             .build(app)?;
    //         Ok(())
    //     })
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}