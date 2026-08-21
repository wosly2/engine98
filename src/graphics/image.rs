use crate::graphics::{
    color::Color,
    shape::{ShapeDrawOptions, over_line},
};
use crate::math::{
    shape::{Line2D, Rect2D, Shape2D, Triangle2D},
    vector::Vec2,
};

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

    /// Set every value in a `Image` to a given `Color`
    pub fn fill(&self, color: Color) -> Self {
        Self {
            width: self.width,
            height: self.height,
            inner: vec![color; self.width * self.height],
        }
    }

    /// Check if a given point is within the logical bounds of
    /// a `Image`
    pub fn is_on_image(&self, x: i64, y: i64) -> bool {
        !(0 > x || x >= self.width as i64 || 0 > y || y >= self.height as i64)
    }

    /// Return the corresponding index for a point that exists in a `Image`
    pub fn index(&self, x: i64, y: i64) -> Result<usize, ()> {
        let (x, y) = self
            .is_on_image(x, y)
            .then(|| (x as usize, y as usize))
            .ok_or(())?;

        Ok((self.height - 1 - y) * self.width + x)
    }

    /// Return the `Color` of a point that exists in a `Image`
    pub fn get(&self, x: i64, y: i64) -> Result<Color, ()> {
        Ok(self.inner[self.index(x, y)?])
    }

    /// Set the `Color` of a point that exists inside a `Image`.
    pub fn set(mut self, x: i64, y: i64, color: Color) -> Result<Self, ()> {
        let index = self.index(x, y)?;
        self.inner[index] = color;
        Ok(self)
    }

    /// Draw a `Line` with a `Color` onto a `Image`. If a point along
    /// the `Line` does not exist in the `Image`, it will not be drawn
    pub fn draw_line(mut self, line: Line2D, color: Color) -> Self {
        _ = over_line(line, |x, y| {
            if let Ok(updated) = self.clone().set(x, y, color) {
                self = updated;
            };
        });

        self
    }

    /// Draw the outline of a `Triangle` using `Line`s onto a `Image`
    pub fn draw_triangle_outline(self, triangle: Triangle2D, color: Color) -> Self {
        self.draw_line(Line2D::new(triangle.a, triangle.b), color)
            .draw_line(Line2D::new(triangle.b, triangle.c), color)
            .draw_line(Line2D::new(triangle.c, triangle.a), color)
    }

    /// Draw the outline of a `Rect` using `Line`s onto a `Image`
    pub fn draw_rect_outline(self, rect: Rect2D, color: Color) -> Self {
        let p0 = rect.a;
        let p1 = Vec2::new([rect.a.x(), rect.b.y()]);
        let p2 = rect.b;
        let p3 = Vec2::new([rect.b.x(), rect.a.y()]);

        self.draw_line(Line2D::new(p0, p1), color)
            .draw_line(Line2D::new(p1, p2), color)
            .draw_line(Line2D::new(p2, p3), color)
            .draw_line(Line2D::new(p3, p0), color)
            .draw_line(Line2D::new(rect.a, rect.b), color)
    }

    /// Draw a `Shape` onto a `Image` using `ShapeDrawOptions` to configure style
    pub fn draw_shape(mut self, shape: Shape2D, options: ShapeDrawOptions) -> Self {
        if let Some(_color) = options.fill {
            self = match shape {
                _ => self,
            };
        }

        if let Some((color, _thickness)) = options.outline {
            self = match shape {
                Shape2D::Line(line) => self.draw_line(line, color),
                Shape2D::Triangle(triangle) => self.draw_triangle_outline(triangle, color),
                Shape2D::Rect(rect) => self.draw_rect_outline(rect, color),
                _ => self,
            };
        }

        self
    }
}
