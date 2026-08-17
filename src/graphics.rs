use crate::math::{Line, Mat4, Rect, Scalar, Shape, Triangle, Vec2};

/// Alias for a `u32` in the 0xRRGGBB format
pub type Color = u32;

/// Data representation of a sized image. `Buffer` contains a list of
/// pixels stored in a row-major `Vec<Color>`.
///
/// `Buffer` implements some basic drawing abilities, such as filling the
///  length with a `Color` or drawing a `Shape` with `ShapeDrawOptions`.
///
/// Most operations on a `Buffer` return a modified clone of the `Buffer`
/// with the new data, rather than consuming a reference.
#[derive(Clone)]
pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub inner: Vec<Color>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            inner: vec![0; width * height],
        }
    }

    /// Set every value in a `Buffer` to a given `Color`
    pub fn fill(mut self, color: Color) -> Self {
        self.inner = vec![color; self.width * self.height];
        self
    }

    /// Check if a given point is within the logical bounds of
    /// a `Buffer`
    pub fn is_on_buffer(&self, x: i32, y: i32) -> bool {
        !(0 > x || x >= self.width as i32 || 0 > y || y >= self.height as i32)
    }

    /// Return the corresponding index for a point that exists in a `Buffer`
    pub fn index(&self, x: i32, y: i32) -> Result<usize, ()> {
        let (x, y) = self
            .is_on_buffer(x, y)
            .then(|| (x as usize, y as usize))
            .ok_or(())?;

        Ok((self.height - 1 - y) * self.width + x)
    }

    /// Return the `Color` of a point that exists in a `Buffer`
    pub fn get(&self, x: i32, y: i32) -> Result<Color, ()> {
        Ok(self.inner[self.index(x, y)?])
    }

    /// Set the `Color` of a point that exists inside a `Buffer`.
    pub fn set(mut self, x: i32, y: i32, color: Color) -> Result<Self, ()> {
        let index = self.index(x, y)?;
        self.inner[index] = color;
        Ok(self)
    }

    /// Draw a `Line` with a `Color` onto a `Buffer`. If a point along
    /// the `Line` does not exist in the `Buffer`, it will not be drawn
    pub fn draw_line(mut self, line: Line, color: Color) -> Self {
        over_line(line, |x, y| {
            if let Ok(updated) = self.clone().set(x, y, color) {
                self = updated;
            };
        });

        self
    }

    /// Draw the outline of a `Triangle` using `Line`s onto a `Buffer`
    pub fn draw_triangle_outline(self, triangle: Triangle, color: Color) -> Self {
        self.draw_line(Line::new(triangle.a, triangle.b), color)
            .draw_line(Line::new(triangle.b, triangle.c), color)
            .draw_line(Line::new(triangle.c, triangle.a), color)
    }

    /// Draw the outline of a `Rect` using `Line`s onto a `Buffer`
    pub fn draw_rect_outline(self, rect: Rect, color: Color) -> Self {
        let p0 = rect.a;
        let p1 = Vec2::new([rect.a.x(), rect.b.y()]);
        let p2 = rect.b;
        let p3 = Vec2::new([rect.b.x(), rect.a.y()]);

        self.draw_line(Line::new(p0, p1), color)
            .draw_line(Line::new(p1, p2), color)
            .draw_line(Line::new(p2, p3), color)
            .draw_line(Line::new(p3, p0), color)
            .draw_line(Line::new(rect.a, rect.b), color)
    }

    /// Draw a `Shape` onto a `Buffer` using `ShapeDrawOptions` to configure style
    pub fn draw_shape(mut self, shape: Shape, options: ShapeDrawOptions) -> Self {
        if let Some(_color) = options.fill {
            self = match shape {
                _ => self,
            };
        }

        if let Some((color, _thickness)) = options.outline {
            self = match shape {
                Shape::Line(line) => self.draw_line(line, color),
                Shape::Triangle(triangle) => self.draw_triangle_outline(triangle, color),
                Shape::Rect(rect) => self.draw_rect_outline(rect, color),
                _ => self,
            };
        }

        self
    }
}

/// Configures the style of a drawn `Shape`
pub struct ShapeDrawOptions {
    /// `(Color, Thickness)`
    pub outline: Option<(Color, u32)>,
    pub fill: Option<Color>,
}

impl Default for ShapeDrawOptions {
    fn default() -> Self {
        Self {
            outline: Some((0xFFFFFF, 1)),
            fill: None,
        }
    }
}
/// Operate a closure over each plotted `(X, Y)` coordinate
/// over the given line according to Bresenham's algorithm
pub fn over_line<F>(line: Line, mut f: F)
where
    F: FnMut(i32, i32),
{
    let (x0, y0, x1, y1) = (
        line.a.x() as i32,
        line.a.y() as i32,
        line.b.x() as i32,
        line.b.y() as i32,
    );

    let (mut x, mut y) = (x0, y0);

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut error = dx + dy;

    loop {
        f(x, y);

        let e2 = 2 * error;
        if e2 >= dy {
            if x == x1 {
                break;
            }
            error += dy;
            x += sx;
        }
        if e2 <= dx {
            if y == y1 {
                break;
            }
            error += dx;
            y += sy;
        }
    }
}

pub fn perspective(fov: Scalar) -> Mat4 {
    Mat4::from([
        [fov / 2., 0., 0., 0.],
        [0., fov / 2., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ])
}

fn _line_values(line: Line) -> (i32, i32, i32, Vec<(i32, i32)>) {
    let mut y_values = Vec::new();

    let mut previous_x = line.a.x() as i32 + 1; // any value other than x, so first check works

    let mut i = 0;

    over_line(line, |x, y| {
        if previous_x != x {
            // add this value
            y_values.push((y, 1));

            previous_x = x;

            i += 1;
        } else {
            // update how far it goes down
            y_values[i - 1].1 += 1;
        }
    });

    let sx = if line.a.x() < line.b.x() { 1 } else { -1 };
    let sy = if line.a.y() < line.b.y() { 1 } else { -1 };

    (line.a.x() as i32, sx, sy, y_values)
}
