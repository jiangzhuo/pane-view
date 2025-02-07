use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    Rect, WebViewBuilder,
};

use tray_icon::{TrayIcon, TrayIconBuilder, menu::Menu, menu::MenuItem, Icon, TrayIconEvent};
use std::path::PathBuf;
use tray_icon::menu::MenuEvent;

#[derive(Default)]
struct State {
    window: Option<Window>,
    webviews: Vec<wry::WebView>,
    tray: Option<TrayIcon>,
}

impl ApplicationHandler for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes();
        attributes.inner_size = Some(LogicalSize::new(800, 768).into());
        attributes.position = Some(LogicalPosition::new(100.0, 100.0).into());
        let window = event_loop.create_window(attributes).unwrap();

        let size = window.inner_size().to_logical::<u32>(window.scale_factor());

        // URLs for the webviews
        let urls = vec!["https://www.google.com/", "https://www.tradingview.com/"];

        // Create webviews based on provided URLs
        for (i, url) in urls.iter().enumerate() {
            let x_pos = if i == 0 { 0 } else { size.width / 2 };
            let webview = WebViewBuilder::new()
                .with_bounds(Rect {
                    position: LogicalPosition::new(x_pos, 0).into(),
                    size: LogicalSize::new(size.width / 2, size.height).into(),
                })
                .with_url(*url)
                .build_as_child(&window)
                .unwrap();
            self.webviews.push(webview);
        }

        self.window = Some(window);




        // Create tray icon
        let icon_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("icons")
            .join("icon.png");

        let icon = {
            let icon_rgba = image::open(&icon_path)
                .expect("Failed to open icon path")
                .into_rgba8();
            let (icon_width, icon_height) = icon_rgba.dimensions();
            Icon::from_rgba(icon_rgba.into_raw(), icon_width, icon_height)
                .expect("Failed to create icon")
        };
        let tray_menu = Menu::new();
        let full_screen_item = MenuItem::new("Full Screen", true, None);
        let quit_item = MenuItem::new("Quit", true, None);

        tray_menu.append_items(&[&full_screen_item, &quit_item]).expect("Failed to append items to tray menu");

        println!("Tray menu items added: Quit and Full Screen");


        let tray = TrayIconBuilder::new()
            .with_icon(icon)
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Pane View")
            .build();

        println!("Tray icon built with menu");

        self.tray = Some(tray.unwrap());
        println!("Tray icon added to system tray");

    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => {
                if let Some(window) = &self.window {
                    let size = size.to_logical::<u32>(window.scale_factor());
                    for (i, webview) in self.webviews.iter().enumerate() {
                        let x_pos = if i == 0 { 0 } else { size.width / 2 };
                        webview
                            .set_bounds(Rect {
                                position: LogicalPosition::new(x_pos, 0).into(),
                                size: LogicalSize::new(size.width / 2, size.height).into(),
                            })
                            .unwrap();
                    }
                }
            }
            WindowEvent::CloseRequested => {
                std::process::exit(0);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {

        if let Ok(event) = TrayIconEvent::receiver().try_recv() {
            println!("tray event: {:?}", event);
        }
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            println!("menu event: {:?}", event);
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ))]
        {
            while gtk::events_pending() {
                gtk::main_iteration_do(false);
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Starting application...");

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    {
        use gtk::prelude::DisplayExtManual;

        gtk::init().unwrap();
        if gtk::gdk::Display::default().unwrap().backend().is_wayland() {
            panic!("This example doesn't support wayland!");
        }

        winit::platform::x11::register_xlib_error_hook(Box::new(|_display, error| {
            let error = error as *mut x11_dl::xlib::XErrorEvent;
            (unsafe { (*error).error_code }) == 170
        }));
    }

    let event_loop = EventLoop::new().unwrap();
    let mut state = State::default();


    event_loop.run_app(&mut state).unwrap();



}
