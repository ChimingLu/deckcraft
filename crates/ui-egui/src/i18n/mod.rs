//! Interface languages. English strings in the code are the keys; each language has a table of
//! translations (`zh_tw.rs`). Missing entries fall back to English, so an untranslated string is
//! never blank.

use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, Ordering};

use deckcraft_engine::Language;

mod zh_tw;

/// The language the interface draws in; `SlideApp::logic` copies it from the engine preferences
/// every frame so widgets without access to the app can translate too.
static CURRENT: AtomicU8 = AtomicU8::new(1);

pub fn set_language(lang: Language) {
    CURRENT.store(
        match lang {
            Language::english => 0,
            Language::traditional_chinese => 1,
        },
        Ordering::Relaxed,
    );
}

pub fn language() -> Language {
    match CURRENT.load(Ordering::Relaxed) {
        0 => Language::english,
        _ => Language::traditional_chinese,
    }
}

/// `en` in the current interface language.
pub fn t(en: &str) -> &str {
    translate(language(), en)
}

/// `en` in `lang`. Two-line ribbon labels ("New\nSlide") match their one-line key.
pub fn translate(lang: Language, en: &str) -> &str {
    if lang == Language::english || en.is_empty() {
        return en;
    }
    let table = zh_tw_table();
    if let Some(zh) = table.get(en) {
        return zh;
    }
    if en.contains('\n') || en.contains("  ") {
        let key = en.split_whitespace().collect::<Vec<_>>().join(" ");
        if let Some(zh) = table.get(key.as_str()) {
            return zh;
        }
    }
    en
}

fn zh_tw_table() -> &'static HashMap<&'static str, &'static str> {
    static TABLE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    TABLE.get_or_init(|| zh_tw::STRINGS.iter().copied().collect())
}

pub fn text(lang: Language, en: &'static str, zh: &'static str) -> &'static str {
    match lang {
        Language::traditional_chinese => zh,
        Language::english => en,
    }
}

pub fn tr(app: &crate::SlideApp, en: &'static str, zh: &'static str) -> &'static str {
    text(app.session.prefs.language, en, zh)
}

pub fn label(app: &crate::SlideApp, en: &'static str, zh: &'static str) -> String {
    text(app.session.prefs.language, en, zh).to_string()
}

pub fn command_label(app: &crate::SlideApp, label: &str) -> String {
    translate(app.session.prefs.language, label).to_string()
}

pub fn tab_label(app: &crate::SlideApp, label: &str) -> String {
    translate(app.session.prefs.language, label).to_string()
}

pub fn language_name(lang: Language) -> &'static str {
    match lang {
        Language::english => "English (United States)",
        Language::traditional_chinese => "繁體中文（台灣）",
    }
}

pub fn dialog_title(lang: Language, id: &str) -> &'static str {
    match id {
        "table" => text(lang, "Insert Table", "插入表格"),
        "slideSize" => text(lang, "Slide Size", "投影片尺寸"),
        "headerFooter" => text(lang, "Header and Footer", "頁首與頁尾"),
        "hyperlink" => text(lang, "Insert Hyperlink", "插入超連結"),
        "setupShow" => text(lang, "Set Up Slide Show", "設定放映"),
        "zoom" => text(lang, "Zoom", "縮放"),
        "comment" => text(lang, "New Comment", "新增留言"),
        "altText" => text(lang, "Alt Text", "替代文字"),
        "outline" => text(lang, "Slides from Outline", "從大綱建立投影片"),
        "renameSection" => text(lang, "Rename Section", "重新命名章節"),
        "renameShape" => text(lang, "Rename", "重新命名"),
        "renameLayout" => text(lang, "Rename Layout", "重新命名版面"),
        "paragraph" => text(lang, "Paragraph", "段落"),
        "about" => text(lang, "About DeckCraft", "關於 DeckCraft"),
        "preferences" => text(lang, "Preferences", "偏好設定"),
        "export" => text(lang, "Export", "匯出"),
        "symbol" => text(lang, "Symbol", "符號"),
        "equation" => text(lang, "Equation", "公式"),
        "spelling" => text(lang, "Spelling", "拼字檢查"),
        "accessibility" => text(lang, "Accessibility Checker", "無障礙檢查器"),
        "language" => text(lang, "Language", "語言"),
        "customShow" => text(lang, "Custom Shows", "自訂放映"),
        "chartData" => text(lang, "Chart Data", "圖表資料"),
        "trim" => text(lang, "Trim Media", "裁剪媒體"),
        "smartart" => text(lang, "Choose a SmartArt Graphic", "選擇 SmartArt 圖示"),
        "quit" => text(lang, "DeckCraft", "DeckCraft"),
        _ => "DeckCraft",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_picker_uses_traditional_chinese_by_default() {
        assert_eq!(text(Language::traditional_chinese, "A", "繁中"), "繁中");
        assert_eq!(text(Language::english, "A", "繁中"), "A");
    }

    #[test]
    fn translate_falls_back_to_english_and_joins_two_line_labels() {
        assert_eq!(translate(Language::traditional_chinese, "New Slide"), "新增投影片");
        assert_eq!(translate(Language::traditional_chinese, "New\nSlide"), "新增投影片");
        assert_eq!(translate(Language::english, "New Slide"), "New Slide");
        assert_eq!(translate(Language::traditional_chinese, "No such string here"), "No such string here");
    }

    #[test]
    fn zh_tw_table_has_one_entry_per_key() {
        let mut seen = std::collections::HashSet::new();
        for (en, zh) in zh_tw::STRINGS {
            assert!(seen.insert(*en), "duplicate key {en:?}");
            assert!(!zh.trim().is_empty(), "empty translation for {en:?}");
            assert!(!en.contains('\n'), "keys are one line: {en:?}");
        }
    }
}
