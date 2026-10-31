use gpui_kit::*;
use std::path::Path;
use crate::drives::detector::format_bytes;
use crate::state::models::MediaItem;

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

/// Renders an individual media card in the media catalog grid.
pub fn render_media_card(
    item: &MediaItem,
    is_selected: bool,
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
        (rgb(0x3b82f6), rgb(0x1d2433)) // Blue border and subtle tint when selected
    } else {
        (rgb(0x23262d), rgb(0x181a20)) // Dark charcoal card
    };

    let item_id = item.id;

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
        .hover(|s| {
            if is_selected {
                s.border_color(rgb(0x60a5fa))
            } else {
                s.border_color(rgb(0x384152)).bg(rgb(0x1f232b))
            }
        })
        .on_click(on_click)
        // Thumbnail & Overlay Area
        .child(
            div()
                .relative()
                .w_full()
                .h(px(108.0))
                .bg(rgb(0x111317))
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
                                .text_color(if is_video { rgb(0xa855f7) } else { rgb(0x38bdf8) })
                                .child(if is_video { "🎬" } else { "🖼" }),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(0x52525b))
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
                        .border_color(if is_selected { rgb(0x3b82f6) } else { rgb(0x3f3f46) })
                        .bg(if is_selected { rgb(0x2563eb) } else { rgba(0x00000099) })
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
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0xf4f4f5))
                                    .child(format!("▶ {}", dur_text)),
                            )
                    } else {
                        div()
                    },
                ),
        )
        // Bottom Meta Bar (Name + size)
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
                        .text_color(if is_selected { rgb(0xffffff) } else { rgb(0xe4e4e7) })
                        .overflow_hidden()
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
                                .text_color(rgb(0x71717a))
                                .child(size_str),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x52525b))
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
        assert_eq!(format_duration(0), "00:00");
        assert_eq!(format_duration(45), "00:45");
        assert_eq!(format_duration(75), "01:15");
        assert_eq!(format_duration(3600), "01:00:00");
        assert_eq!(format_duration(3665), "01:01:05");
    }
}
