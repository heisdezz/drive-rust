use gpui_kit::*;
use crate::state::app_state::{AppRoute, AppState};
use crate::components::sidebar::render_sidebar;
use crate::components::dashboard_view::render_dashboard_view;
use crate::components::media_grid::render_media_grid;
use crate::components::album_grid::render_album_grid;
use crate::components::album_modal::{
    render_create_album_modal, render_delete_album_modal, render_edit_album_modal,
    render_merge_album_modal,
};
use crate::components::discover_view::render_discover_view;
use crate::components::media_player::render_media_player_modal;
use crate::components::move_modal::render_move_to_album_modal;
use crate::components::settings_view::render_settings_view;
use crate::components::video_test_modal::render_video_test_modal;

pub struct AppView {
    app_state: Entity<AppState>,
}

impl AppView {
    pub fn new(app_state: Entity<AppState>, cx: &mut Context<Self>) -> Self {
        // Observe AppState so changes in active_route, theme, or drives trigger re-renders
        cx.observe(&app_state, |_, _, cx| {
            cx.notify();
        }).detach();

        Self { app_state }
    }
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.app_state.read(cx);
        let active_route = state.active_route;
        let selected_drive = state.selected_drive.clone();
        let db_connected = state.active_db.is_some();
        let theme = state.theme;
        let show_create_album = state.show_create_album_modal;
        let show_edit_album = state.editing_album.is_some();
        let show_merge_album = state.merging_album.is_some();
        let show_delete_album = state.deleting_album.is_some();
        let show_move_modal = !state.moving_media_ids.is_empty();
        let show_media_player = state.viewing_media_item.is_some();
        let show_video_test = state.show_video_test_modal;

        let state_theme = self.app_state.clone();

        div()
            .flex()
            .w_full()
            .h_full()
            .bg(theme.bg_canvas)
            .text_color(theme.text_primary)
            // Left: Navigation & Drives Sidebar
            .child(render_sidebar(&self.app_state, window, cx))
            // Right: Main App Canvas
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_grow(1.0)
                    .h_full()
                    .overflow_hidden()
                    // Top App Header & Breadcrumbs (Zed 38px titlebar height)
                    .child(
                        div()
                            .h(px(38.0))
                            .px(px(14.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .bg(theme.bg_header)
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            // Left Breadcrumbs
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::NORMAL)
                                            .text_color(theme.text_muted)
                                            .child(
                                                selected_drive
                                                    .as_ref()
                                                    .map(|d| d.name.clone())
                                                    .unwrap_or_else(|| "No Drive".to_string()),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.border_card)
                                            .child("/"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_primary)
                                            .child(active_route.title()),
                                    ),
                            )
                            // Right: Theme Mode Quick Toggle & Status Indicators
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    // Quick Theme Cycle Button
                                    .child(
                                        div()
                                            .id(SharedString::from("quick-theme-toggle-btn"))
                                            .cursor_pointer()
                                            .flex()
                                            .items_center()
                                            .gap(px(4.0))
                                            .px(px(7.0))
                                            .py(px(2.0))
                                            .rounded(px(4.0))
                                            .bg(theme.bg_pill)
                                            .border_1()
                                            .border_color(theme.border_card)
                                            .hover(move |s| s.bg(theme.bg_pill_active).border_color(theme.border_active))
                                            .on_click(move |_, _, cx| {
                                                state_theme.update(cx, |s, cx| {
                                                    s.cycle_theme();
                                                    cx.notify();
                                                });
                                            })
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .child(theme.mode.icon()),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.text_secondary)
                                                    .child(theme.mode.name()),
                                            ),
                                    )
                                    // DB Status Indicator
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(5.0))
                                            .px(px(6.0))
                                            .py(px(2.0))
                                            .rounded(px(4.0))
                                            .bg(theme.bg_pill)
                                            .border_1()
                                            .border_color(theme.border_card)
                                            .child(
                                                div()
                                                    .w(px(5.0))
                                                    .h(px(5.0))
                                                    .rounded(px(2.5))
                                                    .bg(if db_connected {
                                                        theme.accent_success
                                                    } else {
                                                        theme.accent_warning
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::NORMAL)
                                                    .text_color(theme.text_muted)
                                                    .child(if db_connected {
                                                        "SQLite Ready"
                                                    } else {
                                                        "Drive Disconnected"
                                                    }),
                                            ),
                                    )
                                    // Compact System Status Dot
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(5.0))
                                            .px(px(6.0))
                                            .py(px(2.0))
                                            .rounded(px(4.0))
                                            .bg(theme.bg_pill)
                                            .border_1()
                                            .border_color(theme.border_card)
                                            .child(
                                                div()
                                                    .w(px(5.0))
                                                    .h(px(5.0))
                                                    .rounded(px(2.5))
                                                    .bg(theme.accent),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::NORMAL)
                                                    .text_color(theme.text_muted)
                                                    .child("Native GPUI"),
                                            ),
                                    ),
                            ),
                    )
                    // View Route Canvas (Compact padding)
                    .child(
                        div()
                            .flex_grow(1.0)
                            .p(px(14.0))
                            .overflow_y_hidden()
                            .child(render_route_content(&self.app_state, active_route, selected_drive.as_ref(), window, cx)),
                    ),
            )
            // Modals & Viewer Overlay Layer
            .child(
                if show_create_album {
                    div().child(render_create_album_modal(&self.app_state, window, cx))
                } else {
                    div()
                },
            )
            .child(
                if show_edit_album {
                    div().child(render_edit_album_modal(&self.app_state, window, cx))
                } else {
                    div()
                },
            )
            .child(
                if show_merge_album {
                    div().child(render_merge_album_modal(&self.app_state, window, cx))
                } else {
                    div()
                },
            )
            .child(
                if show_delete_album {
                    div().child(render_delete_album_modal(&self.app_state, window, cx))
                } else {
                    div()
                },
            )
            .child(
                if show_move_modal {
                    div().child(render_move_to_album_modal(&self.app_state, window, cx))
                } else {
                    div()
                },
            )
            .child(
                if show_media_player {
                    div().child(render_media_player_modal(&self.app_state, window, cx))
                } else {
                    div()
                },
            )
            .child(
                if show_video_test {
                    div().child(render_video_test_modal(&self.app_state, window, cx))
                } else {
                    div()
                },
            )
    }
}

