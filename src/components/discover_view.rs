use gpui_kit::*;
use crate::scanner::walker::crawl_directory;
use crate::scanner::relocator::relocate_media;
use crate::state::app_state::AppState;

const SUGGESTED_IGNORES: &[&str] = &["temp", "cache", "raw_dumps", "backups", "downloads", "exports"];

/// Renders the Media Discovery, Ignore Configuration, and Auto-Catalog Scanner view.
pub fn render_discover_view(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let drive_opt = state.selected_drive.clone();
    let db_opt = state.active_db.clone();
    let scan_progress = state.scan_progress.clone();
    let ignore_patterns = state.user_ignore_patterns.clone();

    let state_start = app_state.clone();
    let state_reset = app_state.clone();

    div()
        .id(SharedString::from("discover-view-container"))
        .flex()
        .flex_col()
        .gap(px(12.0))
        .w_full()
        .h_full()
        .overflow_y_scroll()
        // Card 1: Target Storage Drive & Scan Directory
        .child(
            div()
                .p(px(14.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
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
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_primary)
                                        .child("🧭 Media Discovery & Auto-Catalog Scanner"),
                                )
                                .child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(1.5))
                                        .rounded(px(3.0))
                                        .bg(if scan_progress.is_scanning {
                                            theme.accent_warning
                                        } else if scan_progress.found_count > 0 {
                                            theme.accent_success
                                        } else {
                                            theme.bg_pill
                                        })
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(if scan_progress.is_scanning || scan_progress.found_count > 0 {
                                            rgb(0xffffff)
                                        } else {
                                            theme.text_muted
                                        })
                                        .child(if scan_progress.is_scanning {
                                            "Scanning..."
                                        } else if scan_progress.found_count > 0 {
                                            "Scan Complete"
                                        } else {
                                            "Idle"
                                        }),
                                ),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("Auto-classifies files into albums/<folder>/"),
                        ),
                )
                .child(
                    if let Some(drive) = &drive_opt {
                        div()
                            .p(px(10.0))
                            .rounded(px(4.0))
                            .bg(theme.bg_sidebar)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_primary)
                                            .child(format!("Target Drive: {}", drive.name)),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(format!("Root Path: {}", drive.mount_path.display())),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.accent)
                                    .child(format!("{} ({}% used)", drive.size_formatted, drive.used_percentage)),
                            )
                    } else {
                        div()
                            .p(px(10.0))
                            .rounded(px(4.0))
                            .bg(theme.bg_sidebar)
                            .text_xs()
                            .text_color(theme.accent_warning)
                            .child("⚠️ No external drive selected. Please mount or select a volume from the sidebar.")
                    },
                ),
        )
        // Card 2: Folder Ignore Rules Engine
        .child(
            div()
                .p(px(14.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
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
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.text_primary)
                                .child("Folder Ignore Patterns"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("Skipped during discovery (built-in skips: albums/, .* hidden folders, node_modules/)"),
                        ),
                )
                // Suggested Presets
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_muted)
                                .child("ADD PRESETS:"),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap(px(6.0))
                                .children(SUGGESTED_IGNORES.iter().map(|&preset| {
                                    let state_preset = app_state.clone();
                                    let is_already_added = ignore_patterns.contains(&preset.to_string());

                                    div()
                                        .id(SharedString::from(format!("ignore-preset-{}", preset)))
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .rounded(px(3.0))
                                        .bg(if is_already_added { theme.bg_pill_active } else { theme.bg_pill })
                                        .border_1()
                                        .border_color(if is_already_added { theme.accent } else { theme.border_card })
                                        .hover(move |s| s.border_color(theme.accent))
                                        .on_click(move |_, _, cx| {
                                            state_preset.update(cx, |s, cx| {
                                                if is_already_added {
                                                    s.remove_ignore_pattern(preset);
                                                } else {
                                                    s.add_ignore_pattern(preset);
                                                }
                                                cx.notify();
                                            });
                                        })
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(if is_already_added { theme.accent } else { theme.text_secondary })
                                                .child(format!("{}+ {}", if is_already_added { "✓ " } else { "" }, preset)),
                                        )
                                })),
                        ),
                )
                // Active Ignore Rules List
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(6.0))
                        .children(ignore_patterns.iter().map(|pat| {
                            let state_del = app_state.clone();
                            let pattern_name = pat.clone();

                            div()
                                .id(SharedString::from(format!("active-ignore-{}", pattern_name)))
                                .px(px(8.0))
                                .py(px(3.0))
                                .rounded(px(3.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_primary)
                                        .child(pattern_name.clone()),
                                )
                                .child(
                                    div()
                                        .id(SharedString::from(format!("remove-ignore-{}", pattern_name)))
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .hover(move |s| s.text_color(theme.accent_danger))
                                        .on_click(move |_, _, cx| {
                                            state_del.update(cx, |s, cx| {
                                                s.remove_ignore_pattern(&pattern_name);
                                                cx.notify();
                                            });
                                        })
                                        .child("✕"),
                                )
                        })),
                ),
        )
        // Card 3: Live Scanner Progress & Action Controls
        .child(
            div()
                .p(px(14.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_card)
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_primary)
                        .child("Live Scanner Metrics"),
                )
                // Progress stats grid
                .child(
                    div()
                        .flex()
                        .gap(px(12.0))
                        .child(
                            div()
                                .flex_1()
                                .p(px(10.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_sidebar)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("Files Crawled"),
                                )
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(format!("{}", scan_progress.scanned_count)),
                                )
                        )
                        .child(
                            div()
                                .flex_1()
                                .p(px(10.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_sidebar)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("Media Cataloged"),
                                )
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent_success)
                                        .child(format!("{}", scan_progress.found_count)),
                                )
                        )
                        .child(
                            div()
                                .flex_1()
                                .p(px(10.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_sidebar)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("Active Ignore Filters"),
                                )
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child(format!("{}", ignore_patterns.len())),
                                )
                        ),
                )
                // Current Folder
                .child(
                    if !scan_progress.current_folder.is_empty() {
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .overflow_hidden()
                            .child(format!("Processing: {}", scan_progress.current_folder))
                    } else {
                        div()
                    },
                )
                // Action Buttons: Start Scan / Reset
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        // Start Scan Button
                        .child(
                            {
                                let can_scan = drive_opt.is_some() && db_opt.is_some() && !scan_progress.is_scanning;
                                let drive = drive_opt.clone();
                                let db = db_opt.clone();
                                let user_patterns = ignore_patterns.clone();

                                div()
                                    .id(SharedString::from("start-scan-btn"))
                                    .cursor_pointer()
                                    .px(px(14.0))
                                    .py(px(6.0))
                                    .rounded(px(4.0))
                                    .bg(if can_scan { theme.accent } else { theme.bg_pill })
                                    .border_1()
                                    .border_color(if can_scan { theme.accent } else { theme.border_card })
                                    .hover(move |s| if can_scan { s.bg(theme.accent_hover) } else { s })
                                    .on_click(move |_, _, cx| {
                                        if !can_scan {
                                            return;
                                        }

                                        let drive = match &drive {
                                            Some(d) => d.clone(),
                                            None => return,
                                        };
                                        let db = match &db {
                                            Some(d) => d.clone(),
                                            None => return,
                                        };

                                        let patterns = user_patterns.clone();
                                        let state = state_start.clone();

                                        state.update(cx, |s, cx| {
                                            s.scan_progress.is_scanning = true;
                                            s.scan_progress.scanned_count = 0;
                                            s.scan_progress.found_count = 0;
                                            cx.notify();
                                        });

                                        cx.spawn(move |cx: &mut AsyncApp| {
                                            let cx = cx.clone();
                                            let state = state.clone();
                                            let mount_path = drive.mount_path.clone();

                                            async move {
                                                // Run filesystem crawl on background executor
                                                let discovered = cx
                                                    .background_executor()
                                                    .spawn(async move {
                                                        crawl_directory(&mount_path, &patterns).unwrap_or_default()
                                                    })
                                                    .await;

                                                let total_discovered = discovered.len();
                                                let mut imported = 0;

                                                for item in discovered {
                                                    let mount = drive.mount_path.clone();
                                                    let db_ref = db.clone();
                                                    let item_clone = item.clone();

                                                    let res = cx
                                                        .background_executor()
                                                        .spawn(async move {
                                                            relocate_media(&mount, &db_ref, &item_clone)
                                                        })
                                                        .await;

                                                    if res.is_ok() {
                                                        imported += 1;
                                                    }

                                                    let current_name = item.derived_album_name.clone();
                                                    let _ = state.update(&mut cx.clone(), |s, cx| {
                                                        s.scan_progress.scanned_count += 1;
                                                        s.scan_progress.found_count = imported;
                                                        s.scan_progress.current_folder = current_name;
                                                        cx.notify();
                                                    });
                                                }

                                                let _ = state.update(&mut cx.clone(), |s, cx| {
                                                    s.scan_progress.is_scanning = false;
                                                    s.scan_progress.scanned_count = total_discovered;
                                                    s.scan_progress.found_count = imported;
                                                    s.scan_progress.current_folder = "Complete".to_string();
                                                    cx.notify();
                                                });
                                            }
                                        }).detach();
                                    })
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(if can_scan { rgb(0xffffff) } else { theme.text_muted })
                                            .child(if scan_progress.is_scanning { "Scanning..." } else { "▶ Start Discovery Scan" }),
                                    )
                            },
                        )
                        // Reset Metrics Button
                        .child(
                            div()
                                .id(SharedString::from("reset-scan-btn"))
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(6.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .hover(move |s| s.bg(theme.bg_card_hover).text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_reset.update(cx, |s, cx| {
                                        s.reset_scan_progress();
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_secondary)
                                        .child("Reset Metrics"),
                                ),
                        ),
                ),
        )
}
