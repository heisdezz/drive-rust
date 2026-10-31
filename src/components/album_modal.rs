use std::fs;
use gpui_kit::*;
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

/// Renders the modal overlay for editing, renaming, or deleting an existing album.
pub fn render_edit_album_modal(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let editing_album = state.editing_album.clone();
    let db_opt = state.active_db.clone();

    let state_cancel = app_state.clone();

    let album = match editing_album {
        Some(a) => a,
        None => return div().into_any_element(),
    };

    let album_id = album.id;
    let album_name = album.name.clone();
    let media_count = album.media_count;

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
                        // Delete Button (Safely moves files to Unsorted)
                        .child(
                            {
                                let state_delete = app_state.clone();
                                let db = db_opt.clone();
                                div()
                                    .id(SharedString::from("delete-album-btn"))
                                    .cursor_pointer()
                                    .p(px(8.0))
                                    .rounded(px(4.0))
                                    .bg(theme.bg_pill)
                                    .border_1()
                                    .border_color(theme.accent_danger)
                                    .hover(move |s| s.bg(theme.accent_danger).text_color(rgb(0xffffff)))
                                    .on_click(move |_, _, cx| {
                                        if let Some(db) = &db {
                                            let _ = db.delete_album(album_id, false);
                                        }
                                        state_delete.update(cx, |s, cx| {
                                            s.close_edit_album_modal();
                                            s.select_album(None);
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
                                                    .child("Delete Album"),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_muted)
                                                    .child("Moves files to Unknown"),
                                            ),
                                    )
                            },
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
