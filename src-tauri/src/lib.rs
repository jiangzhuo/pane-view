use tauri::{window::WindowBuilder, webview::WebviewBuilder, WebviewUrl, AppHandle, LogicalPosition, LogicalSize};

pub async fn create_webview_window(
    app: &AppHandle,
    urls: &[&str],
    label: &str,
    title: &str,
    x: f64,
    y: f64,
) -> Result<(), String> {
    println!("Creating window with multiple WebViews: {}", label);
    
    // Create the main window
    let window = WindowBuilder::new(app, label)
        .title(title)
        .inner_size(800.0, 768.0)
        .position(x, y)
        .build()
        .map_err(|e| {
            println!("Failed to build window: {}", e);
            e.to_string()
        })?;

    // Add webviews to the window
    for (i, &url) in urls.iter().enumerate() {
        let webview_label = format!("{}_webview_{}", label, i);
        let x_pos = if i == 0 { 0.0 } else { 400.0 };  // First view at 0, second at 400
        let width = 400.0;  // Each view takes half the window width
        
        window.add_child(
            WebviewBuilder::new(
                &webview_label,
                WebviewUrl::External(url.parse().unwrap())
            )
            .initialization_script(&format!(
                "document.body.style.margin = '0'; document.documentElement.style.margin = '0';"
            )),
            LogicalPosition::new(x_pos, 0.0),
            LogicalSize::new(width, 768.0)
        ).map_err(|e| {
            println!("Failed to build webview: {}", e);
            e.to_string()
        })?;
    }

    println!("Window '{}' with multiple WebViews built successfully", label);
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
                    &["https://www.google.com/", "https://www.bing.com/"],
                    "google-view",
                    "Google",
                    100.0, // 初始 x 位置
                    100.0, // 初始 y 位置
                ).await {
                    println!("Failed to create Google WebView: {}", e);
                }
            });

            println!("Tauri application setup complete");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
