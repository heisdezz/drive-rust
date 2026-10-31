use std::sync::Arc;
use gpui_kit::*;
use crate::components::album_card::render_album_card;
use crate::components::filter_bar::render_filter_bar;
use crate::components::media_card::render_media_card;
use crate::drives::detector::format_bytes;
use crate::state::app_state::AppState;
use crate::state::models::{MediaFilterType, MediaSortBy};
use crate::theme::icons::Icon;

const ALBUM_PRESET_QUICK_FILTERS: &[&str] = &[
    "Vacation", "Family", "Screenshots", "Wallpapers", "Camera Roll", "Favorites",
];

/// Renders either the gallery of all albums (Stage 3), or the detail inner view of an individual album (Stage 4).
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
                    .child(format!("{} No Storage Drive Selected", Icon::INFO)),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child("Select a mounted drive from the sidebar to inspect or manage its albums."),
            );
    }

    let db = db_opt.as_ref().unwrap();

    // =========================================================================
    // Stage 4: Album Inner / Detail View (`album-inner`)
    // =========================================================================
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

        // Calculate total storage size of loaded items
        let total_size_bytes: u64 = items.iter().map(|i| i.file_size as u64).sum();
        let total_size_str = format_bytes(total_size_bytes);

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
            // Step UI-4.1: Compact breadcrumb bar (`← Albums / <Album Name>`) with item count and storage size
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
                                            .child(format!("{} Albums", Icon::ARROW_LEFT)),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("/"),
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
                                    .px(px(6.0))
                                    .py(px(1.5))
                                    .rounded(px(3.0))
                                    .bg(theme.bg_pill)
                                    .border_1()
                                    .border_color(theme.border_card)
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.text_muted)
                                    .child(format!("{} items", total_count)),
                            )
                            .child(
                                div()
                                    .px(px(6.0))
                                    .py(px(1.5))
                                    .rounded(px(3.0))
                                    .bg(theme.bg_pill)
                                    .border_1()
                                    .border_color(theme.border_card)
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.text_muted)
                                    .child(total_size_str),
                            ),
                    )
                    // Step UI-4.2: Album management trigger (`✎ Manage`) opening edit/merge/delete modals
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
                                        .child(format!("{} Manage", Icon::EDIT)),
                                )
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        },
                    ),
            )
            // Step UI-4.3: FilterBar scoped to the album (`All`, `Images`, `Videos`, sort criteria, sort direction)
            .child(render_filter_bar(
                filter,
                sort_by,
                sort_order,
                total_count,
                None,
                &theme,
                on_filter,
                on_sort,
                move |_, _, cx| {
                    state_order.update(cx, |s, cx| {
                        s.toggle_media_sort_order();
                        cx.notify();
                    });
                },
                None,
                None,
            ))
            // Step UI-4.5: Floating bulk selection toolbar when items are selected
            .child(
                if !selected_ids.is_empty() {
                    let state_move = app_state.clone();
                    let state_del = app_state.clone();
                    let state_deselect = app_state.clone();
                    let db_bulk = db_opt.clone();
                    let ids_vec: Vec<i64> = selected_ids.iter().copied().collect();
                    let ids_for_move = ids_vec.clone();
                    let ids_for_del = ids_vec.clone();

                    div()
                        .px(px(10.0))
                        .py(px(6.0))
                        .rounded(px(4.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.accent)
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
                                        .px(px(6.0))
                                        .py(px(2.0))
                                        .rounded(px(3.0))
                                        .bg(theme.bg_pill_active)
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.accent)
                                        .child(format!("{} items selected", selected_ids.len())),
                                )
                                .child(
                                    div()
                                        .id(SharedString::from("album-bulk-move-btn"))
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .rounded(px(3.0))
                                        .bg(theme.bg_pill)
                                        .border_1()
                                        .border_color(theme.border_card)
                                        .hover(move |s| s.bg(theme.bg_card_hover).border_color(theme.accent))
                                        .on_click(move |_, _, cx| {
                                            let ids = ids_for_move.clone();
                                            state_move.update(cx, |s, cx| {
                                                s.open_move_modal(ids);
                                                cx.notify();
                                            });
                                        })
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(theme.text_primary)
                                                .child("Move to Album..."),
                                        ),
                                )
                                .child(
                                    div()
                                        .id(SharedString::from("album-bulk-delete-btn"))
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .rounded(px(3.0))
                                        .bg(theme.bg_pill)
                                        .border_1()
                                        .border_color(theme.border_card)
                                        .hover(move |s| s.bg(theme.accent_danger).text_color(rgb(0xffffff)))
                                        .on_click(move |_, _, cx| {
                                            if let Some(ref db) = db_bulk {
                                                let _ = db.delete_media_items(&ids_for_del);
                                            }
                                            state_del.update(cx, |s, cx| {
                                                s.clear_media_selection();
                                                cx.notify();
                                            });
                                        })
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(theme.accent_danger)
                                                .child("Delete Selected"),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .id(SharedString::from("album-bulk-deselect-btn"))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(3.0))
                                .rounded(px(3.0))
                                .bg(theme.bg_pill)
                                .text_xs()
                                .text_color(theme.text_muted)
                                .hover(move |s| s.text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_deselect.update(cx, |s, cx| {
                                        s.clear_media_selection();
                                        cx.notify();
                                    });
                                })
                                .child("Deselect all"),
                        )
                        .into_any_element()
                } else {
                    div().into_any_element()
                },
            )
            // Step UI-4.4: Album-scoped media grid with card click launching `MediaPlayerModal` with neighbor navigation
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
                                        .text_color(theme.text_muted)
                                        .child(Icon::IMAGE),
                                )
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

    // =========================================================================
    // Stage 3: Albums Gallery View (`album`)
    // =========================================================================
    let albums = db.get_albums().unwrap_or_default();
    let albums_total = albums.len();
    let search_query = state.album_search_query.trim().to_string();
    let query_lower = search_query.to_lowercase();

    let filtered_albums: Vec<_> = albums
        .into_iter()
        .filter(|a| {
            if query_lower.is_empty() {
                true
            } else {
                a.name.to_lowercase().contains(&query_lower) || a.relative_path.to_lowercase().contains(&query_lower)
            }
        })
        .collect();

    let state_create = app_state.clone();
    let state_clear_search = app_state.clone();

    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .w_full()
        .h_full()
        // Step UI-3.1: Gallery Toolbar with album search filter, total albums count, and + New Album button
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
                                .px(px(6.0))
                                .py(px(1.5))
                                .rounded(px(3.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(if query_lower.is_empty() {
                                    format!("{} total", albums_total)
                                } else {
                                    format!("{}/{} matched", filtered_albums.len(), albums_total)
                                }),
                        ),
                )
                // Center: Quick Preset Filters & Search Query Status
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(
                            if query_lower.is_empty() {
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(4.0))
                                    .children(ALBUM_PRESET_QUICK_FILTERS.iter().take(4).map(|&preset| {
                                        let state_preset = app_state.clone();
                                        div()
                                            .id(SharedString::from(format!("filter-chip-{}", preset)))
                                            .cursor_pointer()
                                            .px(px(6.0))
                                            .py(px(2.0))
                                            .rounded(px(3.0))
                                            .bg(theme.bg_pill)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .hover(move |s| s.bg(theme.bg_pill_active).border_color(theme.accent))
                                            .on_click(move |_, _, cx| {
                                                state_preset.update(cx, |s, cx| {
                                                    s.set_album_search_query(preset.to_string());
                                                    cx.notify();
                                                });
                                            })
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_muted)
                                                    .child(preset),
                                            )
                                    }))
                                    .into_any_element()
                            } else {
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .px(px(8.0))
                                    .py(px(2.0))
                                    .rounded(px(3.0))
                                    .bg(theme.bg_pill_active)
                                    .border_1()
                                    .border_color(theme.accent)
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.accent)
                                            .child(format!("{} \"{}\"", Icon::SEARCH, search_query)),
                                    )
                                    .child(
                                        div()
                                            .id(SharedString::from("clear-album-search-btn"))
                                            .cursor_pointer()
                                            .text_xs()
                                            .text_color(theme.accent)
                                            .hover(move |s| s.text_color(theme.text_primary))
                                            .on_click(move |_, _, cx| {
                                                state_clear_search.update(cx, |s, cx| {
                                                    s.clear_album_search_query();
                                                    cx.notify();
                                                });
                                            })
                                            .child(Icon::CLOSE),
                                    )
                                    .into_any_element()
                            },
                        ),
                )
                // Right: + New Album button
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
                        .child(format!("{} New Album", Icon::PLUS)),
                ),
        )
        // Step UI-3.4: Center Grid of Albums with Clean Empty States
        .child(
            div()
                .id(SharedString::from("albums-gallery-scroll"))
                .flex_grow(1.0)
                .overflow_y_scroll()
                .child(
                    if albums_total == 0 {
                        // Empty State: No albums on drive
                        let state_empty_create = app_state.clone();
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
                                    .text_color(theme.text_muted)
                                    .child(Icon::FOLDER),
                            )
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
                            .child(
                                div()
                                    .id(SharedString::from("empty-create-album-btn"))
                                    .cursor_pointer()
                                    .mt(px(6.0))
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(4.0))
                                    .bg(theme.accent)
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0xffffff))
                                    .hover(move |s| s.bg(theme.accent_hover))
                                    .on_click(move |_, _, cx| {
                                        state_empty_create.update(cx, |s, cx| {
                                            s.open_create_album_modal();
                                            cx.notify();
                                        });
                                    })
                                    .child(format!("{} Create First Album", Icon::PLUS)),
                            )
                    } else if filtered_albums.is_empty() {
                        // Empty State: Search filter yielded 0 results
                        let state_reset_filter = app_state.clone();
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
                                    .text_color(theme.accent_warning)
                                    .child(Icon::SEARCH),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_primary)
                                    .child(format!("No albums matching \"{}\"", search_query)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("Try a different search term or clear the filter."),
                            )
                            .child(
                                div()
                                    .id(SharedString::from("reset-album-search-btn"))
                                    .cursor_pointer()
                                    .mt(px(6.0))
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(4.0))
                                    .bg(theme.bg_pill)
                                    .border_1()
                                    .border_color(theme.border_card)
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.text_primary)
                                    .hover(move |s| s.bg(theme.bg_pill_active).border_color(theme.accent))
                                    .on_click(move |_, _, cx| {
                                        state_reset_filter.update(cx, |s, cx| {
                                            s.clear_album_search_query();
                                            cx.notify();
                                        });
                                    })
                                    .child(format!("{} Clear Search Filter", Icon::CLOSE)),
                            )
                    } else {
                        // Grid of Album Cards
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(10.0))
                            .children(filtered_albums.into_iter().map(|album| {
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
