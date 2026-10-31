use gpui_kit::*;
use std::path::Path;
use crate::drives::detector::format_bytes;
use crate::state::models::MediaItem;
use crate::theme::AppTheme;

/// Formats media duration in seconds into `MM:SS` or `HH:MM:SS`.
pub fn format_duration(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

/// Renders an individual media card in the media catalog grid with dynamic theme support.
pub fn render_media_card(
    item: &MediaItem,
    is_selected: bool,
    theme: &AppTheme,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_toggle_select: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let filename = Path::new(&item.current_relative_path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| item.current_relative_path.clone());

    let is_video = item.mime_type.starts_with("video/");
    let size_str = format_bytes(item.file_size as u64);
    let date_str = item
        .created_at
        .split(' ')
        .next()
        .unwrap_or("")
        .to_string();

    let (border_color, bg_color) = if is_selected {
        (theme.accent, theme.bg_pill_active)
    } else {
        (theme.border_card, theme.bg_card)
    };

    let item_id = item.id;
    let hover_border = if is_selected { theme.accent_hover } else { theme.border_active };
    let hover_bg = if is_selected { theme.bg_pill_active } else { theme.bg_card_hover };

    div()
        .id(SharedString::from(format!("media-card-{}", item_id)))
        .flex()
        .flex_col()
        .rounded(px(5.0))
        .border_1()
        .border_color(border_color)
        .bg(bg_color)
        .overflow_hidden()
        .cursor_pointer()
        .hover(move |s| s.border_color(hover_border).bg(hover_bg))
        .on_click(on_click)
        // Thumbnail & Overlay Area
        .child(
            div()
                .relative()
                .w_full()
                .h(px(108.0))
                .bg(theme.bg_sidebar)
                .flex()
                .items_center()
                .justify_center()
                // Placeholder icon or preview graphic
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(3.0))
                        .child(
                            div()
                                .text_color(if is_video { theme.accent } else { theme.text_secondary })
                                .child(if is_video { "▷" } else { "▣" }),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_muted)
                                .child(if is_video { "VIDEO" } else { "IMAGE" }),
                        ),
                )
                // Top-Left: Multi-select Checkbox Badge
                .child(
                    div()
                        .id(SharedString::from(format!("check-{}", item_id)))
                        .absolute()
                        .top(px(6.0))
                        .left(px(6.0))
                        .w(px(16.0))
                        .h(px(16.0))
                        .rounded(px(3.0))
                        .border_1()
                        .border_color(if is_selected { theme.accent } else { theme.border_subtle })
                        .bg(if is_selected { theme.accent } else { rgba(0x00000099) })
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .on_click(on_toggle_select)
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .child(if is_selected { "✓" } else { "" }),
                        ),
                )
                // Bottom-Right: Video duration badge if video
                .child(
                    if is_video {
                        let dur_text = item
                            .duration_seconds
                            .map(format_duration)
                            .unwrap_or_else(|| "Video".to_string());

                        div()
                            .absolute()
                            .bottom(px(6.0))
                            .right(px(6.0))
                            .px(px(4.0))
                            .py(px(1.0))
                            .rounded(px(3.0))
                            .bg(rgba(0x000000cc))
                            .border_1()
                            .border_color(rgba(0xffffff15))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::NORMAL)
                                    .text_color(rgb(0xffffff))
                                    .child(dur_text),
                            )
                    } else {
                        div()
                    },
                ),
        )
        // Metadata Footer
        .child(
            div()
                .p(px(6.0))
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.text_primary)
                        .child(filename),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(size_str),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(date_str),
                        ),
                ),
        )
}

#[cfg(test)]
mod tests {
    use super::format_duration;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(45), "00:45");
        assert_eq!(format_duration(125), "02:05");
        assert_eq!(format_duration(3665), "01:01:05");
    }
}
