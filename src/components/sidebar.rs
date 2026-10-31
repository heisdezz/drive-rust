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

    div()
        .w(px(210.0))
        .h_full()
        .flex()
        .flex_col()
        .bg(rgb(0x111317)) // Zed-style dark charcoal sidebar
        .border_r_1()
        .border_color(rgb(0x1e2025)) // 1px subtle divider
        // Compact App Header (38px height)
        .child(
            div()
                .h(px(38.0))
                .px(px(10.0))
                .flex()
                .items_center()
                .gap(px(8.0))
                .border_b_1()
                .border_color(rgb(0x1e2025))
                .child(
                    div()
                        .w(px(18.0))
                        .h(px(18.0))
                        .rounded(px(3.5))
                        .bg(rgb(0x2563eb)) // Subtle vibrant blue
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgb(0xffffff))
                        .font_weight(FontWeight::BOLD)
                        .child("D"),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xe4e4e7))
                        .child("Rust Drive"),
                )
                .child(
                    div()
                        .ml_auto()
                        .px(px(4.0))
                        .py(px(0.5))
                        .rounded(px(3.0))
                        .bg(rgb(0x1c1e24))
                        .text_color(rgb(0x71717a))
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
                        rgb(0xffffff)
                    } else {
                        rgb(0xa1a1aa)
                    };

                    let bg_color = if is_active {
                        rgb(0x22262e) // Refined Zed active pill
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
                        .hover(|s| {
                            if !is_active {
                                s.bg(rgb(0x181a20)).text_color(rgb(0xe4e4e7))
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
                                .gap(px(7.0))
                                .child(
                                    div()
                                        .w(px(14.0))
                                        .flex()
                                        .justify_center()
                                        .text_color(if is_active { rgb(0x60a5fa) } else { rgb(0x71717a) })
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
        // Section Divider & Label
        .child(
            div()
                .px(px(10.0))
                .pt(px(10.0))
                .pb(px(4.0))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x52525b))
                        .child("VOLUMES"),
                )
                .child(
                    div()
                        .px(px(4.0))
                        .py(px(0.5))
                        .rounded(px(2.0))
                        .bg(rgb(0x181a20))
                        .text_xs()
                        .text_color(rgb(0x71717a))
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

                    render_drive_card(&drive, is_selected, move |_, _, cx| {
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
                .border_color(rgb(0x1e2025))
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .w(px(5.0))
                        .h(px(5.0))
                        .rounded(px(2.5))
                        .bg(if state.selected_drive.is_some() { rgb(0x34d399) } else { rgb(0xf59e0b) }),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x71717a))
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
