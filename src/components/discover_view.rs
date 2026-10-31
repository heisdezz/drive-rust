use std::sync::atomic::{AtomicBool, Ordering};
use gpui_kit::*;
use crate::scanner::walker::crawl_directory;
use crate::scanner::relocator::relocate_media;
use crate::state::app_state::AppState;
use crate::theme::icons::Icon;

const SUGGESTED_IGNORES: &[&str] = &[
    "temp",
    "cache",
    "raw_dumps",
    "backups",
    "downloads",
    "exports",
    "thumbnails",
    "node_modules",
];

const COMMON_SUBFOLDERS: &[&str] = &[
    "DCIM",
    "Pictures",
    "Videos",
    "Photos",
    "Camera",
    "GoPro",
    "DJI",
];

// Global atomic controls for active crawl loop
static SCAN_PAUSED: AtomicBool = AtomicBool::new(false);
static SCAN_ABORTED: AtomicBool = AtomicBool::new(false);

/// Renders the Media Discovery, Ignore Configuration, and Auto-Catalog Scanner view
/// with strict monochrome typography, subfolder targeting, and pause/resume/abort lifecycle controls.
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
    let current_subfolder = state.scan_subfolder.clone();

    let state_start = app_state.clone();
    let state_pause = app_state.clone();
    let state_resume = app_state.clone();
    let state_abort = app_state.clone();
    let state_reset = app_state.clone();

    div()
        .id(SharedString::from("discover-view-container"))
        .flex()
        .flex_col()
        .gap(px(12.0))
        .w_full()
        .h_full()
        .overflow_y_scroll()
        // Card 1: Target Storage Drive & Scan Directory with Subfolder Targeting
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
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(format!("{} Media Discovery & Auto-Catalog Scanner", Icon::DISCOVER)),
                                )
                                .child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(1.5))
                                        .rounded(px(3.0))
                                        .bg(if scan_progress.is_scanning {
                                            if scan_progress.is_paused {
                                                theme.bg_pill
                                            } else {
                                                theme.bg_pill_active
                                            }
                                        } else if scan_progress.found_count > 0 {
                                            theme.bg_pill
                                        } else {
                                            theme.bg_pill
                                        })
                                        .border_1()
                                        .border_color(if scan_progress.is_scanning {
                                            if scan_progress.is_paused {
                                                theme.border_card
                                            } else {
                                                theme.accent
                                            }
                                        } else if scan_progress.found_count > 0 {
                                            theme.accent_success
                                        } else {
                                            theme.border_subtle
                                        })
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(if scan_progress.is_scanning {
                                            if scan_progress.is_paused {
                                                theme.text_secondary
                                            } else {
                                                theme.accent
                                            }
                                        } else if scan_progress.found_count > 0 {
                                            theme.accent_success
                                        } else {
                                            theme.text_muted
                                        })
                                        .child(if scan_progress.is_scanning {
                                            if scan_progress.is_paused {
                                                format!("{} Paused", Icon::PAUSE)
                                            } else {
                                                format!("{} Scanning...", Icon::DOT)
                                            }
                                        } else if scan_progress.is_stopped {
                                            format!("{} Aborted", Icon::STOP)
                                        } else if scan_progress.found_count > 0 {
                                            format!("{} Scan Complete", Icon::CHECK)
                                        } else {
                                            format!("{} Ready", Icon::CIRCLE)
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
                        let active_scan_path = if let Some(ref sub) = current_subfolder {
                            drive.mount_path.join(sub)
                        } else {
                            drive.mount_path.clone()
                        };

                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
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
                                                    .child(format!("Target Path: {} • Destination: {}/albums", active_scan_path.display(), drive.mount_path.display())),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .px(px(6.0))
                                            .py(px(2.0))
                                            .rounded(px(3.0))
                                            .bg(theme.bg_pill)
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.text_secondary)
                                            .child(format!("{} ({}% used)", drive.size_formatted, drive.used_percentage)),
                                    ),
                            )
                            // Subfolder Target Selector Pills
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.text_muted)
                                            .child("SCAN SCOPE:"),
                                    )
                                    // Entire Drive option
                                    .child({
                                        let is_root = current_subfolder.is_none();
                                        let state_scope = app_state.clone();
                                        div()
                                            .id(SharedString::from("scan-scope-root"))
                                            .cursor_pointer()
                                            .px(px(7.0))
                                            .py(px(2.0))
                                            .rounded(px(3.0))
                                            .bg(if is_root { theme.bg_pill_active } else { theme.bg_pill })
                                            .border_1()
                                            .border_color(if is_root { theme.accent } else { theme.border_card })
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(if is_root { theme.accent } else { theme.text_secondary })
                                            .on_click(move |_, _, cx| {
                                                state_scope.update(cx, |s, cx| {
                                                    s.set_scan_subfolder(None);
                                                    cx.notify();
                                                });
                                            })
                                            .child(format!("{} Entire Drive", if is_root { "●" } else { "○" }))
                                    })
                                    .children(COMMON_SUBFOLDERS.iter().map(|&sub| {
                                        let is_selected = current_subfolder.as_deref() == Some(sub);
                                        let state_sub = app_state.clone();
                                        let sub_str = sub.to_string();

                                        div()
                                            .id(SharedString::from(format!("scan-scope-{}", sub)))
                                            .cursor_pointer()
                                            .px(px(7.0))
                                            .py(px(2.0))
                                            .rounded(px(3.0))
                                            .bg(if is_selected { theme.bg_pill_active } else { theme.bg_pill })
                                            .border_1()
                                            .border_color(if is_selected { theme.accent } else { theme.border_card })
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(if is_selected { theme.accent } else { theme.text_secondary })
                                            .on_click(move |_, _, cx| {
                                                state_sub.update(cx, |s, cx| {
                                                    if is_selected {
                                                        s.set_scan_subfolder(None);
                                                    } else {
                                                        s.set_scan_subfolder(Some(sub_str.clone()));
                                                    }
                                                    cx.notify();
                                                });
                                            })
                                            .child(format!("{} {}", if is_selected { "●" } else { "○" }, sub))
                                    }))
                            )
                    } else {
                        div()
                            .p(px(10.0))
                            .rounded(px(4.0))
                            .bg(theme.bg_sidebar)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(format!("{} No external drive selected. Please mount or select a volume from the sidebar.", Icon::INFO))
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
                                .child("PRESET FILTERS:"),
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
                                                .child(format!("{}+ {}", if is_already_added { format!("{} ", Icon::CHECK) } else { "".to_string() }, preset)),
                                        )
                                })),
                        ),
                )
                // Active Ignore Rules List
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
                                .child("ACTIVE IGNORES:"),
                        )
                        .child(
                            if ignore_patterns.is_empty() {
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("No custom ignore patterns configured. Built-in defaults active.")
                                    .into_any_element()
                            } else {
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
                                                    .child(Icon::CLOSE),
                                            )
                                    }))
                                    .into_any_element()
                            },
                        ),
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
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.text_primary)
                                .child("Live Scanner Metrics"),
                        )
                        .child(
                            if scan_progress.is_scanning {
                                div()
                                    .text_xs()
                                    .text_color(if scan_progress.is_paused { theme.text_secondary } else { theme.accent })
                                    .child(if scan_progress.is_paused {
                                        format!("{} Paused", Icon::PAUSE)
                                    } else {
                                        format!("{} Processing...", Icon::DOT)
                                    })
                            } else {
                                div()
                            },
                        ),
                )
                // Progress stats grid
                .child(
                    div()
                        .flex()
                        .gap(px(10.0))
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
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_muted)
                                        .child("FILES CRAWLED"),
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
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_muted)
                                        .child("MEDIA CATALOGED"),
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
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_muted)
                                        .child("ACTIVE IGNORES"),
                                )
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child(format!("{}", ignore_patterns.len())),
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
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_muted)
                                        .child("STATUS"),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if scan_progress.is_scanning {
                                            if scan_progress.is_paused {
                                                theme.text_secondary
                                            } else {
                                                theme.accent
                                            }
                                        } else if scan_progress.is_stopped {
                                            theme.accent_danger
                                        } else if scan_progress.found_count > 0 {
                                            theme.accent_success
                                        } else {
                                            theme.text_secondary
                                        })
                                        .child(if scan_progress.is_scanning {
                                            if scan_progress.is_paused {
                                                "Paused"
                                            } else {
                                                "Scanning"
                                            }
                                        } else if scan_progress.is_stopped {
                                            "Aborted"
                                        } else if scan_progress.found_count > 0 {
                                            "Finished"
                                        } else {
                                            "Idle"
                                        }),
                                )
                        ),
                )
                // Current Folder
                .child(
                    if !scan_progress.current_folder.is_empty() {
                        div()
                            .p(px(8.0))
                            .rounded(px(4.0))
                            .bg(theme.bg_sidebar)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .text_color(theme.text_secondary)
                            .overflow_hidden()
                            .child(format!("Processing folder: {}", scan_progress.current_folder))
                    } else {
                        div()
                    },
                )
                // Action Buttons: Start / Pause / Resume / Abort / Reset
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        // Primary Scan / Pause / Resume / Abort Buttons
                        .child(
                            if !scan_progress.is_scanning {
                                let can_scan = drive_opt.is_some() && db_opt.is_some();
                                let drive = drive_opt.clone();
                                let db = db_opt.clone();
                                let user_patterns = ignore_patterns.clone();
                                let target_sub = current_subfolder.clone();

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
                                        let sub = target_sub.clone();

                                        SCAN_PAUSED.store(false, Ordering::SeqCst);
                                        SCAN_ABORTED.store(false, Ordering::SeqCst);

                                        state.update(cx, |s, cx| {
                                            s.scan_progress.is_scanning = true;
                                            s.scan_progress.is_paused = false;
                                            s.scan_progress.is_stopped = false;
                                            s.scan_progress.scanned_count = 0;
                                            s.scan_progress.found_count = 0;
                                            cx.notify();
                                        });

                                        cx.spawn(move |cx: &mut AsyncApp| {
                                            let cx = cx.clone();
                                            let state = state.clone();
                                            let scan_root = if let Some(ref s) = sub {
                                                drive.mount_path.join(s)
                                            } else {
                                                drive.mount_path.clone()
                                            };

                                            async move {
                                                // Run filesystem crawl on background executor
                                                let discovered = cx
                                                    .background_executor()
                                                    .spawn(async move {
                                                        crawl_directory(&scan_root, &patterns).unwrap_or_default()
                                                    })
                                                    .await;

                                                let total_discovered = discovered.len();
                                                let mut imported = 0;

                                                for item in discovered {
                                                    // Check if aborted
                                                    if SCAN_ABORTED.load(Ordering::SeqCst) {
                                                        let _ = state.update(&mut cx.clone(), |s, cx| {
                                                            s.scan_progress.is_scanning = false;
                                                            s.scan_progress.is_stopped = true;
                                                            s.scan_progress.current_folder = "Aborted".to_string();
                                                            cx.notify();
                                                        });
                                                        return;
                                                    }

                                                    // Active pause check
                                                    while SCAN_PAUSED.load(Ordering::SeqCst) {
                                                        std::thread::sleep(std::time::Duration::from_millis(150));
                                                        if SCAN_ABORTED.load(Ordering::SeqCst) {
                                                            let _ = state.update(&mut cx.clone(), |s, cx| {
                                                                s.scan_progress.is_scanning = false;
                                                                s.scan_progress.is_stopped = true;
                                                                s.scan_progress.current_folder = "Aborted".to_string();
                                                                cx.notify();
                                                            });
                                                            return;
                                                        }
                                                    }

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
                                                    s.scan_progress.is_paused = false;
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
                                            .child(format!("{} Start Discovery Scan", Icon::PLAY)),
                                    )
                                    .into_any_element()
                            } else {
                                // Scanning is active: Provide Pause / Resume and Abort controls
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        if !scan_progress.is_paused {
                                            div()
                                                .id(SharedString::from("pause-scan-btn"))
                                                .cursor_pointer()
                                                .px(px(10.0))
                                                .py(px(6.0))
                                                .rounded(px(4.0))
                                                .bg(theme.bg_pill)
                                                .border_1()
                                                .border_color(theme.border_card)
                                                .hover(move |s| s.bg(theme.bg_card_hover).border_color(theme.accent))
                                                .on_click(move |_, _, cx| {
                                                    SCAN_PAUSED.store(true, Ordering::SeqCst);
                                                    state_pause.update(cx, |s, cx| {
                                                        s.pause_scan();
                                                        cx.notify();
                                                    });
                                                })
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.text_primary)
                                                        .child(format!("{} Pause", Icon::PAUSE)),
                                                )
                                                .into_any_element()
                                        } else {
                                            div()
                                                .id(SharedString::from("resume-scan-btn"))
                                                .cursor_pointer()
                                                .px(px(10.0))
                                                .py(px(6.0))
                                                .rounded(px(4.0))
                                                .bg(theme.accent)
                                                .text_color(rgb(0xffffff))
                                                .hover(move |s| s.bg(theme.accent_hover))
                                                .on_click(move |_, _, cx| {
                                                    SCAN_PAUSED.store(false, Ordering::SeqCst);
                                                    state_resume.update(cx, |s, cx| {
                                                        s.resume_scan();
                                                        cx.notify();
                                                    });
                                                })
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child(format!("{} Resume", Icon::PLAY)),
                                                )
                                                .into_any_element()
                                        }
                                    )
                                    .child(
                                        div()
                                            .id(SharedString::from("abort-scan-btn"))
                                            .cursor_pointer()
                                            .px(px(10.0))
                                            .py(px(6.0))
                                            .rounded(px(4.0))
                                            .bg(theme.bg_pill)
                                            .border_1()
                                            .border_color(theme.accent_danger)
                                            .hover(move |s| s.bg(theme.bg_card_hover))
                                            .on_click(move |_, _, cx| {
                                                SCAN_ABORTED.store(true, Ordering::SeqCst);
                                                state_abort.update(cx, |s, cx| {
                                                    s.abort_scan();
                                                    cx.notify();
                                                });
                                            })
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.accent_danger)
                                                    .child(format!("{} Abort", Icon::STOP)),
                                            ),
                                    )
                                    .into_any_element()
                            }
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
