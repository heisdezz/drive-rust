use std::fs;
use gpui_kit::*;
use crate::scanner::relocator::resolve_collision;
use crate::state::app_state::AppState;

const PRESET_NAMES: &[&str] = &[
    "Vacation", "Family", "Screenshots", "Wallpapers", "Camera Roll", "Favorites",
];

/// Renders the modal overlay for creating a new album directory.
pub fn render_create_album_modal(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let drive_opt = state.selected_drive.clone();
    let db_opt = state.active_db.clone();

    let state_cancel = app_state.clone();

    div()
        .id(SharedString::from("create-album-modal-overlay"))
        .absolute()
        .inset_0()
        .bg(rgba(0x00000088))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .w(px(380.0))
                .p(px(16.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_active)
                .flex()
                .flex_col()
                .gap(px(12.0))
                // Title
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("📁 Create New Album"),
                        )
                        .child(
                            div()
                                .id(SharedString::from("close-create-album-btn"))
                                .cursor_pointer()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .hover(move |s| s.text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_cancel.update(cx, |s, cx| {
                                        s.close_create_album_modal();
                                        cx.notify();
                                    });
                                })
                                .child("✕"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_secondary)
                        .child("Choose a name or preset for your new album directory on the active drive."),
                )
                // Preset options
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_muted)
                                .child("QUICK PRESETS:"),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap(px(6.0))
                                .children(PRESET_NAMES.iter().map(|&preset_name| {
                                    let state_create = app_state.clone();
                                    let drive = drive_opt.clone();
                                    let db = db_opt.clone();

                                    div()
                                        .id(SharedString::from(format!("preset-{}", preset_name)))
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(4.0))
                                        .rounded(px(4.0))
                                        .bg(theme.bg_pill)
                                        .border_1()
                                        .border_color(theme.border_card)
                                        .hover(move |s| s.bg(theme.bg_pill_active).border_color(theme.accent))
                                        .on_click(move |_, _, cx| {
                                            if let (Some(db), Some(drive)) = (&db, &drive) {
                                                // Create on disk
                                                let album_dir = drive.mount_path.join("albums").join(preset_name);
                                                let _ = fs::create_dir_all(&album_dir);

                                                // Create in DB
                                                let _ = db.create_album(preset_name, preset_name, None);
                                            }
                                            state_create.update(cx, |s, cx| {
                                                s.close_create_album_modal();
                                                cx.notify();
                                            });
                                        })
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(theme.text_primary)
                                                .child(preset_name),
                                        )
                                })),
                        ),
                )
                // Footer
                .child(
                    div()
                        .mt(px(8.0))
                        .flex()
                        .justify_end()
                        .child(
                            {
                                let state_close = app_state.clone();
                                div()
                                    .id(SharedString::from("cancel-create-album-btn"))
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(4.0))
                                    .bg(theme.bg_pill)
                                    .border_1()
                                    .border_color(theme.border_card)
                                    .text_xs()
                                    .text_color(theme.text_secondary)
                                    .hover(move |s| s.bg(theme.bg_card_hover).text_color(theme.text_primary))
                                    .on_click(move |_, _, cx| {
                                        state_close.update(cx, |s, cx| {
                                            s.close_create_album_modal();
                                            cx.notify();
                                        });
                                    })
                                    .child("Cancel")
                            },
                        ),
                ),
        )
}

