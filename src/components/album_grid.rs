use std::sync::Arc;
use gpui_kit::*;
use crate::components::album_card::render_album_card;
use crate::components::filter_bar::render_filter_bar;
use crate::components::media_card::render_media_card;
use crate::state::app_state::AppState;
use crate::state::models::{MediaFilterType, MediaSortBy};

/// Renders either the gallery of all albums, or the detail view of an individual album.
pub fn render_album_grid(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let db_opt = state.active_db.clone();
    let selected_album_id = state.selected_album_id;

    if db_opt.is_none() {
        return div()
            .p(px(24.0))
            .rounded(px(6.0))
            .bg(theme.bg_card)
            .border_1()
            .border_color(theme.border_card)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.accent_warning)
                    .child("No Storage Drive Selected"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child("Select a mounted drive from the sidebar to inspect or manage its albums."),
            );
    }

    let db = db_opt.as_ref().unwrap();

    // If an album is selected, render Album Detail View
    if let Some(album_id) = selected_album_id {
        let album = db.get_album_by_id(album_id).unwrap_or(None);
        let album_title = album.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown Album".to_string());

        let filter = state.media_filter;
        let sort_by = state.media_sort_by;
        let sort_order = state.media_sort_order;
        let page = state.media_page;
        let page_size = state.media_page_size;
        let selected_ids = state.selected_media_ids.clone();

        let (items, total_count) = {
            let offset = ((page.saturating_sub(1)) * page_size) as i64;
            db.get_media_items(Some(album_id), page_size as i64, offset, filter, None, sort_by, sort_order)
                .unwrap_or_default()
        };

        let state_back = app_state.clone();
        let state_edit = app_state.clone();
        let album_for_edit = album.clone();

        let state_filter = app_state.clone();
        let state_sort = app_state.clone();
        let state_order = app_state.clone();

        let on_filter = Arc::new(move |new_filter: MediaFilterType, cx: &mut App| {
            state_filter.update(cx, |s, cx| {
                s.set_media_filter(new_filter);
                cx.notify();
            });
        });

        let on_sort = Arc::new(move |new_sort: MediaSortBy, cx: &mut App| {
            state_sort.update(cx, |s, cx| {
                s.set_media_sort_by(new_sort);
                cx.notify();
            });
        });

        return div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .w_full()
            .h_full()
            // Top: Back bar and Album Metadata
            .child(
                div()
                    .w_full()
                    .px(px(10.0))
                    .py(px(6.0))
                    .bg(theme.bg_header)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .rounded(px(5.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id(SharedString::from("back-to-albums-btn"))
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(3.0))
                                    .rounded(px(3.0))
                                    .bg(theme.bg_pill)
                                    .border_1()
                                    .border_color(theme.border_card)
                                    .hover(move |s| s.bg(theme.bg_pill_active).text_color(theme.text_primary))
                                    .on_click(move |_, _, cx| {
                                        state_back.update(cx, |s, cx| {
                                            s.select_album(None);
                                            cx.notify();
                                        });
                                    })
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.text_secondary)
                                            .child("← Albums"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(album_title),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(format!("({} items)", total_count)),
                            ),
                    )
                    .child(
                        if let Some(a) = album_for_edit {
                            div()
                                .id(SharedString::from("detail-edit-album-btn"))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(3.0))
                                .rounded(px(3.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .hover(move |s| s.bg(theme.bg_pill_active).text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    let alb = a.clone();
                                    state_edit.update(cx, |s, cx| {
                                        s.open_edit_album_modal(alb);
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_secondary)
                                        .child("✎ Manage"),
                                )
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        },
                    ),
            )
            // Filter Bar
            .child(render_filter_bar(
                filter,
                sort_by,
                sort_order,
                total_count,
                &theme,
                on_filter,
                on_sort,
                move |_, _, cx| {
                    state_order.update(cx, |s, cx| {
                        s.toggle_media_sort_order();
                        cx.notify();
                    });
                },
            ))
            // Media Grid within Album
            .child(
                div()
                    .id(SharedString::from("album-detail-scroll"))
                    .flex_grow(1.0)
                    .overflow_y_scroll()
                    .child(
                        if items.is_empty() {
                            div()
                                .p(px(32.0))
                                .rounded(px(6.0))
                                .bg(theme.bg_card)
                                .border_1()
                                .border_color(theme.border_card)
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_primary)
                                        .child("Album is Empty"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("No media cataloged in this album folder yet. Use Discover to scan and sort files."),
                                )
                        } else {
                            div()
                                .flex()
                                .flex_wrap()
                                .gap(px(8.0))
                                .children(items.into_iter().map(|item| {
                                    let id = item.id;
                                    let is_selected = selected_ids.contains(&id);
                                    let state_click = app_state.clone();
                                    let state_toggle = app_state.clone();
                                    let item_clone = item.clone();
                                    let db_card = db_opt.clone();

                                    div()
                                        .w(px(146.0))
                                        .child(render_media_card(
                                            &item,
                                            is_selected,
                                            &theme,
                                            move |_, _, cx| {
                                                let (p_prev, p_next) = if let Some(ref db) = db_card {
                                                    db.get_media_item_with_neighbors(id, Some(album_id))
                                                        .ok()
                                                        .flatten()
                                                        .map(|(_, p, n)| (p, n))
                                                        .unwrap_or((None, None))
                                                } else {
                                                    (None, None)
                                                };
                                                let it = item_clone.clone();
                                                state_click.update(cx, |s, cx| {
                                                    s.open_player(it, p_prev, p_next);
                                                    cx.notify();
                                                });
                                            },
                                            move |_, _, cx| {
                                                state_toggle.update(cx, |s, cx| {
                                                    s.toggle_media_selection(id);
                                                    cx.notify();
                                                });
                                            },
                                        ))
                                }))
                        },
                    ),
            );
    }

    // Default: Gallery View (all albums)
    let albums = db.get_albums().unwrap_or_default();
    let state_create = app_state.clone();

    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .w_full()
        .h_full()
        // Top: Gallery Toolbar (+ New Album button + album count)
        .child(
            div()
                .h(px(38.0))
                .px(px(10.0))
                .bg(theme.bg_header)
                .border_1()
                .border_color(theme.border_subtle)
                .rounded(px(5.0))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("ALL ALBUMS"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(format!("({} total)", albums.len())),
                        ),
                )
                .child(
                    div()
                        .id(SharedString::from("create-album-btn"))
                        .cursor_pointer()
                        .px(px(8.0))
                        .py(px(3.5))
                        .rounded(px(4.0))
                        .bg(theme.accent)
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .hover(move |s| s.bg(theme.accent_hover))
                        .on_click(move |_, _, cx| {
                            state_create.update(cx, |s, cx| {
                                s.open_create_album_modal();
                                cx.notify();
                            });
                        })
                        .child("+ New Album"),
                ),
        )
        // Center: Grid of Albums
        .child(
            div()
                .id(SharedString::from("albums-gallery-scroll"))
                .flex_grow(1.0)
                .overflow_y_scroll()
                .child(
                    if albums.is_empty() {
                        div()
                            .p(px(32.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_card)
                            .border_1()
                            .border_color(theme.border_card)
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_primary)
                                    .child("No Albums Found"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("Create an album or scan folders from Discover to organize your media."),
                            )
                    } else {
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(10.0))
                            .children(albums.into_iter().map(|album| {
                                let album_id = album.id;
                                let album_clone = album.clone();
                                let state_card_click = app_state.clone();
                                let state_edit_click = app_state.clone();

                                div()
                                    .w(px(160.0))
                                    .child(render_album_card(
                                        &album,
                                        &theme,
                                        move |_, _, cx| {
                                            state_card_click.update(cx, |s, cx| {
                                                s.select_album(Some(album_id));
                                                cx.notify();
                                            });
                                        },
                                        move |_, _, cx| {
                                            let a = album_clone.clone();
                                            state_edit_click.update(cx, |s, cx| {
                                                s.open_edit_album_modal(a);
                                                cx.notify();
                                            });
                                        },
                                    ))
                            }))
                    },
                ),
        )
}
