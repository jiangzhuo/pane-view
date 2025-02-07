use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    Rect, WebViewBuilder,
};

pub fn create_webview_window(urls: &[&str], x: f64, y: f64, title: &str) -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(LogicalSize::new(800.0, 768.0))
        .with_position(LogicalPosition::new(x, y))
        .build(&event_loop)
        .unwrap();

    let build_webview = |builder: WebViewBuilder<'_>| -> wry::Result<wry::WebView> {
        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        let webview = builder.build(&window)?;

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        let webview = {
            use gtk::prelude::*;
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;

            let fixed = gtk::Fixed::new();
            let vbox = window.default_vbox().unwrap();
            vbox.pack_start(&fixed, true, true, 0);
            fixed.show_all();
            builder.build_gtk(&fixed)?
        };

        Ok(webview)
    };

    let size = window.inner_size().to_logical::<u32>(window.scale_factor());
    let mut webviews = Vec::new();

    // Create webviews based on provided URLs
    for (i, &url) in urls.iter().enumerate() {
        let x_pos = if i == 0 { 0 } else { size.width / 2 };
        let builder = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(x_pos, 0).into(),
                size: LogicalSize::new(size.width / 2, size.height).into(),
            })
            .with_url(url);
        let webview = build_webview(builder)?;
        webviews.push(webview);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let size = size.to_logical::<u32>(window.scale_factor());
                for (i, webview) in webviews.iter().enumerate() {
                    let x_pos = if i == 0 { 0 } else { size.width / 2 };
                    webview
                        .set_bounds(Rect {
                            position: LogicalPosition::new(x_pos, 0).into(),
                            size: LogicalSize::new(size.width / 2, size.height).into(),
                        })
                        .unwrap();
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Starting application...");
    
    // Create a window with Google and TradingView
    if let Err(e) = create_webview_window(
        &["https://www.google.com/", "https://www.tradingview.com/"],
        100.0,
        100.0,
        "Google & TradingView"
    ) {
        eprintln!("Failed to create window: {}", e);
    }
}
