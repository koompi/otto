mod effects;
mod model;
mod render;
mod view;

/// A window's corner radius, in logical points, before the screen scale.
///
/// One value for the shadow and the content both. They used to disagree: only
/// the shadow was ever rounded, so a window's own corner stayed square and read
/// as a sharp cut sitting inside a soft shadow.
///
/// 18 is KOOMPI's `Appearance.rounding.windowRounding`, the same radius the
/// Hyprland session gives a window, and a little tighter than the 23 the bar's
/// hug uses at the screen edge.
pub const WINDOW_CORNER_RADIUS: f32 = 18.0;

pub use model::WindowViewBaseModel;
pub use model::WindowViewSurface;
pub use view::WindowView;
