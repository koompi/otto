// Topbar layout/style constants (not user-configurable).
//
// These mirror KOOMPI's shell design tokens, so the Otto session's bar and the
// Hyprland session's bar read as one desktop. Source of truth for the numbers:
// dots/.config/quickshell/koompi/modules/common/Appearance.qml.

/// Bar height in logical points. `Appearance.sizes.baseBarHeight`.
pub const BAR_HEIGHT: u32 = 40;

/// The hug: the bar's material carried this far past its own bottom edge at the
/// screen edges and curved away, so a window below meets a curve rather than a
/// straight cut. `Appearance.rounding.screenRounding`.
pub const BAR_HUG: f32 = 23.0;

/// Left panel initial width (will animate to content size).
pub const LEFT_WIDTH: u32 = 80;

/// Right panel initial width (will animate to content size).
pub const RIGHT_WIDTH: u32 = 80;

/// Top margin from screen edge.
pub const BAR_MARGIN_TOP: i32 = 0;

/// Side margin from screen edge.
pub const BAR_MARGIN_SIDE: i32 = 0;

/// Horizontal padding inside a panel. The clock's inset from the screen edge in
/// the KOOMPI bar is `screenRounding`, and the app name matches it on the left.
pub const BAR_PADDING_H: f32 = 23.0;

/// Spacing between tray icons.
#[allow(dead_code)]
pub const TRAY_ICON_SPACING: f32 = 8.0;

/// Tray icon size in logical points.
pub const TRAY_ICON_SIZE: f32 = 22.0;

/// Gap between tray icons and the clock. `Appearance.spacing.normal`.
pub const TRAY_CLOCK_GAP: f32 = 10.0;

/// Right panel minimum width.
pub const MIN_RIGHT_WIDTH: u32 = 60;

/// Corner radius of a hovered or open item. `Appearance.rounding.small`.
pub const ITEM_CORNER_RADIUS: f32 = 12.0;

/// The bar's text face.
///
/// Named explicitly rather than left to otto-kit's "Inter" default: Inter is not
/// installed on a KOOMPI machine, and fontconfig answers a missing family with a
/// substitute rather than a failure, so the bar rendered every label in Noto
/// Sans Khmer - .notdef boxes for all of it.
pub const FONT_FAMILY: &str = "Google Sans Flex";

/// Label size, for the app name and the menu titles.
/// `Appearance.font.pixelSize.smaller`.
pub const FONT_SIZE_LABEL: f32 = 12.0;

/// Clock size. `Appearance.font.pixelSize.large`.
pub const FONT_SIZE_CLOCK: f32 = 17.0;

// ---------------------------------------------------------------------------
// Runtime config — loaded from topbar.toml on first access
// ---------------------------------------------------------------------------

use std::sync::LazyLock;

/// Default clock format: "21:16". The KOOMPI bar shows the time alone; the date
/// belongs to the panel behind the clock, not to the bar.
const DEFAULT_CLOCK_FORMAT: &str = "%H:%M";

/// User-configurable topbar settings.
#[derive(Debug, Clone)]
pub struct TopbarConfig {
    /// chrono strftime format string for the clock.
    pub clock_format: String,
}

impl Default for TopbarConfig {
    fn default() -> Self {
        Self {
            clock_format: DEFAULT_CLOCK_FORMAT.to_string(),
        }
    }
}

static CONFIG: LazyLock<TopbarConfig> = LazyLock::new(load_config);

/// Access the current topbar configuration.
#[allow(dead_code)]
pub fn config() -> &'static TopbarConfig {
    &CONFIG
}

/// Return the clock format string.
pub fn clock_format() -> &'static str {
    &CONFIG.clock_format
}

fn load_config() -> TopbarConfig {
    // Search order: /etc/otto/otto-bar.toml → ~/.config/otto/otto-bar.toml → ./otto-bar.toml
    let candidates: Vec<std::path::PathBuf> = {
        let mut v = Vec::new();
        v.push(std::path::PathBuf::from("/etc/otto/otto-bar.toml"));
        if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME")
            .map(std::path::PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".config"))
            })
        {
            v.push(xdg.join("otto").join("otto-bar.toml"));
        }
        v.push(std::path::PathBuf::from("otto-bar.toml"));
        v
    };

    let mut cfg = TopbarConfig::default();

    for path in &candidates {
        if let Ok(content) = std::fs::read_to_string(path) {
            match content.parse::<toml::Value>() {
                Ok(table) => {
                    if let Some(fmt) = table.get("clock_format").and_then(|v| v.as_str()) {
                        cfg.clock_format = fmt.to_string();
                    }
                    break;
                }
                Err(e) => {
                    tracing::warn!("Failed to parse {}: {e}", path.display());
                }
            }
        }
    }

    cfg
}
