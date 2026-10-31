#![recursion_limit = "1024"]

pub mod state;
pub mod db;
pub mod drives;
pub mod components;
pub mod app;

use std::time::Duration;
use gpui_kit::*;
use state::app_state::AppState;
use app::AppView;

fn main() {
    tracing_subscriber::fmt::init();

    gpui_kit::application().run(|cx| {
        gpui_kit::init(cx);

        // Initial drive detection
        let initial_drives = drives::list_drives();
        let app_state = cx.new(|_| {
            let mut state = AppState::new();
            state.set_drives(initial_drives);
            state
        });

        // Background drive monitor using GPUI's native background executor and timer
        let state_clone = app_state.clone();
        cx.spawn(|cx: &mut AsyncApp| {
            let cx = cx.clone();
            async move {
                let mut previous_ids: Vec<String> = Vec::new();
                loop {
                    // Poll drives on background executor so disk I/O never blocks UI
                    let current_drives = cx
                        .background_executor()
                        .spawn(async move { drives::list_drives() })
                        .await;

                    let current_ids: Vec<String> =
                        current_drives.iter().map(|d| d.id.clone()).collect();

                    if current_ids != previous_ids {
                        previous_ids = current_ids;
                        let state_clone = state_clone.clone();
                        cx.update(|cx: &mut App| {
                            state_clone.update(cx, |state, cx| {
                                state.set_drives(current_drives);
                                cx.notify();
                            });
                        });
                    }

                    // Native GPUI timer without external Tokio runtime dependency
                    cx.background_executor().timer(Duration::from_secs(3)).await;
                }
            }
        })
        .detach();

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
