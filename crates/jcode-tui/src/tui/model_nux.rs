//! Model availability NUX (New User eXperience) tracking (Codex-compatible).
//!
//! In Codex, when new models (e.g. "gpt-6.1-sol", "cline-pass/deepseek-v4.1-flash") are
//! introduced, a startup tooltip / NUX hint is shown up to `MODEL_AVAILABILITY_NUX_MAX_SHOW_COUNT = 4`
//! times. Once the count reaches 4, further promotional tooltips are permanently muted.
//!
//! Config representation:
//! ```toml
//! [tui.model_availability_nux]
//! "gpt-6.1-sol" = 4
//! ```

use std::collections::BTreeMap;

/// Maximum number of times a model availability tooltip/banner is shown before being permanently silenced.
pub const MODEL_AVAILABILITY_NUX_MAX_SHOW_COUNT: u32 = 4;

/// Check whether the NUX announcement for a given model is eligible to be shown.
/// Returns false if shown_count >= 4 or if the model is explicitly muted.
pub fn is_model_nux_eligible(model: &str, nux_counts: &BTreeMap<String, u32>) -> bool {
    let count = nux_counts.get(model).copied().unwrap_or(0);
    count < MODEL_AVAILABILITY_NUX_MAX_SHOW_COUNT
}

/// Increment the display count for a model NUX announcement.
pub fn record_model_nux_shown(model: &str, nux_counts: &mut BTreeMap<String, u32>) -> u32 {
    let count = nux_counts.entry(model.to_string()).or_insert(0);
    *count = count.saturating_add(1);
    *count
}

/// Get default curated announcement text for recognized new models if eligible.
pub fn model_nux_announcement(model: &str, nux_counts: &BTreeMap<String, u32>) -> Option<String> {
    if !is_model_nux_eligible(model, nux_counts) {
        return None;
    }

    let count = nux_counts.get(model).copied().unwrap_or(0);
    let remaining = MODEL_AVAILABILITY_NUX_MAX_SHOW_COUNT.saturating_sub(count);

    if model.contains("deepseek-v4") || model.contains("cline-pass") {
        Some(format!("新旗舰模型就绪 (提示剩余 {} 次)", remaining))
    } else if model.starts_with("gpt-6") || model.contains("sol") {
        Some(format!("Codex 新模型可用 (提示剩余 {} 次)", remaining))
    } else {
        Some(format!("新模型推荐 (提示剩余 {} 次)", remaining))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_nux_muted_at_limit() {
        let mut counts = BTreeMap::new();
        counts.insert("gpt-6.1-sol".to_string(), 4);

        assert!(!is_model_nux_eligible("gpt-6.1-sol", &counts));
        assert_eq!(model_nux_announcement("gpt-6.1-sol", &counts), None);
    }

    #[test]
    fn test_model_nux_shown_below_limit() {
        let mut counts = BTreeMap::new();
        counts.insert("gpt-6.1-sol".to_string(), 2);

        assert!(is_model_nux_eligible("gpt-6.1-sol", &counts));
        let msg = model_nux_announcement("gpt-6.1-sol", &counts).unwrap();
        assert!(msg.contains("剩余 2 次"));
    }

    #[test]
    fn test_model_nux_deepseek_flagship() {
        let counts = BTreeMap::new();
        assert!(is_model_nux_eligible("cline-pass/deepseek-v4.1-flash", &counts));
        let msg = model_nux_announcement("cline-pass/deepseek-v4.1-flash", &counts).unwrap();
        assert!(msg.contains("新旗舰模型就绪"));
        assert!(msg.contains("剩余 4 次"));
    }
}

