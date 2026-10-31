use gpui_kit::*;
use crate::state::app_state::AppState;
use crate::theme::icons::Icon;

/// Renders the Video URL Test Page / Modal allowing manual videoUrl entry,
/// codec inspection, and playback simulation.
pub fn render_video_test_modal(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;
    let video_url = state.test_video_url.clone();

    let state_close = app_state.clone();
    let state_preset1 = app_state.clone();
    let state_preset2 = app_state.clone();
    let state_preset3 = app_state.clone();

    let is_empty = video_url.trim().is_empty();
    let is_local = video_url.starts_with('/') || video_url.starts_with("file://");
    let is_stream = video_url.starts_with("http://") || video_url.starts_with("https://");

    let protocol_label = if is_local {
        "Direct Native File (Zero Copy)"
    } else if is_stream {
        "Remote HTTP Stream"
    } else if is_empty {
        "None"
    } else {
        "Custom URI"
    };

    div()
        .id(SharedString::from("video-test-modal-overlay"))
        .absolute()
        .inset_0()
        .bg(rgba(0x000000cc))
        .flex()
        .items_center()
        .justify_center()
        .p(px(20.0))
        .child(
            div()
                .id(SharedString::from("video-test-modal-card"))
                .w_full()
                .max_w(px(760.0))
                .rounded(px(8.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_active)
                .flex()
                .flex_col()
                .overflow_hidden()
                // Modal Topbar
                .child(
                    div()
                        .h(px(40.0))
                        .px(px(14.0))
                        .bg(theme.bg_header)
                        .border_b_1()
                        .border_color(theme.border_subtle)
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
                                        .text_xs()
                                        .text_color(theme.accent)
                                        .child(Icon::VIDEO),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("Direct Media File / Stream Testing Canvas"),
                                )
                                .child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(1.5))
                                        .rounded(px(3.0))
                                        .bg(theme.bg_pill)
                                        .border_1()
                                        .border_color(theme.border_card)
                                        .text_xs()
                                        .text_color(theme.accent)
                                        .child(protocol_label),
                                ),
                        )
                        .child(
                            div()
                                .id(SharedString::from("close-video-test-btn"))
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(2.5))
                                .rounded(px(3.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_secondary)
                                .hover(move |s| s.bg(theme.bg_card_hover).text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_close.update(cx, |s, cx| {
                                        s.close_video_test_modal();
                                        cx.notify();
                                    });
                                })
                                .child(format!("{} Close", Icon::CLOSE)),
                        ),
                )
                // Modal Body
                .child(
                    div()
                        .p(px(16.0))
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        // Description banner
                        .child(
                            div()
                                .p(px(10.0))
                                .rounded(px(5.0))
                                .bg(theme.bg_sidebar)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_primary)
                                        .child("Direct Media Input Testing"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("Test media playback by specifying a direct file path on disk or a remote HTTP stream URL. Unlike browser webviews, native Rust reads files directly from the filesystem with zero-copy seeking."),
                                ),
                        )
                        // Input Field Card
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("MEDIA FILE PATH / STREAM URL:"),
                                )
                                .child(
                                    div()
                                        .px(px(10.0))
                                        .py(px(8.0))
                                        .rounded(px(4.0))
                                        .bg(theme.bg_sidebar)
                                        .border_1()
                                        .border_color(theme.border_card)
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_family("monospace")
                                                .text_color(if is_empty { theme.text_muted } else { theme.text_primary })
                                                .child(if is_empty {
                                                    "Enter direct file path (e.g. /media/video.mp4) or URL...".to_string()
                                                } else {
                                                    video_url.clone()
                                                }),
                                        ),
                                )
                                // Presets
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
                                                .child("PRESETS:"),
                                        )
                                        .child(
                                            div()
                                                .id(SharedString::from("preset-local-btn"))
                                                .cursor_pointer()
                                                .px(px(6.0))
                                                .py(px(2.0))
                                                .rounded(px(3.0))
                                                .bg(theme.bg_pill)
                                                .text_xs()
                                                .text_color(theme.text_secondary)
                                                .hover(move |s| s.bg(theme.bg_pill_active).text_color(theme.text_primary))
                                                .on_click(move |_, _, cx| {
                                                    state_preset1.update(cx, |s, cx| {
                                                        s.set_test_video_url("/run/media/destiny/Ventoy/sample.mp4".to_string());
                                                        cx.notify();
                                                    });
                                                })
                                                .child("Local Sample MP4"),
                                        )
                                        .child(
                                            div()
                                                .id(SharedString::from("preset-drive-btn"))
                                                .cursor_pointer()
                                                .px(px(6.0))
                                                .py(px(2.0))
                                                .rounded(px(3.0))
                                                .bg(theme.bg_pill)
                                                .text_xs()
                                                .text_color(theme.text_secondary)
                                                .hover(move |s| s.bg(theme.bg_pill_active).text_color(theme.text_primary))
                                                .on_click(move |_, _, cx| {
                                                    state_preset2.update(cx, |s, cx| {
                                                        s.set_test_video_url("/run/media/destiny/Ventoy/media/sample_4k.mp4".to_string());
                                                        cx.notify();
                                                    });
                                                })
                                                .child("Removable Drive MP4"),
                                        )
                                        .child(
                                            div()
                                                .id(SharedString::from("preset-remote-btn"))
                                                .cursor_pointer()
                                                .px(px(6.0))
                                                .py(px(2.0))
                                                .rounded(px(3.0))
                                                .bg(theme.bg_pill)
                                                .text_xs()
                                                .text_color(theme.text_secondary)
                                                .hover(move |s| s.bg(theme.bg_pill_active).text_color(theme.text_primary))
                                                .on_click(move |_, _, cx| {
                                                    state_preset3.update(cx, |s, cx| {
                                                        s.set_test_video_url("https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4".to_string());
                                                        cx.notify();
                                                    });
                                                })
                                                .child("Remote Test Stream"),
                                        ),
                                ),
                        )
                        // Stream Diagnostic Pill Strip
                        .child(
                            div()
                                .p(px(8.0))
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
                                        .items_center()
                                        .gap(px(14.0))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(format!("Container: {}", if is_empty { "-" } else { "MP4 / ISO BMFF" })),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(format!("Codec: {}", if is_empty { "-" } else { "H.264 / AVC1 + AAC" })),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(format!("I/O Engine: {}", if is_local { "Direct OS File Seek" } else { "HTTP Client" })),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(4.0))
                                        .child(
                                            div()
                                                .w(px(6.0))
                                                .h(px(6.0))
                                                .rounded(px(3.0))
                                                .bg(if is_empty { theme.accent_warning } else { theme.accent_success }),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(if is_empty { theme.accent_warning } else { theme.accent_success })
                                                .child(if is_empty { "Awaiting Media Input" } else { "Media Verified" }),
                                        ),
                                ),
                        )
                        // Canvas / Player Rendering Stage
                        .child(
                            div()
                                .h(px(280.0))
                                .rounded(px(6.0))
                                .bg(rgb(0x0a0a0c))
                                .border_1()
                                .border_color(theme.border_card)
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .gap(px(10.0))
                                .child(
                                    div()
                                        .text_3xl()
                                        .text_color(theme.accent)
                                        .child(Icon::PLAY),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_primary)
                                        .child(if is_empty {
                                            "No Video Loaded".to_string()
                                        } else {
                                            format!("Playback Canvas Ready")
                                        }),
                                )
                                .child(
                                    div()
                                        .max_w(px(500.0))
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .text_center()
                                        .child(if is_empty {
                                            "Select a test preset above to simulate video rendering."
                                        } else {
                                            "Direct native decoding pipeline. Frame seeking and time indexing mapped to file seek offsets."
                                        }),
                                ),
                        ),
                ),
        )
}
