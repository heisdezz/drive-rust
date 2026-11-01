use gpui_kit::*;
use crate::drives::detector::list_drives;
use crate::state::app_state::{AppRoute, AppState};
use crate::components::drive_card::render_drive_card;
use crate::components::folder_picker::prompt_mount_custom_folder;
use crate::state::models::DriveType;
use crate::theme::icons::Icon;

const NAV_ITEMS: &[(AppRoute, &str, &str)] = &[
    (AppRoute::Dashboard, "Dashboard", Icon::HOME),
    (AppRoute::Discover, "Discover", Icon::DISCOVER),
    (AppRoute::Medias, "Media Library", Icon::PLAY),
    (AppRoute::Albums, "Albums", Icon::ALBUMS),
    (AppRoute::StateCounter, "State Counter", Icon::STATS),
    (AppRoute::About, "System Info", Icon::MONITOR),
    (AppRoute::Settings, "Settings", Icon::SETTINGS),
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

    let state_refresh = app_state.clone();
    let state_mount = app_state.clone();

    div()
        .id(SharedString::from("app-sidebar-container"))
        .w(px(236.0))
        .h_full()
        .bg(theme.bg_sidebar)
        .border_r_1()
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .justify_between()
        .p(px(10.0))
        .overflow_y_scroll()
        // Top Section: Brand Header & Application Nav
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(14.0))
                // Brand Logo Header: D-Stream
                .child(
                    div()
                        .px(px(4.0))
                        .py(px(4.0))
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .child(
                            div()
                                .w(px(28.0))
                                .h(px(28.0))
                                .rounded(px(6.0))
                                .bg(theme.accent)
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .child(Icon::PLAY),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .child(
                                    div()
                                        .text_base()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("D-"),
                                )
                                .child(
                                    div()
                                        .text_base()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child("Stream"),
                                ),
                        ),
                )
                // APPLICATION Group
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .px(px(6.0))
                                .py(px(2.0))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("APPLICATION"),
                        )
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
                                        .relative()
                                        .rounded(px(6.0))
                                        .bg(if is_active { theme.bg_card } else { rgba(0x00000000) })
                                        .hover(move |s| {
                                            if !is_active {
                                                s.bg(theme.bg_card_hover)
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
                                        // Left accent vertical indicator bar when active
                                        .child(
                                            if is_active {
                                                div()
                                                    .absolute()
                                                    .left(px(0.0))
                                                    .top(px(4.0))
                                                    .bottom(px(4.0))
                                                    .w(px(3.0))
                                                    .rounded_r(px(1.5))
                                                    .bg(theme.accent)
                                                    .into_any_element()
                                            } else {
                                                div().into_any_element()
                                            }
                                        )
                                        // Nav item content
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(10.0))
                                                .px(px(10.0))
                                                .py(px(6.0))
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(if is_active { theme.accent } else { theme.text_muted })
                                                        .child(*icon),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                                                        .text_color(if is_active { theme.text_primary } else { theme.text_secondary })
                                                        .child(*label),
                                                ),
                                        )
                                })),
                        ),
                ),
        )
        // Bottom Section: Connected Drives & Mount Button
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
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
                                .child("CONNECTED DRIVES"),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .id(SharedString::from("refresh-drives-btn"))
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .hover(move |s| s.text_color(theme.accent))
                                        .on_click(move |_, _, cx| {
                                            state_refresh.update(cx, |s, cx| {
                                                let detected = list_drives();
                                                s.set_drives(detected);
                                                cx.notify();
                                            });
                                        })
                                        .child(Icon::REFRESH),
                                )
                                .child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(1.0))
                                        .rounded(px(8.0))
                                        .bg(theme.accent)
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xffffff))
                                        .child(format!("{}", drives.len())),
                                ),
                        ),
                )
                // Drive Cards list
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .children(
                            if drives.is_empty() {
                                vec![
                                    div()
                                        .p(px(8.0))
                                        .rounded(px(4.0))
                                        .bg(theme.bg_card)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("No drives detected"),
                                        )
                                        .into_any_element(),
                                ]
                            } else {
                                drives.into_iter().map(|drive| {
                                    let is_selected = selected_id.as_ref() == Some(&drive.id);
                                    let state_clone = app_state.clone();
                                    let d = drive.clone();
                                    let is_sys = drive.drive_type == DriveType::Internal || drive.name == "System Root";

                                    div()
                                        .id(SharedString::from(format!("drive-item-{}", drive.id)))
                                        .child(render_drive_card(
                                            &drive,
                                            is_selected,
                                            &theme,
                                            move |_, _, cx| {
                                                if !is_sys {
                                                    let drive_to_select = d.clone();
                                                    state_clone.update(cx, |s, cx| {
                                                        s.select_drive(drive_to_select);
                                                        cx.notify();
                                                    });
                                                }
                                            },
                                        ))
                                        .into_any_element()
                                }).collect::<Vec<_>>()
                            }
                        ),
                )
                // Mount Folder as Drive Button
                .child(
                    div()
                        .id(SharedString::from("mount-folder-btn"))
                        .cursor_pointer()
                        .w_full()
                        .py(px(6.0))
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.accent)
                        .hover(move |s| s.bg(theme.bg_card_hover))
                        .on_click(move |_, _, cx| {
                            prompt_mount_custom_folder(&state_mount, cx);
                        })
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap(px(6.0))
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.accent)
                                .child(Icon::PLUS),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.accent)
                                .child("Mount Drive"),
                        ),
                ),
        )
}
