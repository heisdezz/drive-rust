use std::path::Path;
use gpui_kit::*;
use crate::drives::detector::format_bytes;
use crate::state::app_state::{AppRoute, AppState};
use crate::theme::icons::Icon;

/// Renders the complete Dashboard view following the reference design with compact Zed spacing and monochrome typography.
pub fn render_dashboard_view(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let selected_drive = state.selected_drive.clone();
    let drives = state.drives.clone();
    let db_opt = state.active_db.clone();

    div()
        .id(SharedString::from("dashboard-scroll-view"))
        .w_full()
        .h_full()
        .overflow_y_scroll()
        .flex()
        .flex_col()
        .gap(px(14.0))
        .child(
            if let Some(drive) = selected_drive {
                render_mounted_dashboard(app_state, &drive, db_opt.as_deref(), &theme, cx).into_any_element()
            } else {
                render_unmounted_dashboard(app_state, &drives, &theme).into_any_element()
            }
        )
}

/// Renders Dashboard when no drive is selected.
fn render_unmounted_dashboard(
    app_state: &Entity<AppState>,
    drives: &[crate::state::models::DriveInfo],
    theme: &crate::theme::AppTheme,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(14.0))
        // Compact Welcome Hero Banner (significantly smaller, subtle monochrome background)
        .child(
            div()
                .p(px(16.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_base()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("Welcome to Rust Drive"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_secondary)
                        .child("An ultra-lightweight desktop media library & photo explorer powered by native Rust and GPUI. Connect and select a storage volume to automatically catalog, preview, and search your photo albums."),
                ),
        )
        // Select Connected Drive Section
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_muted)
                .child("SELECT A CONNECTED DRIVE"),
        )
        .child(
            if drives.is_empty() {
                div()
                    .p(px(24.0))
                    .rounded(px(6.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.text_secondary)
                            .child("No storage drives detected"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("Connect a USB flash drive or external hard drive to automatically detect and index media."),
                    )
                    .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(drives.iter().map(|drive| {
                        let state_pick = app_state.clone();
                        let d = drive.clone();
                        let used_pct = drive.used_percentage;

                        div()
                            .id(SharedString::from(format!("dash-drive-{}", drive.id)))
                            .cursor_pointer()
                            .p(px(14.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_card)
                            .border_1()
                            .border_color(theme.border_card)
                            .hover(move |s| s.bg(theme.bg_card_hover).border_color(theme.border_active))
                            .on_click(move |_, _, cx| {
                                let drive_to_select = d.clone();
                                state_pick.update(cx, |s, cx| {
                                    s.select_drive(drive_to_select);
                                    cx.notify();
                                });
                            })
                            .flex()
                            .flex_col()
                            .gap(px(10.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(10.0))
                                            .child(
                                                div()
                                                    .w(px(32.0))
                                                    .h(px(32.0))
                                                    .rounded(px(4.0))
                                                    .bg(theme.bg_pill)
                                                    .border_1()
                                                    .border_color(theme.border_card)
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_sm()
                                                    .text_color(theme.text_secondary)
                                                    .child(Icon::DASHBOARD),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(2.0))
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(theme.text_primary)
                                                            .child(drive.name.clone()),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(theme.text_muted)
                                                            .child(drive.size_formatted.clone()),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .px(px(6.0))
                                            .py(px(2.0))
                                            .rounded(px(3.0))
                                            .bg(theme.bg_pill_active)
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.accent)
                                            .child("● Mounted"),
                                    ),
                            )
                            // Progress bar
                            .child(
                                div()
                                    .w_full()
                                    .h(px(4.0))
                                    .rounded(px(2.0))
                                    .bg(theme.bg_pill)
                                    .child(
                                        div()
                                            .w(px((used_pct as f32).clamp(0.0, 100.0) * 3.6))
                                            .h(px(4.0))
                                            .rounded(px(2.0))
                                            .bg(theme.accent),
                                    ),
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
                                            .child(format!("{}% used", used_pct)),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(format!("{} total", drive.size_formatted)),
                                    ),
                            )
                    }))
                    .into_any_element()
            },
        )
}

