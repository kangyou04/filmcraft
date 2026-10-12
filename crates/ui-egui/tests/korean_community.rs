use filmcraft_ui_egui::{FilmcraftApp, i18n, menus};
use serde_json::json;

#[test]
fn korean_effect_aliases_keep_original_searches_and_do_not_match_unrelated_effects() {
    i18n::set_current(i18n::Language::Ko);
    assert!(i18n::matches_query("Gaussian Blur", "블러"));
    assert!(i18n::matches_query("Gaussian Blur", "흐림"));
    assert!(i18n::matches_query("Gaussian Blur", "gaussian"));
    assert!(i18n::matches_query("Stroke", "외곽선"));
    assert!(!i18n::matches_query("Brush Strokes", "외곽선"));
    assert!(!i18n::matches_query("Gaussian Blur", "외곽선"));
    i18n::set_current(i18n::Language::En);
    assert!(!i18n::matches_query("Gaussian Blur", "블러"));
}

#[test]
fn real_disabled_command_shows_korean_guidance_but_returns_original_error() {
    let mut app = FilmcraftApp::new(filmcraft_engine::Session::default());
    let ctx = egui::Context::default();
    i18n::set_current(i18n::Language::Ko);
    let error = menus::invoke(&mut app, &ctx, "edit.clear", json!({})).unwrap_err();
    assert!(error.contains("no sequence is open"), "{error}");
    assert!(app.ui.status.contains("시퀀스를 만들거나"), "{}", app.ui.status);
    assert!(app.ui.status.contains(&error));
    i18n::set_current(i18n::Language::En);
    let error = menus::invoke(&mut app, &ctx, "edit.clear", json!({})).unwrap_err();
    assert_eq!(app.ui.status, error);
}

#[test]
fn unsupported_media_retains_diagnostic_and_path_words_do_not_change_error_category() {
    i18n::set_current(i18n::Language::Ko);
    let error = filmcraft_media::MediaError::Unsupported("test format".into()).to_string();
    let message = i18n::error_message(&format!("media: {error}"));
    assert!(message.contains("지원하지 않는 형식"));
    assert!(message.contains(&error));
    let message = i18n::error_message("I/O error: C:\\unsupported media\\file.mp4");
    assert!(message.contains("파일을 읽거나 쓰지 못했습니다"));
    assert!(!message.contains("지원하지 않는 형식"));
}
