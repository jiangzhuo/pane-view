// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::tray::TrayIconBuilder;

use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    urls: Vec<String>,
}

fn read_config() -> Config {
    let config_path = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("pane-view")
        .join("config.toml");

    let config = match fs::read_to_string(&config_path) {
        Ok(content) => toml::from_str(&content).unwrap_or_else(|_| Config {
            urls: vec!["https://www.google.com".to_string()],
        }),
        Err(_) => Config {
            urls: vec!["https://www.google.com".to_string()],
        }
    };

    // Validate URL count
    if config.urls.is_empty() || config.urls.len() > 4 {
        panic!("Number of URLs must be between 1 and 4");
    }

    config
}

fn main() {
    let config = read_config();
    pane_view_lib::run(config.urls);

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