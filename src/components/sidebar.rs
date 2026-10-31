use gpui_kit::*;
use crate::state::app_state::{AppRoute, AppState};
use crate::components::drive_card::render_drive_card;

const NAV_ITEMS: &[(AppRoute, &str, &str)] = &[
    (AppRoute::Dashboard, "Dashboard", "📊"),
    (AppRoute::Discover, "Discover", "🧭"),
    (AppRoute::Medias, "Media Library", "🖼️"),
    (AppRoute::Albums, "Albums", "📁"),
    (AppRoute::Settings, "Settings", "⚙️"),
    (AppRoute::About, "System Info", "ℹ️"),
];

pub fn render_sidebar(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let active_route = state.active_route;
    let drives = state.drives.clone();
    let selected_id = state.selected_drive.as_ref().map(|d| d.id.clone());

    div()
        .w(px(260.0))
        .h_full()
        .flex()
        .flex_col()
        .bg(rgb(0x0f172a)) // Deep slate-900
        .border_r_1()
        .border_color(rgb(0x1e293b)) // Slate-800 border
        // App Header
        .child(
            div()
                .h(px(64.0))
                .px(px(16.0))
                .flex()
                .items_center()
                .gap(px(10.0))
                .border_b_1()
                .border_color(rgb(0x1e293b))
                .child(
                    div()
                        .w(px(32.0))
                        .h(px(32.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x3b82f6))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgb(0xffffff))
                        .font_weight(FontWeight::BOLD)
                        .child("RD"),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xf8fafc))
                                .child("Rust Drive"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x64748b))
                                .child("Media Organizer"),
                        ),
                ),
        )
        // Navigation Section
        .child(
            div()
                .px(px(12.0))
                .py(px(16.0))
                .flex()
                .flex_col()
                .gap(px(4.0))
                .children(NAV_ITEMS.iter().map(|&(route, label, icon)| {
                    let is_active = active_route == route;
                    let state_handle = app_state.clone();

                    let text_color = if is_active {
                        rgb(0xf8fafc)
                    } else {
                        rgb(0x94a3b8)
                    };

                    let bg_color = if is_active {
                        rgb(0x1e293b)
                    } else {
                        rgba(0x00000000)
                    };

                    let border_color = if is_active {
                        rgb(0x3b82f6)
                    } else {
                        rgba(0x00000000)
                    };

                    div()
                        .id(SharedString::from(format!("nav-item-{}", label)))
                        .cursor_pointer()
                        .px(px(12.0))
                        .py(px(8.0))
                        .rounded(px(6.0))
                        .border_l_2()
                        .border_color(border_color)
                        .bg(bg_color)
                        .hover(|s| s.bg(rgb(0x1e293b)).text_color(rgb(0xf8fafc)))
                        .on_click(move |_, _, cx| {
                            state_handle.update(cx, |state, cx| {
                                state.set_route(route);
                                cx.notify();
                            });
                        })
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(10.0))
                                .child(div().child(icon))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(if is_active {
                                            FontWeight::BOLD
                                        } else {
                                            FontWeight::MEDIUM
                                        })
                                        .text_color(text_color)
                                        .child(label),
                                ),
                        )
                })),
        )
        // Storage Volumes Header
        .child(
            div()
                .px(px(16.0))
                .pt(px(12.0))
                .pb(px(6.0))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x64748b))
                        .child("STORAGE DRIVES"),
                )
                .child(
                    div()
                        .px(px(6.0))
                        .py(px(1.0))
                        .rounded(px(10.0))
                        .bg(rgb(0x1e293b))
                        .text_xs()
                        .text_color(rgb(0x94a3b8))
                        .child(format!("{}", drives.len())),
                ),
        )
        // Storage Volumes List
        .child(
            div()
                .flex_grow(1.0)
                .px(px(12.0))
                .py(px(4.0))
                .overflow_y_hidden()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .children(drives.into_iter().map(|drive| {
                    let is_selected = selected_id.as_ref() == Some(&drive.id);
                    let state_handle = app_state.clone();
                    let drive_to_select = drive.clone();

                    render_drive_card(&drive, is_selected, move |_, _, cx| {
                        state_handle.update(cx, |state, cx| {
                            state.select_drive(drive_to_select.clone());
                            cx.notify();
                        });
                    })
                })),
        )
        // Footer: Selected Drive Details
        .child(
            div()
                .px(px(14.0))
                .py(px(12.0))
                .border_t_1()
                .border_color(rgb(0x1e293b))
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(
                    div()
                        .w(px(8.0))
                        .h(px(8.0))
                        .rounded(px(4.0))
                        .bg(rgb(0x10b981)), // Green dot
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(0xe2e8f0))
                                .child("Active Storage Volume"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x64748b))
                                .overflow_hidden()
                                .child(
                                    state
                                        .selected_drive
                                        .as_ref()
                                        .map(|d| d.mount_path.to_string_lossy().to_string())
                                        .unwrap_or_else(|| "No Drive Selected".to_string()),
                                ),
                        ),
                ),
        )
}
