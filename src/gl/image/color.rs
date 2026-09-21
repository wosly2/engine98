use crate::math::vector::Vec3;

/// Alias for a `u32` in the 0xRRGGBB format
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Color(pub u32);

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Color(value)
    }
}

impl Color {
    pub const fn rgb_255f(r: f64, g: f64, b: f64) -> Self {
        Self::rgb_255u(r as u32, g as u32, b as u32)
    }

    pub const fn rgb_255u(r: u32, g: u32, b: u32) -> Self {
        Color((r << 16) | (g << 8) | b)
    }

    pub const fn rgb_norm(r: f64, g: f64, b: f64) -> Self {
        Self::rgb_255u((r * 255.) as u32, (g * 255.) as u32, (b * 255.) as u32)
    }

    pub const fn vec_norm(v: Vec3) -> Self {
        Self::rgb_norm(v.x(), v.y(), v.z())
    }

    pub const fn shade(n: f64) -> Self {
        Self::rgb_norm(n, n, n)
    }

    pub const fn as_u32_slice(colors: &[Color]) -> &[u32] {
        unsafe { std::slice::from_raw_parts(colors.as_ptr() as *const u32, colors.len()) }
    }
}