/// Renders Dashboard when a drive is actively selected and mounted.
fn render_mounted_dashboard(
    app_state: &Entity<AppState>,
    drive: &crate::state::models::DriveInfo,
    db_opt: Option<&crate::db::LibraryDatabase>,
    theme: &crate::theme::AppTheme,
    _cx: &mut App,
) -> impl IntoElement {
    let used_pct = drive.used_percentage;

    // Query stats and newest media
    let (recent_media, total_media_items) = if let Some(db) = db_opt {
        db.get_media_items(
            None,
            8,
            0,
            crate::state::models::MediaFilterType::All,
            None,
            crate::state::models::MediaSortBy::Date,
            crate::state::models::SortOrder::Desc,
        ).unwrap_or((Vec::new(), 0))
    } else {
        (Vec::new(), 0)
    };

    let albums = if let Some(db) = db_opt {
        db.get_albums().unwrap_or_default()
    } else {
        Vec::new()
    };
    let total_albums = albums.len();
    let recent_albums = albums.into_iter().take(4).collect::<Vec<_>>();

    let state_scan = app_state.clone();
    let state_all_media = app_state.clone();
    let state_all_albums = app_state.clone();
    let state_eject = app_state.clone();

    div()
        .flex()
        .flex_col()
        .gap(px(14.0))
        // Top Hero Drive Card
        .child(
            div()
                .p(px(14.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(12.0))
                        .child(
                            div()
                                .w(px(36.0))
                                .h(px(36.0))
                                .rounded(px(5.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_base()
                                .text_color(theme.accent)
                                .child(Icon::DASHBOARD),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(2.0))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(drive.name.clone()),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_muted)
                                        .child(format!(
                                            "{} • {} TOTAL",
                                            drive.mount_path.display(),
                                            drive.size_formatted
                                        )),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_secondary)
                                        .child(format!(
                                            "Album folder: {}/albums",
                                            drive.mount_path.display()
                                        )),
                                ),
                        ),
                )
                // Right Action Buttons
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(
                            div()
                                .id(SharedString::from("dash-scan-volume-btn"))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_primary)
                                .hover(move |s| s.bg(theme.bg_card_hover).border_color(theme.accent))
                                .on_click(move |_, _, cx| {
                                    state_scan.update(cx, |s, cx| {
                                        s.set_route(AppRoute::Discover);
                                        cx.notify();
                                    });
                                })
                                .child(format!("{} Scan Volume", Icon::DISCOVER)),
                        )
                        .child(
                            div()
                                .id(SharedString::from("dash-eject-drive-btn"))
                                .cursor_pointer()
                                .px(px(6.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .text_xs()
                                .text_color(theme.text_muted)
                                .hover(move |s| s.bg(theme.bg_card_hover).text_color(theme.accent_danger))
                                .on_click(move |_, _, cx| {
                                    state_eject.update(cx, |s, cx| {
                                        s.selected_drive = None;
                                        cx.notify();
                                    });
                                })
                                .child("Eject"),
                        ),
                ),
        )
        // Middle Row: 3 Metrics Cards (Total Media Items, Total Albums, Storage Used)
        .child(
            div()
                .flex()
                .gap(px(10.0))
                // Card 1: Total Media Items
                .child(
                    div()
                        .flex_1()
                        .p(px(14.0))
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_card)
                        .flex()
                        .items_center()
                        .gap(px(12.0))
                        .child(
                            div()
                                .w(px(34.0))
                                .h(px(34.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_base()
                                .text_color(theme.accent)
                                .child(Icon::MEDIA),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(1.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_muted)
                                        .child("TOTAL MEDIA ITEMS"),
                                )
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(format_number(total_media_items)),
                                ),
                        ),
                )
                // Card 2: Total Albums
                .child(
                    div()
                        .flex_1()
                        .p(px(14.0))
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_card)
                        .flex()
                        .items_center()
                        .gap(px(12.0))
                        .child(
                            div()
                                .w(px(34.0))
                                .h(px(34.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_base()
                                .text_color(theme.accent)
                                .child(Icon::ALBUMS),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(1.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_muted)
                                        .child("TOTAL ALBUMS"),
                                )
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(format_number(total_albums as i64)),
                                ),
                        ),
                )
                // Card 3: Storage Used
                .child(
                    div()
                        .flex_1()
                        .p(px(14.0))
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_card)
                        .flex()
                        .flex_col()
                        .justify_between()
                        .gap(px(6.0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_muted)
                                        .child("STORAGE USED"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(format!("{}%", used_pct)),
                                ),
                        )
                        .child(
                            div()
                                .w_full()
                                .h(px(4.0))
                                .rounded(px(2.0))
                                .bg(theme.bg_pill)
                                .child(
                                    div()
                                        .w(px((used_pct as f32).clamp(0.0, 100.0) * 2.2))
                                        .h(px(4.0))
                                        .rounded(px(2.0))
                                        .bg(theme.accent),
                                ),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(format!("{} total capacity", drive.size_formatted)),
                        ),
                ),
        )
        // Recently Added Media Section
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("RECENTLY ADDED MEDIA"),
                        )
                        .child(
                            div()
                                .id(SharedString::from("dash-view-all-media-btn"))
                                .cursor_pointer()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.accent)
                                .hover(move |s| s.text_color(theme.accent_hover))
                                .on_click(move |_, _, cx| {
                                    state_all_media.update(cx, |s, cx| {
                                        s.set_route(AppRoute::Medias);
                                        cx.notify();
                                    });
                                })
                                .child("View All Media →"),
                        ),
                )
                .child(
                    if recent_media.is_empty() {
                        div()
                            .p(px(20.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_card)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("No media cataloged yet. Click 'Scan Volume' above to begin indexing."),
                            )
                            .into_any_element()
                    } else {
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(8.0))
                            .children(recent_media.into_iter().map(|item| {
                                let item_id = item.id;
                                let item_clone = item.clone();
                                let is_video = item.mime_type.starts_with("video/");
                                let filename = Path::new(&item.current_relative_path)
                                    .file_name()
                                    .map(|s| s.to_string_lossy().to_string())
                                    .unwrap_or_else(|| item.current_relative_path.clone());

                                let state_click = app_state.clone();

                                div()
                                    .id(SharedString::from(format!("recent-media-{}", item_id)))
                                    .cursor_pointer()
                                    .w(px(108.0))
                                    .h(px(86.0))
                                    .rounded(px(5.0))
                                    .bg(theme.bg_card)
                                    .border_1()
                                    .border_color(theme.border_card)
                                    .hover(move |s| s.bg(theme.bg_card_hover).border_color(theme.accent))
                                    .on_click(move |_, _, cx| {
                                        let it = item_clone.clone();
                                        state_click.update(cx, |s, cx| {
                                            s.open_player(it, None, None);
                                            cx.notify();
                                        });
                                    })
                                    .flex()
                                    .flex_col()
                                    .justify_between()
                                    .p(px(6.0))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_muted)
                                                    .child(format_bytes(item.file_size as u64)),
                                            )
                                            .child(
                                                div()
                                                    .px(px(3.0))
                                                    .py(px(0.5))
                                                    .rounded(px(2.0))
                                                    .bg(theme.bg_pill)
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(theme.text_muted)
                                                    .child(if is_video { "VID" } else { "IMG" }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .justify_center()
                                            .text_sm()
                                            .text_color(theme.text_secondary)
                                            .child(if is_video { Icon::VIDEO } else { Icon::IMAGE }),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_secondary)
                                            .overflow_hidden()
                                            .child(truncate_filename(&filename, 14)),
                                    )
                            }))
                            .into_any_element()
                    },
                ),
        )
        // Recent Albums Section
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("RECENT ALBUMS"),
                        )
                        .child(
                            div()
                                .id(SharedString::from("dash-view-all-albums-btn"))
                                .cursor_pointer()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.accent)
                                .hover(move |s| s.text_color(theme.accent_hover))
                                .on_click(move |_, _, cx| {
                                    state_all_albums.update(cx, |s, cx| {
                                        s.set_route(AppRoute::Albums);
                                        s.select_album(None);
                                        cx.notify();
                                    });
                                })
                                .child("View All Albums →"),
                        ),
                )
                .child(
                    if recent_albums.is_empty() {
                        div()
                            .p(px(20.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_card)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("No albums found. Create an album or scan folders from Discover."),
                            )
                            .into_any_element()
                    } else {
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(10.0))
                            .children(recent_albums.into_iter().map(|album| {
                                let album_id = album.id;
                                let album_name = album.name.clone();
                                let media_count = album.media_count;
                                let created_date = album.created_at.split(' ').next().unwrap_or("").to_string();
                                let state_pick_album = app_state.clone();

                                div()
                                    .id(SharedString::from(format!("recent-album-{}", album_id)))
                                    .cursor_pointer()
                                    .w(px(220.0))
                                    .p(px(10.0))
                                    .rounded(px(5.0))
                                    .bg(theme.bg_card)
                                    .border_1()
                                    .border_color(theme.border_card)
                                    .hover(move |s| s.bg(theme.bg_card_hover).border_color(theme.accent))
                                    .on_click(move |_, _, cx| {
                                        state_pick_album.update(cx, |s, cx| {
                                            s.set_route(AppRoute::Albums);
                                            s.select_album(Some(album_id));
                                            cx.notify();
                                        });
                                    })
                                    .flex()
                                    .items_center()
                                    .gap(px(10.0))
                                    .child(
                                        div()
                                            .w(px(40.0))
                                            .h(px(40.0))
                                            .rounded(px(4.0))
                                            .bg(theme.bg_sidebar)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_sm()
                                            .text_color(theme.text_muted)
                                            .child(Icon::FOLDER),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(1.0))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(theme.text_primary)
                                                    .child(album_name),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_secondary)
                                                    .child(format!("{} items", media_count)),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_muted)
                                                    .child(format!("Created: {}", created_date)),
                                            ),
                                    )
                            }))
                            .into_any_element()
                    },
                ),
        )
}

fn truncate_filename(name: &str, max_len: usize) -> String {
    if name.chars().count() <= max_len {
        name.to_string()
    } else {
        let prefix: String = name.chars().take(max_len.saturating_sub(3)).collect();
        format!("{}...", prefix)
    }
}

fn format_number(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    for (i, &c) in chars.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{format_number, truncate_filename};

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(932), "932");
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(15278), "15,278");
        assert_eq!(format_number(1234567), "1,234,567");
    }

    #[test]
    fn test_truncate_filename() {
        assert_eq!(truncate_filename("short.jpg", 14), "short.jpg");
        assert_eq!(truncate_filename("a_very_long_media_filename.mp4", 14), "a_very_long...");
    }
}
