use std::cmp::min;

use image::{GenericImageView, ImageError, ImageReader, Rgba};

use crate::{gl::image::color::Color, util::is_norm};

pub mod color;
pub mod draw;

/// Data representation of a sized image. `Image` contains a list of
/// pixels stored in a row-major `Vec<T>`.
///
/// `Image` implements some basic drawing abilities, such as filling the
///  length with a `T` or drawing a `Shape` with `ShapeDrawOptions`.
#[derive(Clone)]
pub struct Image<T: Clone + Copy + Sized> {
    pub width: usize,
    pub height: usize,
    pub inner: Vec<T>,
}

pub type Sampling<T> = fn(&Image<T>, norm_x: f64, norm_y: f64) -> Option<T>;

impl<T: Clone + Copy> Image<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            width,
            height,
            inner: vec![default; width * height],
        }
    }

    /// Set every value in an `Image` to a given value
    pub fn fill(&mut self, value: T) {
        self.inner = vec![value; self.width * self.height];
    }

    /// Check if a given point is within the bounds of
    /// an `Image`
    pub fn is_on_image(&self, x: i32, y: i32) -> bool {
        !(0 > x || x >= self.width as i32 || 0 > y || y >= self.height as i32)
    }

    pub fn index_unchecked(&self, x: i32, y: i32) -> usize {
        (self.height - 1 - y as usize) * self.width + x as usize
    }

    /// Return the corresponding index for a point that exists in an `Image`.
    /// Images have increasing Y and decreasing index from bottom -> top.
    pub fn index(&self, x: i32, y: i32) -> Option<usize> {
        if self.is_on_image(x, y) {
            Some(self.index_unchecked(x, y))
        } else {
            None
        }
    }

    /// Clamp a given point so that neither X nor Y component
    /// is outside of the bounds of an `Image`
    pub fn clamp(&self, x: i32, y: i32) -> (i32, i32) {
        (
            x.clamp(0, self.width as i32 - 1),
            y.clamp(0, self.height as i32 - 1),
        )
    }

    /// Return the value of a pixel that exists in an `Image`
    pub fn get(&self, x: i32, y: i32) -> Option<T> {
        Some(self.inner[self.index(x, y)?])
    }

    /// Set the value of a point that exists inside an `Image`.
    pub fn set(&mut self, x: i32, y: i32, value: T) -> Option<()> {
        let index = self.index(x, y)?;
        self.inner[index] = value;

        Some(())
    }

    pub fn blit_to(&self, dest: &mut Self, dest_x: usize, dest_y: usize) {
        if dest_x >= dest.width || dest_y >= dest.height {
            return;
        }

        let copy_width = min(self.width, dest.width - dest_x);

        for src_y_offset in 0..min(self.height, dest.height - dest_y) {
            let dest_index = dest.index_unchecked(dest_x as i32, (dest_y + src_y_offset) as i32);
            let src_index = self.index_unchecked(0, src_y_offset as i32);

            let src_row = &self.inner[src_index..src_index + copy_width];
            let dest_row = &mut dest.inner[dest_index..dest_index + copy_width];

            dest_row.copy_from_slice(src_row);
        }
    }

    pub fn sampled_nearest(&self, norm_x: f64, norm_y: f64) -> Option<T> {
        if !is_norm(norm_x) || !is_norm(norm_y) {
            return None;
        }

        let x = (norm_x * (self.width - 1) as f64).round() as i32;
        let y = (norm_y * (self.height - 1) as f64).round() as i32;

        Some(self.get(x, y)?)
    }

    /// TODO !
    #[allow(unused)]
    pub fn scaled(&self, new_width: usize, new_height: usize, sample: Sampling<T>) -> Option<Self> {
        let mut scaled = Self {
            width: new_width,
            height: new_height,
            inner: Vec::with_capacity(new_width * new_height),
        };

        for x in 0..new_width {
            for y in 0..new_height {
                scaled.inner.push(sample(self, 0., 0.)?);
            }
        }

        unimplemented!();
    }
}

impl<T: PartialOrd + Copy + Clone> Image<T> {
    /// Updates an existing pixel on the `Image` only when the
    /// provided value was greater than the existing pixel value.
    /// Returns a boolean describing whether the provided value
    /// was greater than the existing.
    pub fn set_if_greater(&mut self, x: i32, y: i32, value: T) -> Result<bool, ()> {
        if value > self.get(x, y).ok_or(())? {
            self.set(x, y, value).ok_or(())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn set_if_less(&mut self, x: i32, y: i32, value: T) -> Result<bool, ()> {
        if value < self.get(x, y).ok_or(())? {
            self.set(x, y, value).ok_or(())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// =============
// image loading
// =============

pub fn load_image<T: Copy + From<Rgba<u8>>>(path: String) -> Result<Image<T>, ImageError> {
    let file_image = ImageReader::open(path)?.decode()?;

    let mut out_image = Image {
        width: file_image.width() as usize,
        height: file_image.height() as usize,
        inner: Vec::new(),
    };

    // FIXME !
    // this assumes that the pixel format is
    // magically oriented in the same way as our
    // pixel buffer

    for pixel in file_image.pixels() {
        out_image.inner.push(pixel.2.into());
    }

    Ok(out_image)
}

impl From<Rgba<u8>> for Color {
    fn from(value: Rgba<u8>) -> Self {
        Color::rgb_255u(value.0[0] as u32, value.0[1] as u32, value.0[2] as u32)
    }
}