/// Renders the modal overlay for editing, managing, or merging an existing album.
pub fn render_edit_album_modal(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let editing_album = state.editing_album.clone();

    let state_cancel = app_state.clone();

    let album = match editing_album {
        Some(a) => a,
        None => return div().into_any_element(),
    };

    let album_name = album.name.clone();
    let media_count = album.media_count;

    let state_merge = app_state.clone();
    let state_delete = app_state.clone();
    let album_for_merge = album.clone();
    let album_for_del = album.clone();

    div()
        .id(SharedString::from("edit-album-modal-overlay"))
        .absolute()
        .inset_0()
        .bg(rgba(0x00000088))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .w(px(380.0))
                .p(px(16.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_active)
                .flex()
                .flex_col()
                .gap(px(12.0))
                // Title
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(format!("✎ Manage Album: {}", album_name)),
                        )
                        .child(
                            div()
                                .id(SharedString::from("close-edit-album-btn"))
                                .cursor_pointer()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .hover(move |s| s.text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_cancel.update(cx, |s, cx| {
                                        s.close_edit_album_modal();
                                        cx.notify();
                                    });
                                })
                                .child("✕"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_secondary)
                        .child(format!("Contains {} cataloged media items in albums/{}.", media_count, album.relative_path)),
                )
                // Actions
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        // Merge Album Button
                        .child(
                            div()
                                .id(SharedString::from("open-merge-album-btn"))
                                .cursor_pointer()
                                .p(px(8.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .hover(move |s| s.bg(theme.bg_pill_active).border_color(theme.accent))
                                .on_click(move |_, _, cx| {
                                    let a = album_for_merge.clone();
                                    state_merge.update(cx, |s, cx| {
                                        s.close_edit_album_modal();
                                        s.open_merge_modal(a);
                                        cx.notify();
                                    });
                                })
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
                                                .child("Merge Album..."),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("Combine into another album"),
                                        ),
                                ),
                        )
                        // Delete Album Button
                        .child(
                            div()
                                .id(SharedString::from("open-delete-album-btn"))
                                .cursor_pointer()
                                .p(px(8.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.accent_danger)
                                .hover(move |s| s.bg(theme.accent_danger).text_color(rgb(0xffffff)))
                                .on_click(move |_, _, cx| {
                                    let a = album_for_del.clone();
                                    state_delete.update(cx, |s, cx| {
                                        s.close_edit_album_modal();
                                        s.open_delete_modal(a);
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.accent_danger)
                                                .child("Delete Album..."),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("Choose safe or permanent delete"),
                                        ),
                                ),
                        ),
                )
                // Footer
                .child(
                    div()
                        .mt(px(8.0))
                        .flex()
                        .justify_end()
                        .child(
                            {
                                let state_close = app_state.clone();
                                div()
                                    .id(SharedString::from("cancel-edit-album-btn"))
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(4.0))
                                    .bg(theme.bg_pill)
                                    .border_1()
                                    .border_color(theme.border_card)
                                    .text_xs()
                                    .text_color(theme.text_secondary)
                                    .hover(move |s| s.bg(theme.bg_card_hover).text_color(theme.text_primary))
                                    .on_click(move |_, _, cx| {
                                        state_close.update(cx, |s, cx| {
                                            s.close_edit_album_modal();
                                            cx.notify();
                                        });
                                    })
                                    .child("Close")
                            },
                        ),
                ),
        )
        .into_any_element()
}

