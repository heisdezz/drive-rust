use gpui_kit::*;
use crate::state::models::{DriveInfo, DriveType};

/// Renders a single drive card in the sidebar with a compact, refined Zed-style design.
pub fn render_drive_card(
    drive: &DriveInfo,
    is_selected: bool,
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

    // Zed-style subtle active styling
    let (bg_color, border_color) = if is_selected {
        (rgb(0x22262e), rgb(0x384152))
    } else {
        (rgba(0x00000000), rgba(0x00000000))
    };

    let icon_color = if is_external {
        rgb(0x34d399) // Emerald-400
    } else {
        rgb(0x60a5fa) // Blue-400
    };

    let bar_fill_color = if used_pct > 90 {
        rgb(0xf87171) // Red-400
    } else if is_external {
        rgb(0x34d399) // Emerald-400
    } else {
        rgb(0x60a5fa) // Blue-400
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
        .hover(|s| s.bg(if is_selected { rgb(0x272c36) } else { rgb(0x1a1d24) }))
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
                                .text_color(if is_selected { rgb(0xf4f4f5) } else { rgb(0xd4d4d8) })
                                .overflow_hidden()
                                .child(drive_name),
                        ),
                )
                .child(
                    div()
                        .px(px(4.0))
                        .py(px(0.5))
                        .rounded(px(3.0))
                        .bg(rgb(0x181a20))
                        .text_color(rgb(0x71717a))
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
                        .bg(rgb(0x27272a))
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
                        .text_color(rgb(0x71717a))
                        .child(div().child(size_label))
                        .child(div().child(format!("{}%", used_pct))),
                ),
        )
}
