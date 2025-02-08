use winit::{
    application::ApplicationHandler,
    event::{Event, WindowEvent, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    Rect, WebViewBuilder,
};

use std::path::PathBuf;
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
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::Added => {
                println!("Device added: {:?}", device_id);
            }
            DeviceEvent::Removed => {
                println!("Device removed: {:?}", device_id);
            }
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
            // WindowEvent::KeyboardInput { event, .. } => {
            //     println!("Keyboard input event: {:?}", event);
            //     if let PhysicalKey::Code(KeyCode::F11) = event.physical_key {
            //         println!("F11 key was pressed!");
            //         if event.state == ElementState::Pressed {
            //             if let Some(window) = &self.window {
            //                 if window.fullscreen().is_some() {
            //                     println!("Unsetting fullscreen");
            //                     window.set_fullscreen(None);
            //                 } else {
            //                     println!("Setting fullscreen");
            //                     window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
            //                 }
            //             }
            //         }
            //     }
            // }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
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