/// Renders the content panel for the currently selected route.
fn render_route_content(
    app_state: &Entity<AppState>,
    route: AppRoute,
    selected_drive: Option<&crate::state::models::DriveInfo>,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;

    match route {
        AppRoute::Dashboard => div().w_full().h_full().child(render_dashboard_view(app_state, window, cx)),
        AppRoute::Discover => div().w_full().h_full().child(render_discover_view(app_state, window, cx)),
        AppRoute::Medias => div().w_full().h_full().child(render_media_grid(app_state, window, cx)),
        AppRoute::Albums => div().w_full().h_full().child(render_album_grid(app_state, window, cx)),
        AppRoute::Settings => div().w_full().h_full().child(render_settings_view(app_state, window, cx)),
        _ => div()
            .flex()
            .flex_col()
            .gap(px(14.0))
            .child(
                div()
                    .p(px(14.0))
                    .rounded(px(6.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border_card)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.text_primary)
                            .child(format!("Welcome to {}", route.title())),
                    )
                    .child(
                        div()
                            .mt(px(4.0))
                            .text_xs()
                            .text_color(theme.text_secondary)
                            .child(
                                if let Some(drive) = selected_drive {
                                    format!("Active Storage Volume: {} ({:?}) mounted at {}", drive.name, drive.drive_type, drive.mount_path.display())
                                } else {
                                    "No removable storage drive selected. Connect a drive or select one from the sidebar.".to_string()
                                }
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap(px(10.0))
                    .child(
                        div()
                            .flex_1()
                            .p(px(12.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_sidebar)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.text_muted)
                                    .child("PORT STATUS"),
                            )
                            .child(
                                div()
                                    .mt(px(4.0))
                                    .text_xs()
                                    .text_color(theme.accent_success)
                                    .child("100% Native Rust & GPUI (No WebKit/V8)"),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .p(px(12.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_sidebar)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.text_muted)
                                    .child("DATABASE ENGINE"),
                            )
                            .child(
                                div()
                                    .mt(px(4.0))
                                    .text_xs()
                                    .text_color(theme.accent)
                                    .child("Embedded SQLite via rusqlite (WAL mode)"),
                            ),
                    ),
            ),
    }
}
