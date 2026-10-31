use gpui_kit::*;
use crate::state::models::Album;
use crate::theme::AppTheme;

/// Renders a single album card in the album gallery grid.
pub fn render_album_card(
    album: &Album,
    theme: &AppTheme,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_edit_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let album_id = album.id;
    let album_name = album.name.clone();
    let media_count = album.media_count;
    let has_preview = album.preview_item.is_some();

    div()
        .id(SharedString::from(format!("album-card-{}", album_id)))
        .flex()
        .flex_col()
        .rounded(px(5.0))
        .border_1()
        .border_color(theme.border_card)
        .bg(theme.bg_card)
        .overflow_hidden()
        .cursor_pointer()
        .hover(move |s| s.border_color(theme.border_active).bg(theme.bg_card_hover))
        .on_click(on_click)
        // Top: Cover Preview Area with folder visual cues
        .child(
            div()
                .relative()
                .w_full()
                .h(px(112.0))
                .bg(theme.bg_sidebar)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_color(theme.accent)
                                .child(if has_preview { "🖼️" } else { "📁" }),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_muted)
                                .child(if has_preview { "Album" } else { "Empty Album" }),
                        ),
                )
                // Top-Right: Edit / Context action button
                .child(
                    div()
                        .id(SharedString::from(format!("album-edit-btn-{}", album_id)))
                        .absolute()
                        .top(px(6.0))
                        .right(px(6.0))
                        .px(px(6.0))
                        .py(px(2.0))
                        .rounded(px(3.0))
                        .bg(rgba(0x00000088))
                        .border_1()
                        .border_color(rgba(0xffffff20))
                        .cursor_pointer()
                        .hover(move |s| s.bg(theme.bg_pill_active).border_color(theme.accent))
                        .on_click(on_edit_click)
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_secondary)
                                .child("✎"),
                        ),
                )
                // Bottom-Right: Count badge overlay
                .child(
                    div()
                        .absolute()
                        .bottom(px(6.0))
                        .right(px(6.0))
                        .px(px(6.0))
                        .py(px(1.5))
                        .rounded(px(3.0))
                        .bg(rgba(0x000000bb))
                        .border_1()
                        .border_color(rgba(0xffffff15))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(0xf4f4f5))
                                .child(format!("{} items", media_count)),
                        ),
                ),
        )
        // Bottom: Album Name & Metadata
        .child(
            div()
                .p(px(8.0))
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_primary)
                        .overflow_hidden()
                        .child(album_name),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .overflow_hidden()
                        .child(format!("albums/{}", album.relative_path)),
                ),
        )
}
