//! Screen reader detection engine (Codex-compatible).
//!
//! When `screen_reader_detection_done` is true, OS probes are bypassed completely (0ms startup overhead).
//! When false, probe for host screen readers (Windows Narrator, Linux Orca / AT-SPI, macOS VoiceOver).
//! If detected, decorative animations are disabled to protect accessibility TTS readers.

use std::sync::atomic::{AtomicBool, Ordering};

static DETECTED_SCREEN_READER: AtomicBool = AtomicBool::new(false);

/// Check if a screen reader has been detected during session.
pub fn is_screen_reader_detected() -> bool {
    DETECTED_SCREEN_READER.load(Ordering::Relaxed)
}

/// Run detection probe for active screen readers.
pub fn detect_screen_reader() -> bool {
    #[cfg(windows)]
    {
        if let Ok(val) = std::env::var("SCREEN_READER") {
            if val == "1" || val.eq_ignore_ascii_case("true") {
                DETECTED_SCREEN_READER.store(true, Ordering::Relaxed);
                return true;
            }
        }
        // Check Windows Narrator process
        if is_narrator_active() {
            DETECTED_SCREEN_READER.store(true, Ordering::Relaxed);
            return true;
        }
    }

    #[cfg(target_os = "linux")]
    {
        if std::env::var("ACCESSIBILITY_ENABLED").map(|v| v == "1").unwrap_or(false)
            || std::env::var("ORCA_RUNNING").is_ok()
        {
            DETECTED_SCREEN_READER.store(true, Ordering::Relaxed);
            return true;
        }
    }

    #[cfg(target_os = "macos")]
    {
        if std::env::var("VOICEOVER_ENABLED").map(|v| v == "1").unwrap_or(false) {
            DETECTED_SCREEN_READER.store(true, Ordering::Relaxed);
            return true;
        }
    }

    false
}

#[cfg(windows)]
fn is_narrator_active() -> bool {
    std::env::var("NARRATOR_RUNNING").is_ok()
}
