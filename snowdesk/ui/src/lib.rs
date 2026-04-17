#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy)]
pub struct Radius(pub f32);

#[derive(Debug, Clone, Copy)]
pub enum MotionCurve {
    EaseInOutCubic,
    EaseOutQuad,
}

#[derive(Debug, Clone, Copy)]
pub struct MotionSpec {
    pub duration_ms: u16,
    pub curve: MotionCurve,
}

pub mod aqua {
    use super::{Color, MotionCurve, MotionSpec, Radius};

    pub const WINDOW_CORNER_RADIUS: Radius = Radius(8.0);
    pub const BUTTON_CORNER_RADIUS: Radius = Radius(6.0);

    pub const BLUE_HIGHLIGHT: Color = Color(76, 145, 255, 255);
    pub const GLASS_TOP: Color = Color(245, 248, 255, 220);
    pub const GLASS_BOTTOM: Color = Color(177, 190, 214, 220);

    pub const QUICK_ANIMATION: MotionSpec = MotionSpec {
        duration_ms: 180,
        curve: MotionCurve::EaseInOutCubic,
    };

    pub const NORMAL_ANIMATION: MotionSpec = MotionSpec {
        duration_ms: 260,
        curve: MotionCurve::EaseInOutCubic,
    };
}
