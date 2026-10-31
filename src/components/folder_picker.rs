use std::path::PathBuf;
use std::process::Command;
use gpui_kit::*;
use crate::state::app_state::AppState;

/// Prompts user to select a folder via XDG Desktop Portal, falling back to zenity/kdialog on Linux.
/// Sets the chosen directory as custom library and active mounted volume.
pub fn prompt_mount_custom_folder(
    app_state: &Entity<AppState>,
    cx: &mut App,
) {
    let options = PathPromptOptions {
        files: false,
        directories: true,
        multiple: false,
        prompt: Some(SharedString::from("Select Media Library Directory / Storage Volume")),
    };
    let receiver = cx.prompt_for_paths(options);
    let state_clone = app_state.clone();

    cx.spawn(|cx: &mut AsyncApp| {
        let cx = cx.clone();
        async move {
            // Try native portal picker first
            if let Ok(Ok(Some(paths))) = receiver.await {
                if let Some(first) = paths.into_iter().next() {
                    cx.update(|cx: &mut App| {
                        state_clone.update(cx, |s, cx| {
                            s.set_custom_library_path(Some(first));
                            cx.notify();
                        });
                    });
                    return;
                }
            }

            // Fallback to zenity or kdialog
            let fallback_path = cx
                .background_executor()
                .spawn(async move {
                    if let Ok(output) = Command::new("zenity")
                        .args(["--file-selection", "--directory", "--title=Select Media Library Directory / Storage Volume"])
                        .output()
                    {
                        if output.status.success() {
                            let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
                            if !s.is_empty() {
                                return Some(PathBuf::from(s));
                            }
                        }
                    }

                    if let Ok(output) = Command::new("kdialog")
                        .args(["--getexistingdirectory", "."])
                        .output()
                    {
                        if output.status.success() {
                            let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
                            if !s.is_empty() {
                                return Some(PathBuf::from(s));
                            }
                        }
                    }
                    None
                })
                .await;

            if let Some(path) = fallback_path {
                cx.update(|cx: &mut App| {
                    state_clone.update(cx, |s, cx| {
                        s.set_custom_library_path(Some(path));
                        cx.notify();
                    });
                });
            }
        }
    })
    .detach();
}
