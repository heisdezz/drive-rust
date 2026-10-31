#![recursion_limit = "256"]

pub mod state;
pub mod db;
pub mod drives;
pub mod components;
pub mod app;

use std::time::Duration;
use gpui_kit::*;
use state::app_state::AppState;
use app::AppView;
use drives::DriveMonitor;

fn main() {
    tracing_subscriber::fmt::init();

    gpui_kit::application().run(|cx| {
        gpui_kit::init(cx);

        // Initialize central AppState
        let app_state = cx.new(|_| {
            let mut state = AppState::new();
            // Initial drive detection
            let initial_drives = drives::list_drives();
            state.set_drives(initial_drives);
            state
        });

        // Spawn background drive monitor that polls every 3 seconds
        let state_clone = app_state.clone();
        cx.spawn(|cx: &mut AsyncApp| {
            let cx = cx.clone();
            async move {
                let monitor = DriveMonitor::new(Duration::from_secs(3));
                let mut rx = monitor.start();

                while let Some(drives) = rx.recv().await {
                    let state_clone = state_clone.clone();
                    let _ = cx.update(|cx: &mut App| {
                        state_clone.update(cx, |state, cx| {
                            state.set_drives(drives);
                            cx.notify();
                        });
                    });
                }
            }
        }).detach();

        // Configure main window options
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: Point {
                    x: px(100.0),
                    y: px(100.0),
                },
                size: Size {
                    width: px(1200.0),
                    height: px(800.0),
                },
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Rust Drive".into()),
                appears_transparent: true,
                traffic_light_position: None,
            }),
            window_min_size: Some(Size {
                width: px(900.0),
                height: px(600.0),
            }),
            ..Default::default()
        };

        gpui_kit::open_window(window_options, cx, |_, cx| {
            cx.new(|cx| AppView::new(app_state, cx))
        })
        .expect("Failed to initialize main GPUI window");
    });
}
