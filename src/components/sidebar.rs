use gpui_kit::*;
use crate::state::app_state::{AppRoute, AppState};
use crate::components::drive_card::render_drive_card;

const NAV_ITEMS: &[(AppRoute, &str, &str)] = &[
    (AppRoute::Dashboard, "Dashboard", "⊞"),
    (AppRoute::Discover, "Discover", "⌕"),
    (AppRoute::Medias, "Media Library", "▣"),
    (AppRoute::Albums, "Albums", "◫"),
    (AppRoute::Settings, "Settings", "⚙"),
    (AppRoute::About, "System Info", "ℹ"),
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
        .w(px(220.0))
        .h_full()
        .bg(theme.bg_sidebar)
        .border_r_1()
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .justify_between()
        .p(px(8.0))
        // Top Section: App Title & Nav Links
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(14.0))
                // App Logo / Title
                .child(
                    div()
                        .px(px(6.0))
                        .py(px(4.0))
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .w(px(20.0))
                                .h(px(20.0))
                                .rounded(px(4.0))
                                .bg(theme.accent)
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .child("RD"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("Rust Drive"),
                        ),
                )
                // Navigation Link Group
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .children(NAV_ITEMS.iter().map(|(route, label, icon)| {
                            let is_active = active_route == *route;
                            let target_route = *route;
                            let state_clone = app_state.clone();

                            div()
                                .id(SharedString::from(format!("nav-item-{}", label)))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(4.5))
                                .rounded(px(4.0))
                                .bg(if is_active { theme.bg_pill_active } else { rgba(0x00000000) })
                                .text_color(if is_active { theme.text_primary } else { theme.text_secondary })
                                .hover(move |s| {
                                    if !is_active {
                                        s.bg(theme.bg_card_hover).text_color(theme.text_primary)
                                    } else {
                                        s
                                    }
                                })
                                .on_click(move |_, _, cx| {
                                    state_clone.update(cx, |s, cx| {
                                        s.set_route(target_route);
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
                                                .text_xs()
                                                .font_weight(FontWeight::NORMAL)
                                                .text_color(if is_active { theme.accent } else { theme.text_muted })
                                                .child(*icon),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                                                .child(*label),
                                        ),
                                )
                        })),
                ),
        )
        // Bottom Section: Detected Removable Drives
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .px(px(6.0))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("VOLUMES"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(format!("{}", drives.len())),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .children(
                            if drives.is_empty() {
                                vec![
                                    div()
                                        .px(px(6.0))
                                        .py(px(8.0))
                                        .rounded(px(4.0))
                                        .bg(theme.bg_card)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("No removable drives"),
                                        )
                                        .into_any_element(),
                                ]
                            } else {
                                drives.into_iter().map(|drive| {
                                    let is_selected = selected_id.as_ref() == Some(&drive.id);
                                    let state_clone = app_state.clone();
                                    let d = drive.clone();

                                    div()
                                        .id(SharedString::from(format!("drive-item-{}", drive.id)))
                                        .child(render_drive_card(
                                            &drive,
                                            is_selected,
                                            &theme,
                                            move |_, _, cx| {
                                                let drive_to_select = d.clone();
                                                state_clone.update(cx, |s, cx| {
                                                    s.select_drive(drive_to_select);
                                                    cx.notify();
                                                });
                                            },
                                        ))
                                        .into_any_element()
                                }).collect::<Vec<_>>()
                            }
                        ),
                ),
        )
}
