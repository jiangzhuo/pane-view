use serde::Deserialize;
use tauri::{Manager, WebviewWindow};

async fn create_webview_window(
    app: &tauri::AppHandle,
    url: &str,
    label: &str,
    title: &str,
    x: f64,
    y: f64,
) -> Result<(), String> {
    println!("Creating WebView window: {}", label);
    let builder = WebviewWindow::builder(
        app,
        label,
        tauri::WebviewUrl::External(url.parse().unwrap()),
    )
    .title(title)
    .inner_size(800.0, 768.0)
    .position(x, y)
    .decorations(true)
    .always_on_top(false)
    .resizable(true);

    println!("Building WebView...");
    builder.build().map_err(|e| {
        println!("Failed to build WebView: {}", e);
        e.to_string()
    })?;
    println!("WebView '{}' built successfully", label);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Starting Tauri application...");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            println!("Tauri application setup started");
            
            let app_handle = app.handle().clone();
            
            // 直接创建 WebViews
            tauri::async_runtime::spawn(async move {
                // 创建 Google WebView
                if let Err(e) = create_webview_window(
                    &app_handle,
                    "https://www.google.com/",
                    "google-view",
                    "Google",
                    100.0, // 初始 x 位置
                    100.0, // 初始 y 位置
                ).await {
                    println!("Failed to create Google WebView: {}", e);
                }

                // 创建 Bing WebView
                if let Err(e) = create_webview_window(
                    &app_handle,
                    "https://www.bing.com/",
                    "bing-view",
                    "Bing",
                    920.0, // Google窗口宽度(800) + 间距(20) + 初始x位置(100)
                    100.0,
                ).await {
                    println!("Failed to create Bing WebView: {}", e);
                }
            });

            println!("Tauri application setup complete");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
