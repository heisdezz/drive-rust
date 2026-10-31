use gpui_kit::*;
use std::sync::Arc;
use crate::state::models::{MediaFilterType, MediaSortBy, SortOrder};
use crate::theme::AppTheme;
use crate::theme::icons::Icon;

pub type OnFilterCallback = Arc<dyn Fn(MediaFilterType, &mut App) + 'static>;
pub type OnSortCallback = Arc<dyn Fn(MediaSortBy, &mut App) + 'static>;

/// Renders the compact toolbar for filtering, sorting, and searching media catalog with dynamic theme support.
pub fn render_filter_bar(
    filter: MediaFilterType,
    sort_by: MediaSortBy,
    sort_order: SortOrder,
    total_count: i64,
    search_query: Option<&str>,
    theme: &AppTheme,
    on_filter: OnFilterCallback,
    on_sort: OnSortCallback,
    on_toggle_order: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_search: Option<Arc<dyn Fn(String, &mut App) + 'static>>,
    on_clear_search: Option<Arc<dyn Fn(&mut App) + 'static>>,
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

    let search_presets = [".jpg", ".png", ".mp4", "dcim"];
    let t = *theme;
    let current_query = search_query.unwrap_or("").trim();

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
        // Middle: Search Preset Pills & Active Query Tag & Item Count
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(
                    if !current_query.is_empty() {
                        let on_clear = on_clear_search.clone();
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .px(px(8.0))
                            .py(px(2.0))
                            .rounded(px(3.0))
                            .bg(t.bg_pill_active)
                            .border_1()
                            .border_color(t.accent)
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(t.accent)
                                    .child(format!("{} \"{}\"", Icon::SEARCH, current_query)),
                            )
                            .child(
                                div()
                                    .id(SharedString::from("clear-filter-search-btn"))
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(t.accent)
                                    .hover(move |s| s.text_color(t.text_primary))
                                    .on_click(move |_, _, cx| {
                                        if let Some(ref clear_cb) = on_clear {
                                            (clear_cb)(cx);
                                        }
                                    })
                                    .child(Icon::CLOSE),
                            )
                            .into_any_element()
                    } else if let Some(ref search_cb) = on_search {
                        let search_cb = search_cb.clone();
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(t.text_muted)
                                    .child(Icon::SEARCH),
                            )
                            .children(search_presets.into_iter().map(move |preset| {
                                let cb = search_cb.clone();
                                let p = preset.to_string();
                                div()
                                    .id(SharedString::from(format!("search-preset-{}", preset)))
                                    .cursor_pointer()
                                    .px(px(6.0))
                                    .py(px(1.5))
                                    .rounded(px(3.0))
                                    .bg(t.bg_card)
                                    .border_1()
                                    .border_color(t.border_subtle)
                                    .hover(move |s| s.bg(t.bg_pill_active).border_color(t.accent))
                                    .on_click(move |_, _, cx| (cb)(p.clone(), cx))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(t.text_muted)
                                            .child(preset),
                                    )
                            }))
                            .into_any_element()
                    } else {
                        div().into_any_element()
                    }
                )
                .child(
                    div()
                        .px(px(6.0))
                        .py(px(1.5))
                        .rounded(px(3.0))
                        .bg(t.bg_pill)
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
                                    SortOrder::Asc => format!("{} Asc", Icon::ARROW_UP),
                                    SortOrder::Desc => format!("{} Desc", Icon::ARROW_DOWN),
                                }),
                        ),
                ),
        )
}
