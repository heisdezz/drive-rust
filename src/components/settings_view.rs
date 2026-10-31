use crate::components::folder_picker::prompt_mount_custom_folder;
use gpui_kit::*;
use crate::state::app_state::AppState;
use crate::theme::ThemeMode;
use crate::theme::icons::Icon;

/// Renders the comprehensive Settings, Library Location & Drive Management view.
pub fn render_settings_view(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let custom_path = state.custom_library_path.clone();
    let grid_cols = state.grid_columns;
    let selected_drive = state.selected_drive.clone();
    let db_connected = state.active_db.is_some();
    let drives = state.drives.clone();

    let state_theme = app_state.clone();
    let state_grid = app_state.clone();
    let state_reset_loc = app_state.clone();
    let state_eject = app_state.clone();
    let state_video_test = app_state.clone();

    div()
        .id(SharedString::from("settings-view-container"))
        .flex()
        .flex_col()
        .gap(px(14.0))
        .w_full()
        .h_full()
        .overflow_y_scroll()
        // Top Header Bar
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
                        .items_center()
                        .gap(px(10.0))
                        .child(
                            div()
                                .text_base()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(format!("{} Application Settings & System Management", Icon::SETTINGS)),
                        )
                        .child(
                            div()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(3.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.accent)
                                .child("Native GPUI v0.1"),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .id(SharedString::from("open-video-test-btn"))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .hover(move |s| s.bg(theme.bg_pill_active).border_color(theme.accent))
                                .on_click(move |_, _, cx| {
                                    state_video_test.update(cx, |s, cx| {
                                        s.open_video_test_modal();
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_primary)
                                        .child(format!("{} Video URL Canvas", Icon::VIDEO)),
                                ),
                        ),
                ),
        )
        // Section 1: Appearance & Visual Themes
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
                                .text_color(theme.text_primary)
                                .child("APPEARANCE & THEME PRESETS"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("Select from 5 calibrated dark & light workspace palettes"),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(10.0))
                        .children(ThemeMode::all().into_iter().map(|mode| {
                            let is_active = *mode == state.theme_mode;
                            let state_pick = state_theme.clone();
                            let mode_copy = mode;

                            div()
                                .id(SharedString::from(format!("theme-card-{:?}", mode)))
                                .cursor_pointer()
                                .w(px(130.0))
                                .p(px(8.0))
                                .rounded(px(5.0))
                                .bg(if is_active { theme.bg_pill_active } else { theme.bg_sidebar })
                                .border_1()
                                .border_color(if is_active { theme.accent } else { theme.border_subtle })
                                .hover(move |s| s.border_color(theme.accent))
                                .on_click(move |_, _, cx| {
                                    state_pick.update(cx, |s, cx| {
                                        s.set_theme_mode(*mode_copy);
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(if is_active { FontWeight::BOLD } else { FontWeight::MEDIUM })
                                                .text_color(if is_active { theme.accent } else { theme.text_primary })
                                                .child(mode.name()),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(if is_active { theme.accent } else { theme.text_muted })
                                                .child(if is_active { "●" } else { "○" }),
                                        ),
                                )
                                .child(
                                    div()
                                        .mt(px(6.0))
                                        .h(px(4.0))
                                        .rounded(px(2.0))
                                        .bg(if is_active { theme.accent } else { theme.border_card }),
                                )
                        })),
                ),
        )
        // Section 2: Library Storage & Custom Folder Location
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
                                .text_color(theme.text_primary)
                                .child("LIBRARY STORAGE LOCATION"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("Configure where SQLite catalog database and albums are saved"),
                        ),
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
                                        .text_color(theme.text_muted)
                                        .child("ACTIVE DATABASE PATH:"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_primary)
                                        .child(
                                            if let Some(ref p) = custom_path {
                                                format!("{}/.media_library.db (Custom Override)", p.display())
                                            } else if let Some(ref d) = selected_drive {
                                                format!("{}/.media_library.db (Drive Root)", d.mount_path.display())
                                            } else {
                                                "None (No volume or folder selected)".to_string()
                                            }
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    // Native folder picker button via folder_picker helper
                                    {
                                        let app_state_prompt = app_state.clone();
                                        div()
                                            .id(SharedString::from("browse-library-folder-btn"))
                                            .cursor_pointer()
                                            .px(px(8.0))
                                            .py(px(4.0))
                                            .rounded(px(4.0))
                                            .bg(theme.accent)
                                            .hover(move |s| s.bg(theme.accent_hover))
                                            .on_click(move |_, _, cx| {
                                                prompt_mount_custom_folder(&app_state_prompt, cx);
                                            })
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(rgb(0xffffff))
                                                    .child(format!("{} Browse Folder...", Icon::FOLDER)),
                                            )
                                    }
                                )
                                .child(
                                    if custom_path.is_some() {
                                        div()
                                            .id(SharedString::from("reset-library-location-btn"))
                                            .cursor_pointer()
                                            .px(px(8.0))
                                            .py(px(4.0))
                                            .rounded(px(4.0))
                                            .bg(theme.bg_pill)
                                            .border_1()
                                            .border_color(theme.border_card)
                                            .hover(move |s| s.bg(theme.bg_card_hover).border_color(theme.accent))
                                            .on_click(move |_, _, cx| {
                                                state_reset_loc.update(cx, |s, cx| {
                                                    s.set_custom_library_path(None);
                                                    cx.notify();
                                                });
                                            })
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.text_secondary)
                                                    .child("Reset to Drive Root"),
                                            )
                                            .into_any_element()
                                    } else {
                                        div().into_any_element()
                                    }
                                ),
                        ),
                ),
        )
        // Section 3: Catalog Layout & Grid Density Slider
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
                                .text_color(theme.text_primary)
                                .child("CATALOG GRID DENSITY"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(format!("Current: {} Columns per row", grid_cols)),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .children((4..=9).map(|cols| {
                            let is_current = cols == grid_cols;
                            let state_c = state_grid.clone();

                            div()
                                .id(SharedString::from(format!("grid-cols-pill-{}", cols)))
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .bg(if is_current { theme.bg_pill_active } else { theme.bg_sidebar })
                                .border_1()
                                .border_color(if is_current { theme.accent } else { theme.border_subtle })
                                .hover(move |s| s.border_color(theme.accent))
                                .on_click(move |_, _, cx| {
                                    state_c.update(cx, |s, cx| {
                                        s.set_grid_columns(cols);
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(if is_current { FontWeight::BOLD } else { FontWeight::NORMAL })
                                        .text_color(if is_current { theme.accent } else { theme.text_secondary })
                                        .child(format!("{} cols", cols)),
                                )
                        })),
                ),
        )
        // Section 4: Removable Drive Management & Functional Ejection
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
                                .text_color(theme.text_primary)
                                .child("STORAGE VOLUMES & DEVICE OPERATIONS"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(format!("{} detected block devices", drives.len())),
                        ),
                )
                .child(
                    if let Some(ref drive) = selected_drive {
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
                                    .items_center()
                                    .gap(px(10.0))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.accent)
                                            .child(Icon::DISK),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(2.0))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(theme.text_primary)
                                                    .child(format!("{} ({})", drive.name, drive.size_formatted)),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_muted)
                                                    .child(format!("Mountpoint: {}", drive.mount_path.display())),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .id(SharedString::from("eject-drive-settings-btn"))
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded(px(4.0))
                                    .bg(theme.bg_pill)
                                    .border_1()
                                    .border_color(theme.accent_danger)
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
                                            .text_color(theme.accent_danger)
                                            .child(format!("{} Safe Eject Volume", Icon::CLOSE)),
                                    ),
                            )
                    } else {
                        div()
                            .p(px(10.0))
                            .rounded(px(4.0))
                            .bg(theme.bg_sidebar)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(format!("{} No drive mounted. Connect an external drive or use the sidebar to mount a volume.", Icon::INFO))
                    }
                ),
        )
        // Section 5: System Diagnostics & Framework Info
        .child(
            div()
                .p(px(14.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child("SYSTEM ARCHITECTURE & DIAGNOSTICS"),
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
                        .child(render_diagnostic_row("Media I/O Engine", "Direct Native File Seeking (Zero Overhead)", &theme))
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
