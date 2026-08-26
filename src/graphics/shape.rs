use crate::graphics::shape::LineStepError::{Finished, InfinitePoints};
use crate::image::color::Color;
use crate::math::shape::{Line2D, Triangle2D};
use crate::util;

/// Configures the style of a drawn `Shape`
pub struct ShapeDrawOptions {
    /// `(Color, Thickness)`
    pub outline: Option<(Color, u32)>,
    pub fill: Option<Color>,
}

impl Default for ShapeDrawOptions {
    fn default() -> Self {
        Self {
            outline: Some((0xFFFFFF.into(), 1)),
            fill: None,
        }
    }
}

/*
 * ==================================
 *   RASTER CLOSURE CONSTRUCTIONS
 * ==================================
*/

/// Invoke a function `f` over each plotted `(X, Y)` coordinate
/// over the given line between `line.a` and `line.b` according
/// to Bresenham's algorithm.
///
/// Returns `Err(())` if any endpoint contains infinite values in
/// its coordinate.
pub fn raster_over_line<F>(line: Line2D, mut f: F) -> Result<(), LineStepError>
where
    F: FnMut(i64, i64),
{
    let stepper = LineStepper::new(line)?;

    for (x, y) in stepper {
        f(x, y)
    }

    if let Some(reason) = stepper.stop_reason() {
        Err(reason)
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct LineStepper {
    x: i64,
    y: i64,
    x1: i64,
    y1: i64,
    error: i64,
    dx: i64,
    sx: i64,
    dy: i64,
    sy: i64,
    finished: bool,
    stopped_because: Option<LineStepError>,
}

#[derive(Clone, Copy)]
pub enum LineStepError {
    InfinitePoints,
    Finished,
}

pub fn replace_line_finished<T>(
    res: Result<T, LineStepError>,
    replace: T,
) -> Result<T, LineStepError> {
    match res {
        Ok(o) => Ok(o),
        Err(e) => match e {
            Finished => Ok(replace),
            _ => Err(e),
        },
    }
}

impl LineStepper {
    pub fn new(line: Line2D) -> Result<Self, LineStepError> {
        if line.a.x().is_infinite()
            || line.a.y().is_infinite()
            || line.b.x().is_infinite()
            || line.b.y().is_infinite()
        {
            return Err(InfinitePoints);
        }

        let (x0, y0, x1, y1) = (
            line.a.x() as i64,
            line.a.y() as i64,
            line.b.x() as i64,
            line.b.y() as i64,
        );

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };

        let error = dx + dy;

        Ok(LineStepper {
            x: x0,
            y: y0,
            x1,
            y1,
            error,
            dx,
            sx,
            dy,
            sy,
            finished: false,
            stopped_because: None,
        })
    }

    fn finish(&mut self) {
        self.finished = true;
        self.stopped_because = Some(Finished)
    }

    pub fn step(&mut self) -> Result<(), LineStepError> {
        if self.x == self.x1 && self.y == self.y1 {
            self.finish();
        }

        if self.finished {
            return Err(Finished);
        };

        let e2 = 2 * self.error;
        if e2 >= self.dy {
            if self.x == self.x1 {
                return Ok(());
            }
            self.error += self.dy;
            self.x += self.sx;
        }
        if e2 <= self.dx {
            if self.y == self.y1 {
                return Ok(());
            }
            self.error += self.dx;
            self.y += self.sy;
        }

        Ok(())
    }

    pub fn xy(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn stop_reason(&self) -> Option<LineStepError> {
        self.stopped_because
    }

    pub fn as_edge(self, axis: Axis2D) -> EdgeStepper {
        EdgeStepper {
            stepper: self,
            axis,
        }
    }
}

impl Iterator for LineStepper {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let xy = self.xy();
        if let Ok(_) = self.step() {
            Some(xy)
        } else {
            None
        }
    }
}

pub enum Axis2D {
    X,
    Y,
}

pub struct EdgeStepper {
    stepper: LineStepper,
    pub axis: Axis2D,
}

impl EdgeStepper {
    pub fn line_stepper(&self) -> &LineStepper {
        &self.stepper
    }

    pub fn xy(&self) -> (i64, i64) {
        self.stepper.xy()
    }

    pub fn step(&mut self) -> Result<(), LineStepError> {
        loop {
            let current = self.xy();
            self.stepper.step()?;
            let next = self.stepper.xy();

            if match self.axis {
                Axis2D::X => current.0 != next.0,
                Axis2D::Y => current.1 != next.1,
            } {
                break Ok(());
            }
        }
    }
}

impl Iterator for EdgeStepper {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let xy = self.stepper.xy();
        if let Ok(_) = self.stepper.step() {
            Some(xy)
        } else {
            None
        }
    }
}

