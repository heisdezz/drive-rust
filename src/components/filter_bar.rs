use gpui_kit::*;
use std::sync::Arc;
use crate::state::models::{MediaFilterType, MediaSortBy, SortOrder};

pub type OnFilterCallback = Arc<dyn Fn(MediaFilterType, &mut App) + 'static>;
pub type OnSortCallback = Arc<dyn Fn(MediaSortBy, &mut App) + 'static>;

/// Renders the compact toolbar for filtering, sorting, and searching media catalog.
pub fn render_filter_bar(
    filter: MediaFilterType,
    sort_by: MediaSortBy,
    sort_order: SortOrder,
    total_count: i64,
    on_filter: OnFilterCallback,
    on_sort: OnSortCallback,
    on_toggle_order: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let filter_options = [
        (MediaFilterType::All, "All"),
        (MediaFilterType::Images, "Images"),
        (MediaFilterType::Videos, "Videos"),
    ];

    let sort_options = [
        (MediaSortBy::Date, "Date"),
        (MediaSortBy::Name, "Name"),
        (MediaSortBy::Size, "Size"),
    ];

    div()
        .w_full()
        .px(px(10.0))
        .py(px(6.0))
        .bg(rgb(0x131519))
        .border_1()
        .border_color(rgb(0x1e2025))
        .rounded(px(5.0))
        .flex()
        .items_center()
        .justify_between()
        .gap(px(10.0))
        // Left: Segmented Filter Buttons (All / Images / Videos)
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(2.0))
                .p(px(2.0))
                .rounded(px(4.0))
                .bg(rgb(0x181a20))
                .border_1()
                .border_color(rgb(0x23262d))
                .children(filter_options.into_iter().map(move |(f, label)| {
                    let is_active = filter == f;
                    let (bg, text, border) = if is_active {
                        (rgb(0x272c36), rgb(0xffffff), rgb(0x384152))
                    } else {
                        (rgba(0x00000000), rgb(0x8e8e93), rgba(0x00000000))
                    };

                    let cb = on_filter.clone();

                    div()
                        .id(SharedString::from(format!("filter-btn-{}", label)))
                        .cursor_pointer()
                        .px(px(8.0))
                        .py(px(3.0))
                        .rounded(px(3.0))
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .hover(|s| if !is_active { s.bg(rgb(0x1e2128)).text_color(rgb(0xd4d4d8)) } else { s })
                        .on_click(move |_, _window, cx| (cb)(f, cx))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                                .text_color(text)
                                .child(label),
                        )
                })),
        )
        // Center: Item Count Badge
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(4.0))
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x71717a))
                        .child(format!("{} items", total_count)),
                ),
        )
        // Right: Sort by & Order Controls
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                // Sort by pill group
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(2.0))
                        .p(px(2.0))
                        .rounded(px(4.0))
                        .bg(rgb(0x181a20))
                        .border_1()
                        .border_color(rgb(0x23262d))
                        .children(sort_options.into_iter().map(move |(s, label)| {
                            let is_active = sort_by == s;
                            let (bg, text, border) = if is_active {
                                (rgb(0x272c36), rgb(0xffffff), rgb(0x384152))
                            } else {
                                (rgba(0x00000000), rgb(0x8e8e93), rgba(0x00000000))
                            };

                            let cb = on_sort.clone();

                            div()
                                .id(SharedString::from(format!("sort-btn-{}", label)))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(3.0))
                                .rounded(px(3.0))
                                .bg(bg)
                                .border_1()
                                .border_color(border)
                                .hover(|st| if !is_active { st.bg(rgb(0x1e2128)).text_color(rgb(0xd4d4d8)) } else { st })
                                .on_click(move |_, _window, cx| (cb)(s, cx))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                                        .text_color(text)
                                        .child(label),
                                )
                        })),
                )
                // Direction Toggle button (Asc / Desc)
                .child(
                    div()
                        .id(SharedString::from("sort-order-toggle"))
                        .cursor_pointer()
                        .px(px(7.0))
                        .py(px(3.0))
                        .rounded(px(3.0))
                        .bg(rgb(0x181a20))
                        .border_1()
                        .border_color(rgb(0x23262d))
                        .hover(|s| s.bg(rgb(0x272c36)).text_color(rgb(0xffffff)))
                        .on_click(on_toggle_order)
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(0xa1a1aa))
                                .child(match sort_order {
                                    SortOrder::Asc => "▲ Asc",
                                    SortOrder::Desc => "▼ Desc",
                                }),
                        ),
                ),
        )
}