/// Renders the modal dialog for merging an album into another destination album.
pub fn render_merge_album_modal(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let source_album = state.merging_album.clone();
    let db_opt = state.active_db.clone();
    let drive_opt = state.selected_drive.clone();

    let source = match source_album {
        Some(a) => a,
        None => return div().into_any_element(),
    };

    let source_id = source.id;
    let source_name = source.name.clone();
    let source_rel = source.relative_path.clone();

    let albums = if let Some(ref db) = db_opt {
        db.get_albums()
            .unwrap_or_default()
            .into_iter()
            .filter(|a| a.id != source_id)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let state_cancel = app_state.clone();

    div()
        .id(SharedString::from("merge-album-modal-overlay"))
        .absolute()
        .inset_0()
        .bg(rgba(0x00000088))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .w(px(400.0))
                .p(px(16.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_active)
                .flex()
                .flex_col()
                .gap(px(12.0))
                // Title
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(format!("🔀 Merge '{}' Into...", source_name)),
                        )
                        .child(
                            div()
                                .id(SharedString::from("close-merge-modal-btn"))
                                .cursor_pointer()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .hover(move |s| s.text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_cancel.update(cx, |s, cx| {
                                        s.close_merge_modal();
                                        cx.notify();
                                    });
                                })
                                .child("✕"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_secondary)
                        .child("Select target album to receive all media from this album. Source album directory will be cleanly merged and cleaned up."),
                )
                // Destination Album List
                .child(
                    div()
                        .id(SharedString::from("merge-dest-list-scroll"))
                        .max_h(px(220.0))
                        .overflow_y_scroll()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .children(albums.into_iter().map(|target| {
                            let target_id = target.id;
                            let target_name = target.name.clone();
                            let target_rel = target.relative_path.clone();
                            let target_count = target.media_count;

                            let state_merge_action = app_state.clone();
                            let db_clone = db_opt.clone();
                            let drive_clone = drive_opt.clone();
                            let src_rel = source_rel.clone();

                            div()
                                .id(SharedString::from(format!("merge-dest-{}", target_id)))
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(8.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .hover(move |s| s.bg(theme.bg_pill_active).border_color(theme.accent))
                                .on_click(move |_, _, cx| {
                                    if let (Some(db), Some(drive)) = (&db_clone, &drive_clone) {
                                        let src_dir = drive.mount_path.join("albums").join(&src_rel);
                                        let dst_dir = drive.mount_path.join("albums").join(&target_rel);
                                        let _ = fs::create_dir_all(&dst_dir);

                                        // Move physical files from source to dest
                                        if src_dir.exists() && src_dir.is_dir() {
                                            if let Ok(entries) = fs::read_dir(&src_dir) {
                                                for entry in entries.flatten() {
                                                    let path = entry.path();
                                                    if path.is_file() {
                                                        let fname = path.file_name().unwrap_or_default();
                                                        let (dst_file, _) = resolve_collision(&dst_dir, fname.to_str().unwrap_or("file"));
                                                        let _ = fs::rename(&path, &dst_file).or_else(|_| {
                                                            fs::copy(&path, &dst_file)?;
                                                            fs::remove_file(&path)
                                                        });
                                                    }
                                                }
                                            }
                                            let _ = fs::remove_dir(&src_dir);
                                        }

                                        // Update SQLite
                                        let _ = db.merge_albums(source_id, target_id);
                                    }

                                    state_merge_action.update(cx, |s, cx| {
                                        s.close_merge_modal();
                                        s.select_album(Some(target_id));
                                        cx.notify();
                                    });
                                })
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
                                                .child(div().text_sm().child("📁"))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.text_primary)
                                                        .child(target_name),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(format!("{} items", target_count)),
                                        ),
                                )
                        })),
                )
                // Footer
                .child(
                    div()
                        .mt(px(8.0))
                        .flex()
                        .justify_end()
                        .child({
                            let state_close = app_state.clone();
                            div()
                                .id(SharedString::from("cancel-merge-btn"))
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .text_xs()
                                .text_color(theme.text_secondary)
                                .hover(move |s| s.bg(theme.bg_card_hover).text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_close.update(cx, |s, cx| {
                                        s.close_merge_modal();
                                        cx.notify();
                                    });
                                })
                                .child("Cancel")
                        }),
                ),
        )
        .into_any_element()
}

