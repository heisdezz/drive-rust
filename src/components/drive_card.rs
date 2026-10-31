use gpui_kit::*;
use crate::state::models::{DriveInfo, DriveType};
use crate::theme::AppTheme;

/// Renders a single drive card in the sidebar with dynamic theme support.
pub fn render_drive_card(
    drive: &DriveInfo,
    is_selected: bool,
    theme: &AppTheme,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let drive_name = drive.name.clone();
    let size_label = drive.size_formatted.clone();
    let used_pct = drive.used_percentage;
    let is_external = drive.drive_type == DriveType::External;

    let type_label = match drive.drive_type {
        DriveType::External => "USB",
        DriveType::Internal => "SYS",
        DriveType::Network => "NET",
    };

    let (bg_color, border_color) = if is_selected {
        (theme.bg_pill_active, theme.border_active)
    } else {
        (rgba(0x00000000), rgba(0x00000000))
    };

    let icon_color = if is_external {
        theme.accent_success
    } else {
        theme.accent
    };

    let bar_fill_color = if used_pct > 90 {
        theme.accent_danger
    } else if is_external {
        theme.accent_success
    } else {
        theme.accent
    };

    let hover_bg = if is_selected {
        theme.bg_pill_active
    } else {
        theme.bg_card_hover
    };

    div()
        .id(SharedString::from(format!("drive-card-{}", drive.id)))
        .cursor_pointer()
        .px(px(7.0))
        .py(px(5.0))
        .rounded(px(4.0))
        .border_1()
        .border_color(border_color)
        .bg(bg_color)
        .hover(move |s| s.bg(hover_bg))
        .on_click(on_click)
        // Top row: status dot + volume name + type pill
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap(px(6.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .overflow_hidden()
                        .child(
                            div()
                                .w(px(6.0))
                                .h(px(6.0))
                                .rounded(px(3.0))
                                .bg(icon_color)
                                .flex_shrink_0(),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(if is_selected { FontWeight::MEDIUM } else { FontWeight::NORMAL })
                                .text_color(if is_selected { theme.text_primary } else { theme.text_secondary })
                                .overflow_hidden()
                                .child(drive_name),
                        ),
                )
                .child(
                    div()
                        .px(px(4.0))
                        .py(px(0.5))
                        .rounded(px(3.0))
                        .bg(theme.bg_pill)
                        .text_color(theme.text_muted)
                        .font_weight(FontWeight::MEDIUM)
                        .child(type_label),
                ),
        )
        // Bottom row: slim 2px progress bar + usage metrics
        .child(
            div()
                .mt(px(4.0))
                .child(
                    div()
                        .w_full()
                        .h(px(2.0))
                        .rounded(px(1.0))
                        .bg(theme.border_subtle)
                        .overflow_hidden()
                        .child(
                            div()
                                .h_full()
                                .w(relative(used_pct as f32 / 100.0))
                                .bg(bar_fill_color)
                                .rounded(px(1.0)),
                        ),
                )
                .child(
                    div()
                        .mt(px(2.5))
                        .flex()
                        .items_center()
                        .justify_between()
                        .text_color(theme.text_muted)
                        .child(div().child(size_label))
                        .child(div().child(format!("{}%", used_pct))),
                ),
        )
}
