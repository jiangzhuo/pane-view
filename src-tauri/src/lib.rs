use winit::{
    application::ApplicationHandler,
    event::{WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode},
    window::{Window, WindowId},
};
use wry::{dpi::{LogicalPosition, LogicalSize}, Rect, WebContext, WebViewBuilder};
use winit::event::{DeviceEvent, DeviceId};

#[derive(Default)]
struct State {
    window: Option<Window>,
    webviews: Vec<wry::WebView>,
    urls: Vec<String>,
}

impl ApplicationHandler for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes();
        attributes.inner_size = Some(LogicalSize::new(800, 768).into());
        attributes.position = Some(LogicalPosition::new(100.0, 100.0).into());
        let window = event_loop.create_window(attributes).unwrap();

        let size = window.inner_size().to_logical::<u32>(window.scale_factor());

        #[cfg(target_os = "linux")]
        let data_path =
            std::path::PathBuf::from(concat!("/home/", env!("USER"), "/.config/pane-view/"));
        #[cfg(target_os = "linux")]
        if !std::path::Path::new(&data_path).exists() {
            let _ = std::fs::create_dir(&data_path);
        }
        // unite web context
        let mut web_context = WebContext::new(Some(data_path));

        // Calculate dimensions based on number of URLs
        let url_count = self.urls.len();
        let (columns, rows) = match url_count {
            1 => (1, 1),
            2 => (2, 1),
            3 => (3, 1),
            4 => (2, 2),
            _ => unreachable!(), // This should never happen due to validation in main.rs
        };

        let columns = columns as u32;
        let rows = rows as u32;
        let cell_width = size.width / columns;
        let cell_height = size.height / rows;

        // Create webviews based on provided URLs
        for (i, url) in self.urls.iter().enumerate() {
            let i = i as u32;
            let x_pos = (i % columns) * cell_width;
            let y_pos = (i / columns) * cell_height;
            let webview = WebViewBuilder::with_web_context(&mut web_context)
                .with_initialization_script(
                    r#"
                    window.addEventListener('contextmenu', (e) => {
                        e.preventDefault();
                    });
                    "#,
                )
                .with_bounds(Rect {
                    position: LogicalPosition::new(x_pos, y_pos).into(),
                    size: LogicalSize::new(cell_width, cell_height).into(),
                })
                .with_url(url.clone())
                .build_as_child(&window)
                .unwrap();
            self.webviews.push(webview);
        }

        self.window = Some(window);
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
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
        match event {
            WindowEvent::Resized(size) => {
                if let Some(window) = &self.window {
                    let size = size.to_logical::<u32>(window.scale_factor());
                    let url_count = self.webviews.len();
                    let (columns, rows) = match url_count {
                        1 => (1, 1),
                        2 => (2, 1),
                        3 => (3, 1),
                        4 => (2, 2),
                        _ => unreachable!(),
                    };

                    let columns = columns as u32;
                    let rows = rows as u32;
                    let cell_width = size.width / columns;
                    let cell_height = size.height / rows;

                    for (i, webview) in self.webviews.iter().enumerate() {
                        let i = i as u32;
                        let x_pos = (i % columns) * cell_width;
                        let y_pos = (i / columns) * cell_height;
                        webview
                            .set_bounds(Rect {
                                position: LogicalPosition::new(x_pos, y_pos).into(),
                                size: LogicalSize::new(cell_width, cell_height).into(),
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
            while gtk::events_pending() {
                gtk::main_iteration_do(false);
            }
        }
        event_loop.set_control_flow(ControlFlow::Poll);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(urls: Vec<String>) {
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
    let mut state = State {
        window: None,
        webviews: Vec::new(),
        urls,
    };

    event_loop.run_app(&mut state).unwrap();
}
