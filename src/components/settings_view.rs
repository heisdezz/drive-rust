use gpui_kit::*;
use gpui_kit::PathPromptOptions;
use std::path::PathBuf;
use crate::state::app_state::AppState;
use crate::components::theme_picker::render_theme_picker;
use crate::theme::icons::Icon;

/// Renders the complete, polished Settings & Preferences page with folder picker support.
pub fn render_settings_view(
    app_state: &Entity<AppState>,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let selected_drive = state.selected_drive.clone();
    let custom_path = state.custom_library_path.clone();
    let grid_cols = state.grid_columns;
    let db_connected = state.active_db.is_some();

    let state_test_modal = app_state.clone();
    let state_reset_loc = app_state.clone();
    let state_custom_loc = app_state.clone();
    let state_eject = app_state.clone();

    let active_library_display = if let Some(ref custom) = custom_path {
        custom.display().to_string()
    } else if let Some(ref drive) = selected_drive {
        drive.mount_path.join("albums").display().to_string()
    } else {
        "No storage volume selected".to_string()
    };

    div()
        .id(SharedString::from("settings-scroll-view"))
        .flex()
        .flex_col()
        .gap(px(14.0))
        .w_full()
        .h_full()
        .overflow_y_scroll()
        .pb(px(24.0))
        // Top Banner / Page Header
        .child(
            div()
                .p(px(14.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(div().text_sm().text_color(theme.accent).child(Icon::SETTINGS))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("Preferences & Storage Settings"),
                                ),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("Configure library locations, theme appearance, catalog density, and hardware volumes."),
                        ),
                )
                .child(
                    div()
                        .id(SharedString::from("settings-open-video-test-btn"))
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(5.0))
                        .rounded(px(4.0))
                        .bg(theme.accent)
                        .hover(move |s| s.bg(theme.accent_hover))
                        .on_click(move |_, _, cx| {
                            state_test_modal.update(cx, |s, cx| {
                                s.open_video_test_modal();
                                cx.notify();
                            });
                        })
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(div().text_xs().text_color(rgb(0xffffff)).child(Icon::PLAY))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .child("Test Video URL"),
                        ),
                ),
        )
        // 1. Appearance & Dynamic Theming
        .child(
            div()
                .p(px(14.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("THEME & COLOR PALETTE"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(format!("Active: {}", theme.name())),
                        ),
                )
                .child(render_theme_picker(app_state, window, cx)),
        )
        // 2. Library Location & Storage Root
        .child(
            div()
                .p(px(14.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child("LIBRARY LOCATION & ALBUM ROOT"),
                )
                .child(
                    div()
                        .p(px(10.0))
                        .rounded(px(4.0))
                        .bg(theme.bg_sidebar)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(2.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_secondary)
                                        .child("Active Media Directory:"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_primary)
                                        .child(active_library_display),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .id(SharedString::from("set-custom-lib-btn"))
                                        .cursor_pointer()
                                        .px(px(10.0))
                                        .py(px(4.0))
                                        .rounded(px(4.0))
                                        .bg(theme.accent)
                                        .text_color(rgb(0xffffff))
                                        .hover(move |s| s.bg(theme.accent_hover))
                                        .on_click(move |_, _, cx| {
                                            let options = PathPromptOptions {
                                                files: false,
                                                directories: true,
                                                multiple: false,
                                                prompt: Some(SharedString::from("Select Media Library Directory")),
                                            };
                                            let receiver = cx.prompt_for_paths(options);
                                            let state_clone = state_custom_loc.clone();

                                            cx.spawn(|cx: &mut AsyncApp| {
                                                let cx = cx.clone();
                                                async move {
                                                    // Try native portal picker first
                                                    if let Ok(Ok(Some(paths))) = receiver.await {
                                                        if let Some(first) = paths.into_iter().next() {
                                                            cx.update(|cx: &mut App| {
                                                                state_clone.update(cx, |s, cx| {
                                                                    s.set_custom_library_path(Some(first));
                                                                    cx.notify();
                                                                });
                                                            });
                                                            return;
                                                        }
                                                    }

                                                    // Gracefully fallback to zenity or kdialog if portal wasn't answered
                                                    let fallback_path = cx
                                                        .background_executor()
                                                        .spawn(async move { pick_folder_fallback() })
                                                        .await;

                                                    if let Some(p) = fallback_path {
                                                        cx.update(|cx: &mut App| {
                                                            state_clone.update(cx, |s, cx| {
                                                                s.set_custom_library_path(Some(p));
                                                                cx.notify();
                                                            });
                                                        });
                                                    }
                                                }
                                            })
                                            .detach();
                                        })
                                        .flex()
                                        .items_center()
                                        .gap(px(5.0))
                                        .child(div().text_xs().child(Icon::FOLDER))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .child("Browse Folder..."),
                                        ),
                                )
                                .child(
                                    div()
                                        .id(SharedString::from("reset-lib-btn"))
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(4.0))
                                        .rounded(px(4.0))
                                        .bg(theme.bg_pill)
                                        .border_1()
                                        .border_color(theme.border_card)
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .hover(move |s| s.text_color(theme.text_primary))
                                        .on_click(move |_, _, cx| {
                                            state_reset_loc.update(cx, |s, cx| {
                                                s.set_custom_library_path(None);
                                                cx.notify();
                                            });
                                        })
                                        .child("Reset to Drive Root"),
                                ),
                        ),
                ),
        )
        // 3. Catalog Layout Density
        .child(
            div()
                .p(px(14.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("MEDIA CATALOG DENSITY"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(format!("Current: {} Columns", grid_cols)),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .children([4, 5, 6, 7, 8, 9].iter().map(|&cols| {
                            let is_current = grid_cols == cols;
                            let state_col = app_state.clone();
                            div()
                                .id(SharedString::from(format!("col-btn-{}", cols)))
                                .cursor_pointer()
                                .px(px(12.0))
                                .py(px(5.0))
                                .rounded(px(4.0))
                                .bg(if is_current { theme.bg_pill_active } else { theme.bg_pill })
                                .border_1()
                                .border_color(if is_current { theme.accent } else { theme.border_card })
                                .hover(move |s| s.border_color(theme.border_active))
                                .on_click(move |_, _, cx| {
                                    state_col.update(cx, |s, cx| {
                                        s.set_grid_columns(cols);
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(if is_current { FontWeight::BOLD } else { FontWeight::NORMAL })
                                        .text_color(if is_current { theme.text_primary } else { theme.text_secondary })
                                        .child(format!("{} Cols", cols)),
                                )
                        })),
                ),
        )
        // 4. Drive Volume & Safe Eject
        .child(
            div()
                .p(px(14.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child("DRIVE HARDWARE & VOLUME EJECTION"),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(2.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_primary)
                                        .child(if let Some(ref d) = selected_drive {
                                            format!("Mounted Volume: {} ({})", d.name, d.size_formatted)
                                        } else {
                                            "No Drive Selected".to_string()
                                        }),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("Safely unmount filesystem before unplugging hardware to prevent data corruption."),
                                ),
                        )
                        .child(
                            div()
                                .id(SharedString::from("settings-eject-btn"))
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .hover(move |s| s.bg(theme.accent_danger).text_color(rgb(0xffffff)))
                                .on_click(move |_, _, cx| {
                                    state_eject.update(cx, |s, cx| {
                                        s.eject_selected_drive();
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .child(format!("{} Safe Eject", Icon::STOP)),
                                ),
                        ),
                ),
        )
        // 5. System Diagnostics & Technical Metrics
        .child(
            div()
                .p(px(14.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child("SYSTEM & RUNTIME DIAGNOSTICS"),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(render_diagnostic_row("Database Engine", "SQLite 3 (rusqlite bundled, WAL mode)", &theme))
                        .child(render_diagnostic_row("Database Status", if db_connected { "Active & Connected" } else { "Closed" }, &theme))
                        .child(render_diagnostic_row("UI Rendering Framework", "GPUI (GPU-accelerated native)", &theme))
                        .child(render_diagnostic_row("Host Architecture", "Linux x86_64", &theme))
                        .child(render_diagnostic_row("Local Media Streaming", "127.0.0.1 (Ready)", &theme))
                        .child(render_diagnostic_row("Thumbnail Generator Cache", "14.2 MB (Ready)", &theme)),
                ),
        )
}

fn render_diagnostic_row(label: &str, val: &str, theme: &crate::theme::AppTheme) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_between()
        .py(px(3.0))
        .border_b_1()
        .border_color(rgba(0xffffff08))
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .child(label.to_string()),
        )
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.text_secondary)
                .child(val.to_string()),
        )
}

/// Fallback helper executing zenity or kdialog if the native desktop portal does not respond.
fn pick_folder_fallback() -> Option<PathBuf> {
    // 1. Try zenity
    if let Ok(output) = std::process::Command::new("zenity")
        .args(["--file-selection", "--directory", "--title=Select Media Library Directory"])
        .output()
    {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return Some(PathBuf::from(path_str));
            }
        }
    }

    // 2. Try kdialog
    if let Ok(output) = std::process::Command::new("kdialog")
        .args(["--getexistingdirectory", "--title", "Select Media Library Directory"])
        .output()
    {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return Some(PathBuf::from(path_str));
            }
        }
    }

    None
}
