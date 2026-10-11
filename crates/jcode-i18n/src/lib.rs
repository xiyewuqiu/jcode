pub mod commands;
pub mod ui;

use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Language {
    #[default]
    ZhCn = 0,
    EnUs = 1,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ZhCn => "zh-CN",
            Self::EnUs => "en-US",
        }
    }

    pub fn from_str_loose(input: &str) -> Option<Self> {
        let s = input.trim().to_ascii_lowercase();
        if s.starts_with("zh") || s.contains("cn") || s == "chinese" {
            Some(Self::ZhCn)
        } else if s.starts_with("en") || s == "english" {
            Some(Self::EnUs)
        } else {
            None
        }
    }
}

// Default to Chinese (0) for this localized fork.
// Can be changed dynamically via `set_language`.
static CURRENT_LANGUAGE: AtomicU8 = AtomicU8::new(0);

/// Set current UI language
pub fn set_language(lang: Language) {
    CURRENT_LANGUAGE.store(lang as u8, Ordering::Relaxed);
}

/// Get current UI language
pub fn current_language() -> Language {
    match CURRENT_LANGUAGE.load(Ordering::Relaxed) {
        0 => Language::ZhCn,
        _ => Language::EnUs,
    }
}

/// Helper to check if currently rendering in Chinese
pub fn is_chinese() -> bool {
    current_language() == Language::ZhCn
}

/// Translate slash command help text.
/// If current language is Chinese, returns the translated Chinese string.
/// Otherwise returns the original `fallback_en`.
pub fn t_command(name: &str, fallback_en: &'static str) -> &'static str {
    if is_chinese() {
        commands::translate_command_help(name, fallback_en)
    } else {
        fallback_en
    }
}

/// Translate common UI text, prompt, or menu description.
pub fn t_ui<'a>(text: &'a str) -> &'a str {
    if is_chinese() {
        ui::translate_ui_text(text)
    } else {
        text
    }
}

/// Translate section header (e.g. "  Commands", "  Session")
pub fn t_section<'a>(header: &'a str) -> &'a str {
    if is_chinese() {
        ui::translate_section_header(header)
    } else {
        header
    }
}

/// Translate detailed multi-line help documentation for `/help <topic>`.
pub fn t_detailed_help(topic: &str) -> Option<&'static str> {
    if is_chinese() {
        commands::translate_detailed_help(topic)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_translation_and_fallback() {
        set_language(Language::ZhCn);
        assert!(is_chinese());
        assert_eq!(
            t_command("/help", "Show help"),
            "显示帮助与快捷键列表"
        );
        assert_eq!(
            t_command("/unknown_future_cmd", "Future fallback"),
            "Future fallback"
        );
        assert!(t_detailed_help("fix").is_some());

        // Switch to English
        set_language(Language::EnUs);
        assert!(!is_chinese());
        assert_eq!(t_command("/help", "Show help"), "Show help");
        assert_eq!(t_ui("Scroll history"), "Scroll history");
        assert!(t_detailed_help("fix").is_none());

        // Switch back to Chinese
        set_language(Language::ZhCn);
        assert_eq!(t_ui("Scroll history"), "滚动浏览历史记录");
    }
}
