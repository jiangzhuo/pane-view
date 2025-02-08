use winit::{
    application::ApplicationHandler,
    event::{Event, WindowEvent, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};
use wry::{dpi::{LogicalPosition, LogicalSize}, Rect, WebContext, WebViewBuilder, WebViewExtUnix};
use std::path::PathBuf;
use webkit2gtk::WebContextBuilder;
use winit::event::{DeviceEvent, DeviceId, RawKeyEvent};

#[derive(Default)]
struct State {
    window: Option<Window>,
    webviews: Vec<wry::WebView>,
}

impl ApplicationHandler for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes();
        attributes.inner_size = Some(LogicalSize::new(800, 768).into());
        attributes.position = Some(LogicalPosition::new(100.0, 100.0).into());
        let window = event_loop.create_window(attributes).unwrap();

        let size = window.inner_size().to_logical::<u32>(window.scale_factor());

        // URLs for the webviews
        let urls = vec!["https://www.tradingview.com/chart/?symbol=FOREXCOM%3AJP225", "https://www.tradingview.com/chart/?symbol=ETHBTC"];

        #[cfg(target_os = "linux")]
        let data_path =
            std::path::PathBuf::from(concat!("/home/", env!("USER"), "/.config/pane-view/"));
        #[cfg(target_os = "linux")]
        if !std::path::Path::new(&data_path).exists() {
            std::fs::create_dir(&data_path);
        }
        // unite web context
        let mut web_context = WebContext::new(Some(data_path));

        // Create webviews based on provided URLs
        for (i, url) in urls.iter().enumerate() {
            let x_pos = if i == 0 { 0 } else { size.width / 2 };
            let webview = WebViewBuilder::with_web_context(&mut web_context)
                .with_initialization_script(
                    r#"
                    window.addEventListener('contextmenu', (e) => {
                        e.preventDefault();
                    });
                    "#,
                )
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
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::Key(key_event) if key_event.physical_key == KeyCode::F11 && key_event.state.is_pressed() => {
                println!("F11 pressed, toggling full screen");
                if let Some(window) = &self.window {
                    if window.fullscreen().is_some() {
                        println!("Unsetting fullscreen");
                        window.set_fullscreen(None);
                    } else {
                        println!("Setting fullscreen");
                        window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                    }
                }
            }
            _ => {}
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        println!("Handling window event: {:?}", event);
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

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ))]
        {
            use gtk::prelude::*;
            while gtk::events_pending() {
                gtk::main_iteration_do(false);
            }
        }
        event_loop.set_control_flow(ControlFlow::Poll);
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
