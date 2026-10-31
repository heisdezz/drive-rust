use std::sync::Arc;
use gpui_kit::*;
use crate::components::filter_bar::render_filter_bar;
use crate::components::media_card::render_media_card;
use crate::state::app_state::AppState;
use crate::state::models::{MediaFilterType, MediaSortBy};

/// Renders the complete media catalog grid view, including filter toolbar and pagination.
pub fn render_media_grid(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let filter = state.media_filter;
    let sort_by = state.media_sort_by;
    let sort_order = state.media_sort_order;
    let page = state.media_page;
    let page_size = state.media_page_size;
    let selected_ids = state.selected_media_ids.clone();

    let db_opt = state.active_db.clone();

    // Query media items from SQLite
    let (items, total_count) = if let Some(ref db) = db_opt {
        let offset = ((page.saturating_sub(1)) * page_size) as i64;
        db.get_media_items(None, page_size as i64, offset, filter, None, sort_by, sort_order)
            .unwrap_or_default()
    } else {
        (Vec::new(), 0)
    };

    let total_pages = if total_count == 0 {
        1
    } else {
        ((total_count as usize + page_size - 1) / page_size).max(1)
    };

    let has_prev = page > 1;
    let has_next = page < total_pages;

    let state_filter = app_state.clone();
    let state_sort = app_state.clone();
    let state_order = app_state.clone();
    let state_prev = app_state.clone();
    let state_next = app_state.clone();

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

    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .w_full()
        .h_full()
        // Top: Filter & Sort Bar
        .child(render_filter_bar(
            filter,
            sort_by,
            sort_order,
            total_count,
            on_filter,
            on_sort,
            move |_, _, cx| {
                state_order.update(cx, |s, cx| {
                    s.toggle_media_sort_order();
                    cx.notify();
                });
            },
        ))
        // Center: Grid of cards or Empty State
        .child(
            div()
                .id(SharedString::from("media-grid-scroll"))
                .flex_grow(1.0)
                .overflow_y_scroll()
                .child(
                    if db_opt.is_none() {
                        div()
                            .p(px(24.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x181a20))
                            .border_1()
                            .border_color(rgb(0x23262d))
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xfbbf24))
                                    .child("No Storage Drive Selected"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x71717a))
                                    .child("Select a mounted drive from the sidebar to inspect its media library."),
                            )
                    } else if items.is_empty() {
                        div()
                            .p(px(32.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x181a20))
                            .border_1()
                            .border_color(rgb(0x23262d))
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xd4d4d8))
                                    .child("No Media Items Found"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x71717a))
                                    .child("No files match your current filters, or the drive hasn't been scanned yet. Go to Discover to start indexing."),
                            )
                    } else {
                        // Responsive card flow grid
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(8.0))
                            .children(items.into_iter().map(|item| {
                                let id = item.id;
                                let is_selected = selected_ids.contains(&id);
                                let state_click = app_state.clone();
                                let state_toggle = app_state.clone();

                                div()
                                    .w(px(146.0))
                                    .child(render_media_card(
                                        &item,
                                        is_selected,
                                        move |_, _, cx| {
                                            state_click.update(cx, |s, cx| {
                                                s.toggle_media_selection(id);
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
        )
        // Bottom: Compact Zed Pagination Bar
        .child(
            div()
                .h(px(36.0))
                .px(px(10.0))
                .bg(rgb(0x131519))
                .border_1()
                .border_color(rgb(0x1e2025))
                .rounded(px(5.0))
                .flex()
                .items_center()
                .justify_between()
                // Left: Page status & selection info
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x71717a))
                                .child(format!("Page {} of {}", page, total_pages)),
                        )
                        .child(
                            if !selected_ids.is_empty() {
                                let state_clear = app_state.clone();
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .px(px(6.0))
                                            .py(px(1.5))
                                            .rounded(px(3.0))
                                            .bg(rgb(0x1e2433))
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x60a5fa))
                                            .child(format!("{} selected", selected_ids.len())),
                                    )
                                    .child(
                                        div()
                                            .id(SharedString::from("deselect-all-btn"))
                                            .cursor_pointer()
                                            .text_xs()
                                            .text_color(rgb(0xa1a1aa))
                                            .hover(|s| s.text_color(rgb(0xf87171)))
                                            .on_click(move |_, _, cx| {
                                                state_clear.update(cx, |s, cx| {
                                                    s.clear_media_selection();
                                                    cx.notify();
                                                });
                                            })
                                            .child("Deselect all"),
                                    )
                            } else {
                                div()
                            },
                        ),
                )
                // Right: Prev / Next Buttons
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        // Prev Button
                        .child(
                            div()
                                .id(SharedString::from("pagination-prev-btn"))
                                .px(px(8.0))
                                .py(px(3.0))
                                .rounded(px(3.0))
                                .border_1()
                                .border_color(if has_prev { rgb(0x23262d) } else { rgba(0x00000000) })
                                .bg(if has_prev { rgb(0x181a20) } else { rgba(0x00000000) })
                                .text_color(if has_prev { rgb(0xd4d4d8) } else { rgb(0x3f3f46) })
                                .cursor_pointer()
                                .hover(|s| if has_prev { s.bg(rgb(0x272c36)).text_color(rgb(0xffffff)) } else { s })
                                .on_click(move |_, _, cx| {
                                    if has_prev {
                                        state_prev.update(cx, |s, cx| {
                                            s.set_media_page(page - 1);
                                            cx.notify();
                                        });
                                    }
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .child("← Prev"),
                                ),
                        )
                        // Next Button
                        .child(
                            div()
                                .id(SharedString::from("pagination-next-btn"))
                                .px(px(8.0))
                                .py(px(3.0))
                                .rounded(px(3.0))
                                .border_1()
                                .border_color(if has_next { rgb(0x23262d) } else { rgba(0x00000000) })
                                .bg(if has_next { rgb(0x181a20) } else { rgba(0x00000000) })
                                .text_color(if has_next { rgb(0xd4d4d8) } else { rgb(0x3f3f46) })
                                .cursor_pointer()
                                .hover(|s| if has_next { s.bg(rgb(0x272c36)).text_color(rgb(0xffffff)) } else { s })
                                .on_click(move |_, _, cx| {
                                    if has_next {
                                        state_next.update(cx, |s, cx| {
                                            s.set_media_page(page + 1);
                                            cx.notify();
                                        });
                                    }
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .child("Next →"),
                                ),
                        ),
                ),
        )
}