/// Renders the modal dialog for deleting an album, with explicit safe vs permanent options.
pub fn render_delete_album_modal(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let deleting_album = state.deleting_album.clone();
    let is_permanent = state.delete_album_permanent;
    let db_opt = state.active_db.clone();
    let drive_opt = state.selected_drive.clone();

    let album = match deleting_album {
        Some(a) => a,
        None => return div().into_any_element(),
    };

    let album_id = album.id;
    let album_name = album.name.clone();
    let album_rel = album.relative_path.clone();
    let media_count = album.media_count;

    let state_cancel = app_state.clone();
    let state_radio_safe = app_state.clone();
    let state_radio_perm = app_state.clone();
    let state_confirm = app_state.clone();

    div()
        .id(SharedString::from("delete-album-modal-overlay"))
        .absolute()
        .inset_0()
        .bg(rgba(0x00000088))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .w(px(420.0))
                .p(px(18.0))
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.accent_danger)
                .flex()
                .flex_col()
                .gap(px(14.0))
                // Title
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.accent_danger)
                                .child(format!("🗑 Delete Album: {}", album_name)),
                        )
                        .child(
                            div()
                                .id(SharedString::from("close-delete-modal-btn"))
                                .cursor_pointer()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .hover(move |s| s.text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_cancel.update(cx, |s, cx| {
                                        s.close_delete_modal();
                                        cx.notify();
                                    });
                                })
                                .child("✕"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_secondary)
                        .child(format!("This album contains {} media items. How would you like to handle them?", media_count)),
                )
                // Radio Options
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        // Option 1: Safe preservation
                        .child(
                            div()
                                .id(SharedString::from("radio-safe-delete"))
                                .cursor_pointer()
                                .p(px(10.0))
                                .rounded(px(5.0))
                                .bg(if !is_permanent { theme.bg_pill_active } else { theme.bg_pill })
                                .border_1()
                                .border_color(if !is_permanent { theme.accent } else { theme.border_card })
                                .on_click(move |_, _, cx| {
                                    state_radio_safe.update(cx, |s, cx| {
                                        s.set_delete_permanent(false);
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(2.0))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(6.0))
                                                .child(div().text_xs().child(if !is_permanent { "🔘" } else { "⚪" }))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.text_primary)
                                                        .child("Preserve media (Recommended)"),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .pl(px(18.0))
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("Files stay intact on drive and are reassigned to the 'unknown' album."),
                                        ),
                                ),
                        )
                        // Option 2: Permanent deletion
                        .child(
                            div()
                                .id(SharedString::from("radio-perm-delete"))
                                .cursor_pointer()
                                .p(px(10.0))
                                .rounded(px(5.0))
                                .bg(if is_permanent { theme.bg_pill_active } else { theme.bg_pill })
                                .border_1()
                                .border_color(if is_permanent { theme.accent_danger } else { theme.border_card })
                                .on_click(move |_, _, cx| {
                                    state_radio_perm.update(cx, |s, cx| {
                                        s.set_delete_permanent(true);
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(2.0))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(6.0))
                                                .child(div().text_xs().child(if is_permanent { "🔘" } else { "⚪" }))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.accent_danger)
                                                        .child("Delete files permanently"),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .pl(px(18.0))
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("All files in this album directory will be permanently removed from disk."),
                                        ),
                                ),
                        ),
                )
                // Footer
                .child(
                    div()
                        .mt(px(6.0))
                        .flex()
                        .justify_end()
                        .gap(px(8.0))
                        .child({
                            let state_close = app_state.clone();
                            div()
                                .id(SharedString::from("cancel-delete-modal-btn"))
                                .cursor_pointer()
                                .px(px(10.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .text_xs()
                                .text_color(theme.text_secondary)
                                .hover(move |s| s.bg(theme.bg_card_hover).text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_close.update(cx, |s, cx| {
                                        s.close_delete_modal();
                                        cx.notify();
                                    });
                                })
                                .child("Cancel")
                        })
                        .child({
                            let db = db_opt.clone();
                            let drive = drive_opt.clone();
                            let rel = album_rel.clone();

                            div()
                                .id(SharedString::from("confirm-delete-album-btn"))
                                .cursor_pointer()
                                .px(px(12.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .bg(theme.accent_danger)
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0xffffff))
                                .hover(move |s| s.bg(theme.accent_hover))
                                .on_click(move |_, _, cx| {
                                    if let Some(db) = &db {
                                        if is_permanent {
                                            if let Some(drive) = &drive {
                                                let album_dir = drive.mount_path.join("albums").join(&rel);
                                                let _ = fs::remove_dir_all(&album_dir);
                                            }
                                            let _ = db.delete_album(album_id, true);
                                        } else {
                                            let _ = db.delete_album(album_id, false);
                                        }
                                    }

                                    state_confirm.update(cx, |s, cx| {
                                        s.close_delete_modal();
                                        s.select_album(None);
                                        cx.notify();
                                    });
                                })
                                .child(if is_permanent { "Delete Permanently" } else { "Delete Album" })
                        }),
                ),
        )
        .into_any_element()
}
