use gpui_kit::*;
use crate::state::models::{DriveInfo, DriveType};

/// Renders a single drive card in the sidebar.
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
        DriveType::Internal => "NVME",
        DriveType::Network => "NET",
    };

    // Border and background styling based on selection state
    let border_color = if is_selected {
        rgb(0x3b82f6) // Blue-500
    } else {
        rgb(0x334155) // Slate-700
    };

    let bg_color = if is_selected {
        rgb(0x1e293b) // Slate-800
    } else {
        rgba(0x0f172a80) // Semi-transparent dark
    };

    let icon_color = if is_external {
        rgb(0x10b981) // Emerald-500
    } else {
        rgb(0x60a5fa) // Blue-400
    };

    let bar_fill_color = if used_pct > 90 {
        rgb(0xef4444) // Red-500
    } else if is_external {
        rgb(0x10b981) // Emerald-500
    } else {
        rgb(0x3b82f6) // Blue-500
    };

    div()
        .id(SharedString::from(format!("drive-card-{}", drive.id)))
        .cursor_pointer()
        .px(px(10.0))
        .py(px(8.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(border_color)
        .bg(bg_color)
        .hover(|s| s.bg(rgb(0x334155)))
        .on_click(on_click)
        .child(
            // Top Row: Icon + Name + Type Badge
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
                            // Disk Icon indicator
                            div()
                                .w(px(8.0))
                                .h(px(8.0))
                                .rounded(px(4.0))
                                .bg(icon_color)
                                .flex_shrink_0(),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0xf1f5f9))
                                .overflow_hidden()
                                .child(drive_name),
                        ),
                )
                .child(
                    div()
                        .px(px(4.0))
                        .py(px(1.0))
                        .rounded(px(3.0))
                        .bg(rgb(0x334155))
                        .text_color(rgb(0x94a3b8))
                        .child(type_label),
                ),
        )
        // Bottom Row: Size + Progress Bar + %
        .child(
            div()
                .mt(px(6.0))
                .child(
                    // Storage Progress Bar track
                    div()
                        .w_full()
                        .h(px(4.0))
                        .rounded(px(2.0))
                        .bg(rgb(0x334155))
                        .overflow_hidden()
                        .child(
                            div()
                                .h_full()
                                .w(relative(used_pct as f32 / 100.0))
                                .bg(bar_fill_color)
                                .rounded(px(2.0)),
                        ),
                )
                .child(
                    div()
                        .mt(px(4.0))
                        .flex()
                        .items_center()
                        .justify_between()
                        .text_color(rgb(0x64748b))
                        .child(div().child(size_label))
                        .child(div().child(format!("{}% used", used_pct))),
                ),
        )
}