pub fn raster_over_triangle_area_by_edges<F>(
    triangle: Triangle2D,
    mut f: F,
) -> Result<(), LineStepError>
where
    F: FnMut(i64, i64),
{
    // our current goal is to determine which line spans the full Y
    // displacement of the two others. we do this by sorting the lines
    // based on their Y values, then taking our "span" line as
    //
    //      sorted.0 -> sorted.2    (top -> bottom)
    //
    // and our two "spanned" as
    //
    //      sorted.0 -> sorted.1    (top -> middle)
    //      sorted.1 -> sorted.2    (middle -> bottom)

    let mut sorted = [triangle.a, triangle.b, triangle.c];
    sorted.sort_by(|p0, p1| p0.y().total_cmp(&p1.y()));

    // get the raster run-length data for each of our sorted lines in
    // the triangle, so that we can determine the proper X run length
    // to fill for each Y offset for the scanline operation

    let mut long = LineStepper::new(Line2D::new(sorted[0], sorted[2]))?.as_edge(Axis2D::Y);
    let mut short = LineStepper::new(Line2D::new(sorted[0], sorted[1]))?.as_edge(Axis2D::Y);

    for step in 0..2 {
        loop {
            let (long_x, long_y) = long.xy();
            let (short_x, short_y) = short.xy();

            assert_eq!(long_y, short_y);

            for x in util::bi_range(long_x, short_x) {
                f(x, long_y);
            }

            // step!

            replace_line_finished(short.step(), ())?;

            if short.line_stepper().is_finished() {
                break;
            } else {
                // we don't want to consume past the length of short1
                replace_line_finished(long.step(), ())?;
            }
        }

        // switch to short2
        if step == 0 {
            short = LineStepper::new(Line2D::new(sorted[1], sorted[2]))?.as_edge(Axis2D::Y);
        }
    }

    Ok(())
}

/// Takes a `Line2D` and builds a compressed data representation of the line's
/// raster coordinates. `raster_line_runs` returns the following data in tuple form:
/// ```
/// (
///     i64,     // starting Y coordinate of given line
///     i64,     // sign of X direction
///     i64,     // sign of Y direction
///     Vec<(
///         i64, // X value where Y value is starting Y + index here * sign of Y direction
///         i64, // Length of contininuous X values in the direction of sign of X direction
///     )>,
/// )
/// ```
///
/// `raster_line_runs` uses `over_line` to obtain its raster coordinates, and thus
/// wraps its output tuple in a `Result`, propagating any error from `over_line`.
pub fn raster_line_runs(line: Line2D) -> Result<(i64, i64, i64, Vec<(i64, i64)>), LineStepError> {
    let mut x_values = Vec::new();

    let mut previous_y = line.a.y() as i64 + 1; // any value other than y, so first check works

    let mut i = 0;

    let err = raster_over_line(line, |x, y| {
        // if the y has changed, the line has moved along the Y
        if previous_y != y {
            // add this X offset for the Y offset represented by this index
            x_values.push((x, 1));

            previous_y = y;

            i += 1;
        // if the y has not changed, the line has moved along the X
        } else {
            // update how far it goes right/left by increasing the second tuple value representing unsigned X run
            x_values[i - 1].1 += 1;
        }
    });

    err.map(|_| {
        let sx = if line.a.x() < line.b.x() { 1 } else { -1 };
        let sy = if line.a.y() < line.b.y() { 1 } else { -1 };

        (line.a.y() as i64, sx, sy, x_values)
    })
}
