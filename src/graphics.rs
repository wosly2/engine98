use crate::math::{Line, Rect, Shape, Triangle, Vec2};

pub type Color = u32;

#[derive(Clone)]
pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub inner: Vec<Color>,
}

pub struct ShapeDrawOptions {
    /// `(Color, Thickness)`
    pub line: Option<(Color, u32)>,
    pub fill: Option<Color>,
}

impl Default for ShapeDrawOptions {
    fn default() -> Self {
        Self {
            line: Some((0xFFFFFF, 1)),
            fill: None,
        }
    }
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            inner: vec![0; width * height],
        }
    }

    pub fn fill(mut self, color: Color) -> Self {
        self.inner = vec![color; self.width * self.height];
        self
    }

    pub fn on_buffer(&self, x: i32, y: i32) -> Result<(usize, usize), ()> {
        if 0 > x || x >= self.width as i32 || 0 > y || y >= self.height as i32 {
            Err(())
        } else {
            Ok((x as usize, y as usize))
        }
    }

    pub fn index(&self, x: i32, y: i32) -> Result<usize, ()> {
        let (x, y) = self.on_buffer(x, y)?;
        Ok((self.height - 1 - y) * self.width + x)
    }

    pub fn get(&self, x: i32, y: i32) -> Result<Color, ()> {
        Ok(self.inner[self.index(x, y)?])
    }

    pub fn set(mut self, x: i32, y: i32, color: Color) -> Result<Self, ()> {
        let index = self.index(x, y)?;
        self.inner[index] = color;
        Ok(self)
    }

    pub fn line(mut self, line: Line, color: Color) -> Self {
        over_line(line, |x, y| {
            if let Ok(updated) = self.clone().set(x, y, color) {
                self = updated;
            };
        });

        self
    }

    pub fn triangle(self, triangle: Triangle, color: Color) -> Self {
        self.line(Line::new(triangle.a, triangle.b), color)
            .line(Line::new(triangle.b, triangle.c), color)
            .line(Line::new(triangle.c, triangle.a), color)
    }

    pub fn rect(self, rect: Rect, color: Color) -> Self {
        let p0 = rect.a;
        let p1 = Vec2::new(rect.a.x, rect.b.y);
        let p2 = rect.b;
        let p3 = Vec2::new(rect.b.x, rect.a.y);

        self.line(Line::new(p0, p1), color)
            .line(Line::new(p1, p2), color)
            .line(Line::new(p2, p3), color)
            .line(Line::new(p3, p0), color)
            .line(Line::new(rect.a, rect.b), color)
    }

    pub fn shape(mut self, shape: Shape, options: ShapeDrawOptions) -> Self {
        if let Some(_color) = options.fill {
            self = match shape {
                _ => self,
            };
        }

        if let Some((color, _thickness)) = options.line {
            self = match shape {
                Shape::Line(line) => self.line(line, color),
                Shape::Triangle(triangle) => self.triangle(triangle, color),
                Shape::Rect(rect) => self.rect(rect, color),
                _ => self,
            };
        }

        self
    }
}

pub fn over_line<F>(line: Line, mut f: F)
where
    F: FnMut(i32, i32),
{
    let (x0, y0, x1, y1) = (
        line.a.x as i32,
        line.a.y as i32,
        line.b.x as i32,
        line.b.y as i32,
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

fn _line_values(line: Line) -> (i32, i32, i32, Vec<(i32, i32)>) {
    let mut y_values = Vec::new();

    let mut previous_x = line.a.x as i32 + 1; // any value other than x, so first check works

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

    let sx = if line.a.x < line.b.x { 1 } else { -1 };
    let sy = if line.a.y < line.b.y { 1 } else { -1 };

    (line.a.x as i32, sx, sy, y_values)
}
