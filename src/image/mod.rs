pub mod color;

use crate::image::color::Color;

/// Data representation of a sized image. `Image` contains a list of
/// pixels stored in a row-major `Vec<Color>`.
///
/// `Image` implements some basic drawing abilities, such as filling the
///  length with a `Color` or drawing a `Shape` with `ShapeDrawOptions`.
///
/// Most operations on a `Image` return a modified clone of the `Image`
/// with the new data, rather than consuming a mutable reference.
#[derive(Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub inner: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            inner: vec![0; width * height],
        }
    }

    /// Set every value in an `Image` to a given `Color`
    pub fn fill(&self, color: Color) -> Self {
        Self {
            width: self.width,
            height: self.height,
            inner: vec![color; self.width * self.height],
        }
    }

    /// Check if a given point is within the bounds of
    /// an `Image`
    pub fn is_on_image(&self, x: i64, y: i64) -> bool {
        !(0 > x || x >= self.width as i64 || 0 > y || y >= self.height as i64)
    }

    /// Return the corresponding index for a point that exists in an `Image`.
    /// Images have increasing Y and decreasing index from bottom -> top.
    pub fn index(&self, x: i64, y: i64) -> Result<usize, ()> {
        let (x, y) = self
            .is_on_image(x, y)
            .then(|| (x as usize, y as usize))
            .ok_or(())?;

        Ok((self.height - 1 - y) * self.width + x)
    }

    /// Clamp a given point so that neither X nor Y component
    /// is outside of the bounds of an `Image`
    pub fn clamp(&self, x: i64, y: i64) -> (i64, i64) {
        (
            x.clamp(0, self.width as i64 - 1),
            y.clamp(0, self.height as i64 - 1),
        )
    }

    /// Return the `Color` of a point that exists in an `Image`
    pub fn get(&self, x: i64, y: i64) -> Result<Color, ()> {
        Ok(self.inner[self.index(x, y)?])
    }

    /// Set the `Color` of a point that exists inside an `Image`.
    pub fn set(&mut self, x: i64, y: i64, color: Color) -> Result<(), ()> {
        let index = self.index(x, y)?;
        self.inner[index] = color;

        Ok(())
    }
}
