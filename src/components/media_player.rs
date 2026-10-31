use crate::theme::icons::Icon;
use gpui_kit::*;
use std::path::Path;
use crate::drives::detector::format_bytes;
use crate::state::app_state::AppState;
use crate::components::media_card::format_duration;

/// Renders the full-screen / large window Media Player modal with preview, neighbor navigation, and EXIF drawer.
pub fn render_media_player_modal(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let viewing_item = state.viewing_media_item.clone();
    let db_opt = state.active_db.clone();
    let drive_opt = state.selected_drive.clone();
    let show_metadata = state.player_show_metadata;
    let prev_id = state.player_prev_id;
    let next_id = state.player_next_id;
    let zoom_fit = state.player_zoom_fit;
    let state_zoom = app_state.clone();

    let item = match viewing_item {
        Some(i) => i,
        None => return div().into_any_element(),
    };

    let item_id = item.id;
    let is_video = item.mime_type.starts_with("video/");
    let filename = Path::new(&item.current_relative_path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| item.current_relative_path.clone());

    let size_str = format_bytes(item.file_size as u64);
    let disk_path = drive_opt
        .as_ref()
        .map(|d| d.mount_path.join(&item.current_relative_path).to_string_lossy().to_string())
        .unwrap_or_else(|| item.current_relative_path.clone());

    let state_close = app_state.clone();
    let state_meta_toggle = app_state.clone();
    let state_prev = app_state.clone();
    let state_next = app_state.clone();
    let state_move = app_state.clone();
    let state_delete = app_state.clone();

    let db_for_prev = db_opt.clone();
    let db_for_next = db_opt.clone();
    let db_for_del = db_opt.clone();
    let selected_album_id = state.selected_album_id;

    div()
        .id(SharedString::from("media-player-overlay"))
        .absolute()
        .inset_0()
        .bg(rgba(0x000000dd))
        .flex()
        .flex_col()
        // Top Header Bar (Zed 38px compact)
        .child(
            div()
                .h(px(38.0))
                .px(px(14.0))
                .bg(theme.bg_header)
                .border_b_1()
                .border_color(theme.border_subtle)
                .flex()
                .items_center()
                .justify_between()
                // Left: Close button and Title
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .child(
                            div()
                                .id(SharedString::from("player-close-btn"))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_secondary)
                                .hover(move |s| s.bg(theme.bg_card_hover).text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_close.update(cx, |s, cx| {
                                        s.close_player();
                                        cx.notify();
                                    });
                                })
                                .child(format!("{} Close", Icon::CLOSE)),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.text_primary)
                                .child(filename.clone()),
                        )
                        .child(
                            div()
                                .px(px(5.0))
                                .py(px(1.0))
                                .rounded(px(3.0))
                                .bg(theme.bg_pill)
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(if is_video { "VIDEO" } else { "IMAGE" }),
                        ),
                )
                // Right: Action buttons (Move, Delete, Toggle Metadata)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        // Move to album button
                        .child(
                            div()
                                .id(SharedString::from("player-move-btn"))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(3.5))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .text_xs()
                                .text_color(theme.text_secondary)
                                .hover(move |s| s.bg(theme.bg_card_hover).text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_move.update(cx, |s, cx| {
                                        s.open_move_modal(vec![item_id]);
                                        cx.notify();
                                    });
                                })
                                .child("Move to Album"),
                        )
                        // Delete item button
                        .child(
                            div()
                                .id(SharedString::from("player-delete-btn"))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(3.5))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .text_xs()
                                .text_color(theme.accent_danger)
                                .hover(move |s| s.bg(theme.accent_danger).text_color(rgb(0xffffff)))
                                .on_click(move |_, _, cx| {
                                    if let Some(db) = &db_for_del {
                                        let _ = db.delete_media_items(&[item_id]);
                                    }
                                    state_delete.update(cx, |s, cx| {
                                        s.close_player();
                                        cx.notify();
                                    });
                                })
                                .child("Delete"),
                        )
                        // Zoom toggle button (if image)
                        .child(if !is_video {
                            div()
                                .id(SharedString::from("player-zoom-btn"))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(3.5))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .text_xs()
                                .text_color(theme.text_secondary)
                                .hover(move |s| s.bg(theme.bg_card_hover).text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_zoom.update(cx, |s, cx| {
                                        s.toggle_player_zoom_fit();
                                        cx.notify();
                                    });
                                })
                                .child(if zoom_fit { "100% Zoom" } else { "Fit Window" })
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        })
                        // Toggle info button
                        .child(
                            div()
                                .id(SharedString::from("player-toggle-info-btn"))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(3.5))
                                .rounded(px(4.0))
                                .bg(if show_metadata { theme.bg_pill_active } else { theme.bg_pill })
                                .border_1()
                                .border_color(if show_metadata { theme.accent } else { theme.border_card })
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(if show_metadata { theme.accent } else { theme.text_secondary })
                                .hover(move |s| s.bg(theme.bg_pill_active))
                                .on_click(move |_, _, cx| {
                                    state_meta_toggle.update(cx, |s, cx| {
                                        s.toggle_player_metadata();
                                        cx.notify();
                                    });
                                })
                                .child("Info"),
                        ),
                ),
        )
        // Center Stage: Media Display + Navigation Arrows + Collapsible Metadata Drawer
        .child(
            div()
                .flex_grow(1.0)
                .flex()
                .overflow_hidden()
                // Left Navigation Arrow Button
                .child(
                    div()
                        .w(px(60.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            if let Some(pid) = prev_id {
                                div()
                                    .id(SharedString::from("player-nav-prev-btn"))
                                    .cursor_pointer()
                                    .w(px(40.0))
                                    .h(px(40.0))
                                    .rounded(px(20.0))
                                    .bg(rgba(0x00000066))
                                    .border_1()
                                    .border_color(rgba(0xffffff22))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_sm()
                                    .text_color(rgb(0xffffff))
                                    .hover(move |s| s.bg(rgba(0x000000aa)).border_color(theme.accent))
                                    .on_click(move |_, _, cx| {
                                        if let Some(ref db) = db_for_prev {
                                            if let Ok(Some((prev_item, p_prev, p_next))) = db.get_media_item_with_neighbors(pid, selected_album_id) {
                                                state_prev.update(cx, |s, cx| {
                                                    s.open_player(prev_item, p_prev, p_next);
                                                    cx.notify();
                                                });
                                            }
                                        }
                                    })
                                    .child(Icon::CHEVRON_LEFT)
                            } else {
                                div()
                                    .id(SharedString::from("player-nav-prev-disabled"))
                                    .w(px(40.0))
                                    .h(px(40.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_sm()
                                    .text_color(rgba(0xffffff33))
                                    .child(Icon::CHEVRON_LEFT)
                            },
                        ),
                )
                // Center Viewer
                .child(
                    div()
                        .flex_grow(1.0)
                        .h_full()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .p(px(20.0))
                        .child(
                            if is_video {
                                // Video Player Canvas Mockup
                                div()
                                    .w_full()
                                    .max_w(px(720.0))
                                    .h(px(420.0))
                                    .rounded(px(8.0))
                                    .bg(rgba(0x0a0f18ee))
                                    .border_1()
                                    .border_color(theme.border_card)
                                    .flex()
                                    .flex_col()
                                    .justify_between()
                                    .p(px(16.0))
                                    .child(
                                        div()
                                            .flex_grow(1.0)
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .justify_center()
                                            .gap(px(12.0))
                                            .child(
                                                div()
                                                    .w(px(64.0))
                                                    .h(px(64.0))
                                                    .rounded(px(32.0))
                                                    .bg(theme.accent)
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_xl()
                                                    .text_color(rgb(0xffffff))
                                                    .child(Icon::PLAY),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.text_secondary)
                                                    .child("Native Video Player"),
                                            ),
                                    )
                                    // Video Controls Bar
                                    .child(
                                        div()
                                            .h(px(36.0))
                                            .px(px(12.0))
                                            .rounded(px(4.0))
                                            .bg(rgba(0x00000088))
                                            .border_1()
                                            .border_color(rgba(0xffffff15))
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(10.0))
                                                    .child(div().text_xs().text_color(rgb(0xffffff)).child(Icon::PLAY))
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(theme.text_muted)
                                                            .child(format!(
                                                                "00:00 / {}",
                                                                item.duration_seconds.map(format_duration).unwrap_or_else(|| "00:00".to_string())
                                                            )),
                                                    ),
                                            )
                                            // Seeker timeline bar
                                            .child(
                                                div()
                                                    .w(px(260.0))
                                                    .h(px(4.0))
                                                    .rounded(px(2.0))
                                                    .bg(theme.bg_pill)
                                                    .child(
                                                        div()
                                                            .w(px(80.0))
                                                            .h(px(4.0))
                                                            .rounded(px(2.0))
                                                            .bg(theme.accent),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(8.0))
                                                    .child(div().text_xs().text_color(theme.text_muted).child("VOL"))
                                                    .child(div().text_xs().text_color(theme.text_muted).child("EXP")),
                                            ),
                                    )
                            } else {
                                // Image Viewer Canvas Mockup
                                div()
                                    .w_full()
                                    .max_w(px(720.0))
                                    .h(px(440.0))
                                    .rounded(px(8.0))
                                    .bg(theme.bg_card)
                                    .border_1()
                                    .border_color(theme.border_active)
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .gap(px(16.0))
                                    .p(px(20.0))
                                    .child(
                                        div()
                                            .text_2xl()
                                            .text_color(theme.text_secondary)
                                            .child("▣"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(filename.clone()),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .px(px(8.0))
                                                    .py(px(3.0))
                                                    .rounded(px(4.0))
                                                    .bg(theme.bg_pill)
                                                    .border_1()
                                                    .border_color(theme.border_card)
                                                    .text_xs()
                                                    .text_color(theme.text_secondary)
                                                    .child(size_str.clone()),
                                            )
                                            .child(
                                                div()
                                                    .px(px(8.0))
                                                    .py(px(3.0))
                                                    .rounded(px(4.0))
                                                    .bg(theme.bg_pill)
                                                    .border_1()
                                                    .border_color(theme.border_card)
                                                    .text_xs()
                                                    .text_color(theme.text_secondary)
                                                    .child(item.mime_type.clone()),
                                            ),
                                    )
                            },
                        ),
                )
                // Right Navigation Arrow Button
                .child(
                    div()
                        .w(px(60.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            if let Some(nid) = next_id {
                                div()
                                    .id(SharedString::from("player-nav-next-btn"))
                                    .cursor_pointer()
                                    .w(px(40.0))
                                    .h(px(40.0))
                                    .rounded(px(20.0))
                                    .bg(rgba(0x00000066))
                                    .border_1()
                                    .border_color(rgba(0xffffff22))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_sm()
                                    .text_color(rgb(0xffffff))
                                    .hover(move |s| s.bg(rgba(0x000000aa)).border_color(theme.accent))
                                    .on_click(move |_, _, cx| {
                                        if let Some(ref db) = db_for_next {
                                            if let Ok(Some((next_item, n_prev, n_next))) = db.get_media_item_with_neighbors(nid, selected_album_id) {
                                                state_next.update(cx, |s, cx| {
                                                    s.open_player(next_item, n_prev, n_next);
                                                    cx.notify();
                                                });
                                            }
                                        }
                                    })
                                    .child(Icon::CHEVRON_RIGHT)
                            } else {
                                div()
                                    .id(SharedString::from("player-nav-next-disabled"))
                                    .w(px(40.0))
                                    .h(px(40.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_sm()
                                    .text_color(rgba(0xffffff33))
                                    .child(Icon::CHEVRON_RIGHT)
                            },
                        ),
                )
                // Collapsible EXIF Metadata Drawer
                .child(
                    if show_metadata {
                        div()
                            .id(SharedString::from("player-metadata-drawer"))
                            .w(px(280.0))
                            .h_full()
                            .bg(theme.bg_sidebar)
                            .border_l_1()
                            .border_color(theme.border_subtle)
                            .p(px(14.0))
                            .overflow_y_scroll()
                            .flex()
                            .flex_col()
                            .gap(px(14.0))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child("METADATA & EXIF"),
                            )
                            // Filename & Relative Path
                            .child(render_meta_field("File Name", &filename, &theme))
                            .child(render_meta_field("Album Path", &item.current_relative_path, &theme))
                            .child(render_meta_field("Original Source", &item.original_relative_path, &theme))
                            .child(render_meta_field("File Size", &size_str, &theme))
                            .child(render_meta_field("MIME Type", &item.mime_type, &theme))
                            .child(render_meta_field("Date Cataloged", &item.created_at, &theme))
                            .child(render_meta_field("Full Disk Path", &disk_path, &theme))
                            .child(render_meta_field("SHA-256 Hash", &item.file_hash, &theme))
                    } else {
                        div().id(SharedString::from("player-metadata-hidden"))
                    },
                ),
        )
        .into_any_element()
}

fn render_meta_field(label: &str, val: &str, theme: &crate::theme::AppTheme) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.text_muted)
                .child(label.to_string()),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.text_secondary)
                .child(val.to_string()),
        )
}
