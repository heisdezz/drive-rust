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
        "Local File System"
    } else if is_stream {
        "HTTP Stream (Range 206)"
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
                .overflow_hidden()
                .flex()
                .flex_col()
                // Header (Zed 38px height)
                .child(
                    div()
                        .h(px(38.0))
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
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child(Icon::VIDEO),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("Video URL / Stream Tester"),
                                )
                                .child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(1.0))
                                        .rounded(px(3.0))
                                        .bg(theme.bg_pill)
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("Test Suite"),
                                ),
                        )
                        .child(
                            div()
                                .id(SharedString::from("close-video-test-modal-btn"))
                                .cursor_pointer()
                                .w(px(22.0))
                                .h(px(22.0))
                                .rounded(px(4.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_xs()
                                .text_color(theme.text_secondary)
                                .hover(move |s| s.bg(theme.bg_pill_active).text_color(theme.text_primary))
                                .on_click(move |_, _, cx| {
                                    state_close.update(cx, |s, cx| {
                                        s.close_video_test_modal();
                                        cx.notify();
                                    });
                                })
                                .child(Icon::CLOSE),
                        ),
                )
                // Body
                .child(
                    div()
                        .p(px(16.0))
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        // URL Input Box & Presets
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_secondary)
                                        .child("Target Video URL / File Path:"),
                                )
                                .child(
                                    div()
                                        .p(px(8.0))
                                        .rounded(px(5.0))
                                        .bg(theme.bg_sidebar)
                                        .border_1()
                                        .border_color(theme.border_card)
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::NORMAL)
                                                .text_color(if is_empty { theme.text_muted } else { theme.text_primary })
                                                .child(if is_empty {
                                                    "Enter or select a video URL (e.g. http://127.0.0.1:8080/stream/video.mp4)...".to_string()
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
                                                .text_color(theme.text_muted)
                                                .child("Presets:"),
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
                                                        s.set_test_video_url("/run/media/destiny/Ventoy/sample_media/4k_demo.mp4".to_string());
                                                        cx.notify();
                                                    });
                                                })
                                                .child("Local Sample MP4"),
                                        )
                                        .child(
                                            div()
                                                .id(SharedString::from("preset-http-btn"))
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
                                                        s.set_test_video_url("http://127.0.0.1:8080/stream/4k_landscape.mp4".to_string());
                                                        cx.notify();
                                                    });
                                                })
                                                .child("Localhost 127.0.0.1 Stream"),
                                        )
                                        .child(
                                            div()
                                                .id(SharedString::from("preset-hls-btn"))
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
                                                .flex()
                                                .items_center()
                                                .gap(px(4.0))
                                                .child(div().text_xs().text_color(theme.text_muted).child("Protocol:"))
                                                .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(protocol_label)),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(4.0))
                                                .child(div().text_xs().text_color(theme.text_muted).child("Codec:"))
                                                .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(theme.text_primary).child("H.264 / AAC")),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(4.0))
                                                .child(div().text_xs().text_color(theme.text_muted).child("Resolution:"))
                                                .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(theme.text_primary).child("3840 × 2160 (4K)")),
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
                                                .child(if is_empty { "No Stream" } else { "Stream Connected" }),
                                        ),
                                ),
                        )
                        // Mock Video Canvas with monochrome controls
                        .child(
                            div()
                                .w_full()
                                .h(px(240.0))
                                .rounded(px(6.0))
                                .bg(rgba(0x050a12ee))
                                .border_1()
                                .border_color(theme.border_card)
                                .flex()
                                .flex_col()
                                .justify_between()
                                .p(px(12.0))
                                // Center Canvas graphics
                                .child(
                                    div()
                                        .flex_grow(1.0)
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .justify_center()
                                        .gap(px(8.0))
                                        .child(
                                            div()
                                                .w(px(48.0))
                                                .h(px(48.0))
                                                .rounded(px(24.0))
                                                .bg(theme.accent)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .text_lg()
                                                .text_color(rgb(0xffffff))
                                                .child(Icon::PLAY),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(theme.text_secondary)
                                                .child(if is_empty { "Awaiting videoUrl input..." } else { "Playback Ready (Click to Play)" }),
                                        ),
                                )
                                // Video timeline & controls
                                .child(
                                    div()
                                        .h(px(32.0))
                                        .px(px(8.0))
                                        .rounded(px(4.0))
                                        .bg(rgba(0x00000088))
                                        .border_1()
                                        .border_color(rgba(0xffffff15))
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(8.0))
                                                .child(div().text_xs().text_color(rgb(0xffffff)).child(Icon::PLAY))
                                                .child(div().text_xs().text_color(theme.text_muted).child("00:00 / 03:15")),
                                        )
                                        .child(
                                            div()
                                                .w(px(320.0))
                                                .h(px(4.0))
                                                .rounded(px(2.0))
                                                .bg(theme.bg_pill)
                                                .child(
                                                    div()
                                                        .w(px(100.0))
                                                        .h(px(4.0))
                                                        .rounded(px(2.0))
                                                        .bg(theme.accent),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(8.0))
                                                .child(div().text_xs().text_color(theme.text_muted).child("1080p"))
                                                .child(div().text_xs().text_color(theme.text_muted).child("100%")),
                                        ),
                                ),
                        ),
                ),
        )
}
