use crate::{
    gl::image::{Image, color::Color},
    gl::shape::{ShapeDrawOptions, raster_over_line},
    math::{
        shape::{Line2D, Rect2D, Shape2D, Triangle2D},
        vector::Vec2,
    },
};

impl Image<Color> {
    /// Draw a `Line` with a `Color` onto an `Image`. If a point along
    /// the `Line` does not exist in the `Image`, it will not be drawn
    pub fn draw_line(&mut self, line: Line2D, color: Color) {
        // TODO !
        // add a clipping operation if offscreen
        // by calculating the intersection with
        // the image boundary

        _ = raster_over_line(line, |x, y| {
            _ = self.set(x, y, color);
        });
    }

    /// Draw the outline of a `Triangle` using `Line`s onto an `Image`
    pub fn draw_triangle_outline(&mut self, triangle: Triangle2D, color: Color) {
        self.draw_line(Line2D::new(triangle.a, triangle.b), color);
        self.draw_line(Line2D::new(triangle.b, triangle.c), color);
        self.draw_line(Line2D::new(triangle.c, triangle.a), color);
    }

    /// Draw the outline of a `Rect` using `Line`s onto an `Image`
    pub fn draw_rect_outline(&mut self, rect: Rect2D, color: Color) {
        let p0 = rect.a;
        let p1 = Vec2::new([rect.a.x(), rect.b.y()]);
        let p2 = rect.b;
        let p3 = Vec2::new([rect.b.x(), rect.a.y()]);

        self.draw_line(Line2D::new(p0, p1), color);
        self.draw_line(Line2D::new(p1, p2), color);
        self.draw_line(Line2D::new(p2, p3), color);
        self.draw_line(Line2D::new(p3, p0), color);
        self.draw_line(Line2D::new(rect.a, rect.b), color);
    }

    /// Draw a `Shape` onto an `Image` using `ShapeDrawOptions` to configure style
    pub fn draw_shape(&mut self, shape: Shape2D, options: ShapeDrawOptions) {
        if let Some(_color) = options.fill {
            match shape {
                _ => {}
            };
        }

        if let Some((color, _thickness)) = options.outline {
            match shape {
                Shape2D::Line(line) => self.draw_line(line, color),
                Shape2D::Triangle(triangle) => self.draw_triangle_outline(triangle, color),
                Shape2D::Rect(rect) => self.draw_rect_outline(rect, color),
                _ => {}
            };
        };
    }
}
