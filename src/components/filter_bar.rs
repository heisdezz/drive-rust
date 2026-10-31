use gpui_kit::*;
use std::sync::Arc;
use crate::state::models::{MediaFilterType, MediaSortBy, SortOrder};
use crate::theme::AppTheme;

pub type OnFilterCallback = Arc<dyn Fn(MediaFilterType, &mut App) + 'static>;
pub type OnSortCallback = Arc<dyn Fn(MediaSortBy, &mut App) + 'static>;

/// Renders the compact toolbar for filtering, sorting, and searching media catalog with dynamic theme support.
pub fn render_filter_bar(
    filter: MediaFilterType,
    sort_by: MediaSortBy,
    sort_order: SortOrder,
    total_count: i64,
    theme: &AppTheme,
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

    let t = *theme;

    div()
        .w_full()
        .px(px(10.0))
        .py(px(6.0))
        .bg(t.bg_header)
        .border_1()
        .border_color(t.border_subtle)
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
                .bg(t.bg_card)
                .border_1()
                .border_color(t.border_card)
                .children(filter_options.into_iter().map(move |(f, label)| {
                    let is_active = filter == f;
                    let (bg, text, border) = if is_active {
                        (t.bg_pill_active, t.text_primary, t.border_active)
                    } else {
                        (rgba(0x00000000), t.text_muted, rgba(0x00000000))
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
                        .hover(move |s| if !is_active { s.bg(t.bg_card_hover).text_color(t.text_secondary) } else { s })
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
                        .text_color(t.text_muted)
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
                        .bg(t.bg_card)
                        .border_1()
                        .border_color(t.border_card)
                        .children(sort_options.into_iter().map(move |(s, label)| {
                            let is_active = sort_by == s;
                            let (bg, text, border) = if is_active {
                                (t.bg_pill_active, t.text_primary, t.border_active)
                            } else {
                                (rgba(0x00000000), t.text_muted, rgba(0x00000000))
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
                                .hover(move |st| if !is_active { st.bg(t.bg_card_hover).text_color(t.text_secondary) } else { st })
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
                        .bg(t.bg_card)
                        .border_1()
                        .border_color(t.border_card)
                        .hover(move |s| s.bg(t.bg_pill_active).text_color(t.text_primary))
                        .on_click(on_toggle_order)
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(t.text_secondary)
                                .child(match sort_order {
                                    SortOrder::Asc => "▲ Asc",
                                    SortOrder::Desc => "▼ Desc",
                                }),
                        ),
                ),
        )
}
