pub mod color;
pub mod draw;

/// Data representation of a sized image. `Image` contains a list of
/// pixels stored in a row-major `Vec<T>`.
///
/// `Image` implements some basic drawing abilities, such as filling the
///  length with a `T` or drawing a `Shape` with `ShapeDrawOptions`.
#[derive(Clone)]
pub struct Image<T: Clone + Copy> {
    pub width: usize,
    pub height: usize,
    pub inner: Vec<T>,
}

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

    /// Return the value of a pixel that exists in an `Image`
    pub fn get(&self, x: i64, y: i64) -> Result<T, ()> {
        Ok(self.inner[self.index(x, y)?])
    }

    /// Set the value of a point that exists inside an `Image`.
    pub fn set(&mut self, x: i64, y: i64, value: T) -> Result<(), ()> {
        let index = self.index(x, y)?;
        self.inner[index] = value;

        Ok(())
    }
}

impl<T: PartialOrd + Copy + Clone> Image<T> {
    /// Updates an existing pixel on the `Image` only when the
    /// provided value was greater than the existing pixel value.
    /// Returns a boolean describing whether the provided value
    /// was greater than the existing.
    pub fn set_if_greater(&mut self, x: i64, y: i64, value: T) -> Result<bool, ()> {
        if value > self.get(x, y)? {
            self.set(x, y, value)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn set_if_less(&mut self, x: i64, y: i64, value: T) -> Result<bool, ()> {
        if value < self.get(x, y)? {
            self.set(x, y, value)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
