use crate::graphics::color::Color;
use crate::math::shape::{Line2D, Triangle2D};

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
pub fn over_line<F>(line: Line2D, mut f: F) -> Result<(), ()>
where
    F: FnMut(i64, i64),
{
    if line.a.x().is_infinite()
        || line.a.y().is_infinite()
        || line.b.x().is_infinite()
        || line.b.y().is_infinite()
    {
        return Err(());
    }

    let (x0, y0, x1, y1) = (
        line.a.x() as i64,
        line.a.y() as i64,
        line.b.x() as i64,
        line.b.y() as i64,
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
                break Ok(());
            }
            error += dy;
            x += sx;
        }
        if e2 <= dx {
            if y == y1 {
                break Ok(());
            }
            error += dx;
            y += sy;
        }
    }
}

pub fn over_triangle<F>(_triangle: Triangle2D, mut _f: F) -> Result<(), ()>
where
    F: FnMut(i64, i64),
{
    Ok(())
}

fn _line_values(line: Line2D) -> (i64, i64, i64, Vec<(i64, i64)>) {
    let mut y_values = Vec::new();

    let mut previous_x = line.a.x() as i64 + 1; // any value other than x, so first check works

    let mut i = 0;

    _ = over_line(line, |x, y| {
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

    (line.a.x() as i64, sx, sy, y_values)
}
