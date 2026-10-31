use gpui_kit::*;
use crate::state::app_state::AppState;
use crate::theme::{AppTheme, ThemeMode};

/// Renders a visual theme selection grid for the Settings view.
pub fn render_theme_picker(
    app_state: &Entity<AppState>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let current_mode = state.theme_mode;
    let theme = state.theme;

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
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_primary)
                        .child("Interface Theme"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(format!("Active: {}", current_mode.name())),
                ),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(8.0))
                .children(ThemeMode::all().iter().map(|&mode| {
                    let is_active = mode == current_mode;
                    let preview = AppTheme::from_mode(mode);
                    let state_handle = app_state.clone();

                    div()
                        .id(SharedString::from(format!("theme-card-{}", mode.name())))
                        .cursor_pointer()
                        .w(px(130.0))
                        .p(px(8.0))
                        .rounded(px(5.0))
                        .border_1()
                        .border_color(if is_active { theme.accent } else { theme.border_card })
                        .bg(if is_active { theme.bg_pill_active } else { theme.bg_card })
                        .hover(|s| s.border_color(theme.border_active))
                        .on_click(move |_, _, cx| {
                            state_handle.update(cx, |s, cx| {
                                s.set_theme_mode(mode);
                                cx.notify();
                            });
                        })
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(6.0))
                                // Swatch preview bar
                                .child(
                                    div()
                                        .h(px(20.0))
                                        .rounded(px(3.0))
                                        .border_1()
                                        .border_color(preview.border_subtle)
                                        .flex()
                                        .overflow_hidden()
                                        .child(div().w_full().bg(preview.bg_canvas))
                                        .child(div().w_full().bg(preview.bg_sidebar))
                                        .child(div().w_full().bg(preview.accent)),
                                )
                                // Theme Name & Icon
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(4.0))
                                                .child(div().text_xs().child(mode.icon()))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                                                        .text_color(if is_active { theme.text_primary } else { theme.text_secondary })
                                                        .child(mode.name()),
                                                ),
                                        )
                                        .child(
                                            if is_active {
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(theme.accent)
                                                    .child("✓")
                                            } else {
                                                div()
                                            },
                                        ),
                                ),
                        )
                })),
        )
}
