use gpui_kit::*;
use crate::state::models::{DriveInfo, DriveType};
use crate::theme::AppTheme;
use crate::theme::icons::Icon;

/// Renders a single drive card in the sidebar based on the reference design.
/// Internal system drives are protected and disabled from selection by default.
pub fn render_drive_card(
    drive: &DriveInfo,
    is_selected: bool,
    theme: &AppTheme,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let drive_name = drive.name.clone();
    let size_label = drive.size_formatted.clone();
    let used_pct = drive.used_percentage;
    let is_system = drive.drive_type == DriveType::Internal || drive.name == "System Root";

    let border_color = if is_selected {
        theme.accent
    } else if is_system {
        theme.border_subtle
    } else {
        theme.border_card
    };

    let hover_bg = if is_selected {
        theme.bg_card
    } else if is_system {
        theme.bg_card
    } else {
        theme.bg_card_hover
    };

    let mut card = div()
        .id(SharedString::from(format!("drive-card-{}", drive.id)))
        .p(px(8.0))
        .rounded(px(6.0))
        .border_1()
        .border_color(border_color)
        .bg(theme.bg_card)
        .flex()
        .flex_col()
        .gap(px(6.0));

    if is_system {
        card = card.cursor_not_allowed();
    } else {
        card = card.cursor_pointer().hover(move |s| s.bg(hover_bg)).on_click(on_click);
    }

    card
        // Top row: disk icon + volume name & size + type badge
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
                        .gap(px(8.0))
                        .overflow_hidden()
                        .child(
                            div()
                                .w(px(26.0))
                                .h(px(26.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_xs()
                                .text_color(if is_selected {
                                    theme.accent
                                } else if is_system {
                                    theme.text_muted
                                } else {
                                    theme.text_secondary
                                })
                                .child(Icon::DISK),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(1.0))
                                .overflow_hidden()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if is_system { theme.text_secondary } else { theme.text_primary })
                                        .overflow_hidden()
                                        .child(drive_name),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(size_label.clone()),
                                ),
                        ),
                )
                .child(
                    if is_system {
                        div()
                            .px(px(5.0))
                            .py(px(1.5))
                            .rounded(px(3.0))
                            .bg(theme.bg_pill)
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.text_muted)
                            .child("SYSTEM")
                    } else if drive.is_mounted {
                        div()
                            .px(px(6.0))
                            .py(px(1.5))
                            .rounded(px(3.0))
                            .bg(theme.bg_pill_active)
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.accent)
                            .child(format!("{} Mounted", Icon::DOT))
                    } else {
                        div()
                            .px(px(5.0))
                            .py(px(1.5))
                            .rounded(px(3.0))
                            .bg(theme.bg_pill)
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.text_muted)
                            .child("Unmounted")
                    },
                ),
        )
        // Middle row: slim progress bar
        .child(
            div()
                .w_full()
                .h(px(2.5))
                .rounded(px(1.25))
                .bg(theme.bg_pill)
                .overflow_hidden()
                .child(
                    div()
                        .h_full()
                        .w(relative((used_pct as f32).clamp(0.0, 100.0) / 100.0))
                        .bg(if is_system { theme.text_muted } else { theme.accent })
                        .rounded(px(1.25)),
                ),
        )
        // Bottom row: usage percent on left, total size on right
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(format!("{}% used", used_pct)),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(size_label),
                ),
        )
}
