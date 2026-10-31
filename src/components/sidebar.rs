use gpui_kit::*;
use crate::state::app_state::{AppRoute, AppState};
use crate::components::drive_card::render_drive_card;

const NAV_ITEMS: &[(AppRoute, &str, &str)] = &[
    (AppRoute::Dashboard, "Dashboard", "◫"),
    (AppRoute::Discover, "Discover", "🧭"),
    (AppRoute::Medias, "Media Library", "🖼"),
    (AppRoute::Albums, "Albums", "📁"),
    (AppRoute::Settings, "Settings", "⚙"),
    (AppRoute::About, "System Info", "ⓘ"),
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
    let theme = state.theme;

    div()
        .w(px(210.0))
        .h_full()
        .flex()
        .flex_col()
        .bg(theme.bg_sidebar)
        .border_r_1()
        .border_color(theme.border_subtle)
        // Compact App Header (38px height)
        .child(
            div()
                .h(px(38.0))
                .px(px(10.0))
                .flex()
                .items_center()
                .gap(px(8.0))
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .w(px(18.0))
                        .h(px(18.0))
                        .rounded(px(3.5))
                        .bg(theme.accent)
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(if theme.is_dark { rgb(0xffffff) } else { rgb(0xffffff) })
                        .font_weight(FontWeight::BOLD)
                        .child("D"),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_primary)
                        .child("Rust Drive"),
                )
                .child(
                    div()
                        .ml_auto()
                        .px(px(4.0))
                        .py(px(0.5))
                        .rounded(px(3.0))
                        .bg(theme.bg_pill)
                        .text_color(theme.text_muted)
                        .child("v0.1"),
                ),
        )
        // Navigation Section (Tight Zed file-tree style)
        .child(
            div()
                .px(px(6.0))
                .py(px(6.0))
                .flex()
                .flex_col()
                .gap(px(1.5))
                .children(NAV_ITEMS.iter().map(|&(route, label, icon)| {
                    let is_active = active_route == route;
                    let state_handle = app_state.clone();

                    let text_color = if is_active {
                        theme.text_primary
                    } else {
                        theme.text_secondary
                    };

                    let bg_color = if is_active {
                        theme.bg_pill_active
                    } else {
                        rgba(0x00000000)
                    };

                    div()
                        .id(SharedString::from(format!("nav-item-{}", label)))
                        .cursor_pointer()
                        .px(px(7.0))
                        .py(px(4.5))
                        .rounded(px(4.0))
                        .bg(bg_color)
                        .hover(move |s| {
                            if !is_active {
                                s.bg(theme.bg_card_hover).text_color(theme.text_primary)
                            } else {
                                s
                            }
                        })
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
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .w(px(14.0))
                                        .text_xs()
                                        .text_color(if is_active { theme.accent } else { theme.text_muted })
                                        .child(icon),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(if is_active {
                                            FontWeight::MEDIUM
                                        } else {
                                            FontWeight::NORMAL
                                        })
                                        .text_color(text_color)
                                        .child(label),
                                ),
                        )
                })),
        )
        // Divider
        .child(
            div()
                .mx(px(8.0))
                .my(px(4.0))
                .h(px(1.0))
                .bg(theme.border_subtle),
        )
        // Volumes Section Header
        .child(
            div()
                .px(px(10.0))
                .py(px(4.0))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_muted)
                        .child("VOLUMES"),
                )
                .child(
                    div()
                        .px(px(4.0))
                        .py(px(0.5))
                        .rounded(px(2.0))
                        .bg(theme.bg_pill)
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(format!("{}", drives.len())),
                ),
        )
        // Storage Volumes List
        .child(
            div()
                .flex_grow(1.0)
                .px(px(6.0))
                .py(px(2.0))
                .overflow_y_hidden()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .children(drives.into_iter().map(|drive| {
                    let is_selected = selected_id.as_ref() == Some(&drive.id);
                    let state_handle = app_state.clone();
                    let drive_to_select = drive.clone();

                    render_drive_card(&drive, is_selected, &theme, move |_, _, cx| {
                        state_handle.update(cx, |state, cx| {
                            state.select_drive(drive_to_select.clone());
                            cx.notify();
                        });
                    })
                })),
        )
        // Footer: Active Volume Status (Compact 30px)
        .child(
            div()
                .px(px(10.0))
                .py(px(7.0))
                .border_t_1()
                .border_color(theme.border_subtle)
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .w(px(5.0))
                        .h(px(5.0))
                        .rounded(px(2.5))
                        .bg(if state.selected_drive.is_some() { theme.accent_success } else { theme.accent_warning }),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .overflow_hidden()
                        .child(
                            state
                                .selected_drive
                                .as_ref()
                                .map(|d| d.name.clone())
                                .unwrap_or_else(|| "No Drive".to_string()),
                        ),
                ),
        )
}
