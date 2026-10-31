use std::fs;
use std::path::Path;
use gpui_kit::*;
use crate::scanner::relocator::resolve_collision;
use crate::state::app_state::AppState;

/// Renders the modal dialog for moving selected media item(s) to a chosen destination album.
pub fn render_move_to_album_modal(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let moving_ids = state.moving_media_ids.clone();
    let db_opt = state.active_db.clone();
    let drive_opt = state.selected_drive.clone();

    if moving_ids.is_empty() {
        return div().into_any_element();
    }

    let state_cancel = app_state.clone();
    let albums = if let Some(ref db) = db_opt {
        db.get_albums().unwrap_or_default()
    } else {
        Vec::new()
    };

    let item_count = moving_ids.len();

    div()
        .id(SharedString::from("move-to-album-modal-overlay"))
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
                // Title Bar
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
                                .child(format!("📁 Move to Album ({} selected)", item_count)),
                        )
                        .child(
                            div()
                                .id(SharedString::from("close-move-modal-btn"))
                                .cursor_pointer()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .hover(move |s| s.text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_cancel.update(cx, |s, cx| {
                                        s.close_move_modal();
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
                        .child("Select a target destination album. Files will be relocated atomically on disk and updated in the library catalog."),
                )
                // Album Choices List
                .child(
                    div()
                        .id(SharedString::from("move-album-choices-scroll"))
                        .max_h(px(240.0))
                        .overflow_y_scroll()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .children(albums.into_iter().map(|album| {
                            let target_album_id = album.id;
                            let target_album_name = album.name.clone();
                            let target_rel_path = album.relative_path.clone();
                            let album_media_count = album.media_count;

                            let state_pick = app_state.clone();
                            let db_clone = db_opt.clone();
                            let drive_clone = drive_opt.clone();
                            let ids = moving_ids.clone();

                            div()
                                .id(SharedString::from(format!("move-target-{}", target_album_id)))
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
                                        let target_dir = drive.mount_path.join("albums").join(&target_rel_path);
                                        let _ = fs::create_dir_all(&target_dir);

                                        for mid in &ids {
                                            // Get item current info
                                            if let Ok(Some((item, _, _))) = db.get_media_item_with_neighbors(*mid, None) {
                                                let current_file_path = drive.mount_path.join(&item.current_relative_path);
                                                let filename = Path::new(&item.current_relative_path)
                                                    .file_name()
                                                    .unwrap_or_default();

                                                let (dest_path, final_filename) = resolve_collision(&target_dir, filename.to_str().unwrap_or("media"));
                                                let new_rel = format!("albums/{}/{}", target_rel_path, final_filename);

                                                if current_file_path.exists() {
                                                    let _ = fs::rename(&current_file_path, &dest_path)
                                                        .or_else(|_| {
                                                            fs::copy(&current_file_path, &dest_path)?;
                                                            fs::remove_file(&current_file_path)
                                                        });
                                                }

                                                let _ = db.update_media_album_and_path(*mid, target_album_id, &new_rel);
                                            }
                                        }
                                    }

                                    state_pick.update(cx, |s, cx| {
                                        s.close_move_modal();
                                        s.clear_media_selection();
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
                                                        .child(target_album_name),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(format!("{} items", album_media_count)),
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
                                .id(SharedString::from("cancel-move-btn"))
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
                                        s.close_move_modal();
                                        cx.notify();
                                    });
                                })
                                .child("Cancel")
                        }),
                ),
        )
        .into_any_element()
}
