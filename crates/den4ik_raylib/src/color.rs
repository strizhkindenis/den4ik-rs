pub trait ToRlColor {
    fn to_rl_color(&self) -> crate::ffi::Color;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl ToRlColor for Color {
    fn to_rl_color(&self) -> crate::ffi::Color {
        crate::ffi::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

impl ToRlColor for crate::ffi::Color {
    fn to_rl_color(&self) -> crate::ffi::Color {
        *self
    }
}

// Custom raylib color palette for amazing visuals on WHITE background
impl Color {
    pub const LIGHTGRAY: Self = Self::new(200, 200, 200, 255);
    pub const GRAY: Self = Self::new(130, 130, 130, 255);
    pub const DARKGRAY: Self = Self::new(80, 80, 80, 255);
    pub const YELLOW: Self = Self::new(253, 249, 0, 255);
    pub const GOLD: Self = Self::new(255, 203, 0, 255);
    pub const ORANGE: Self = Self::new(255, 161, 0, 255);
    pub const PINK: Self = Self::new(255, 109, 194, 255);
    pub const RED: Self = Self::new(230, 41, 55, 255);
    pub const MAROON: Self = Self::new(190, 33, 55, 255);
    pub const GREEN: Self = Self::new(0, 228, 48, 255);
    pub const LIME: Self = Self::new(0, 158, 47, 255);
    pub const DARKGREEN: Self = Self::new(0, 117, 44, 255);
    pub const SKYBLUE: Self = Self::new(102, 191, 255, 255);
    pub const BLUE: Self = Self::new(0, 121, 241, 255);
    pub const DARKBLUE: Self = Self::new(0, 82, 172, 255);
    pub const PURPLE: Self = Self::new(200, 122, 255, 255);
    pub const VIOLET: Self = Self::new(135, 60, 190, 255);
    pub const DARKPURPLE: Self = Self::new(112, 31, 126, 255);
    pub const BEIGE: Self = Self::new(211, 176, 131, 255);
    pub const BROWN: Self = Self::new(127, 106, 79, 255);
    pub const DARKBROWN: Self = Self::new(76, 63, 47, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const BLANK: Self = Self::new(0, 0, 0, 0);
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);
    pub const RAYWHITE: Self = Self::new(245, 245, 245, 255);
}

pub fn fade(color: impl ToRlColor, alpha: f32) -> crate::ffi::Color {
    unsafe { crate::ffi::Fade(color.to_rl_color(), alpha) }
}
