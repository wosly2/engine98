use crate::math::{
    scalar::Scalar,
    vector::{Vec2, Vec3},
};
use new_macro::New;

#[derive(Clone, Copy)]
pub enum Shape2D {
    Point(Vec2),
    Line(Line2D),
    Triangle(Triangle2D),
    Rect(Rect2D),
    Circle(Circle2D),
}

pub enum Shape3D {
    Point(Vec3),
    Line(Vec3),
    Triangle(Vec3),
}

#[derive(Clone, Copy, New)]
pub struct Line3D {
    pub a: Vec3,
    pub b: Vec3,
}

#[derive(Clone, Copy, New)]
pub struct Triangle3D {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

#[derive(Clone, Copy, New)]
pub struct Line2D {
    pub a: Vec2,
    pub b: Vec2,
}

#[derive(Clone, Copy, New)]
pub struct Triangle2D {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}

impl Triangle2D {
    /// This is faster than `area()`. Signed.
    pub fn double_area(&self) -> Scalar {
        let i = self.b - self.a;
        let j = self.c - self.a;

        i.x() * j.y() - i.y() * j.x()
    }

    // Signed area. Slower than `double_area()`.
    pub fn area(&self) -> Scalar {
        self.double_area() / 2.
    }

    pub fn barycentric_coord(&self, point: Vec2) -> (Scalar, Scalar, Scalar) {
        self.barycentric_coord_double_area(point, self.double_area())
    }

    pub fn barycentric_coord_double_area(
        &self,
        point: Vec2,
        double_area: Scalar,
    ) -> (Scalar, Scalar, Scalar) {
        (
            (Triangle2D::new(self.b, self.c, point).double_area() / double_area).abs(),
            (Triangle2D::new(self.a, self.b, point).double_area() / double_area).abs(),
            (Triangle2D::new(self.a, self.c, point).double_area() / double_area).abs(),
        )
    }
}

#[derive(Clone, Copy, New)]
pub struct Rect2D {
    pub a: Vec2,
    pub b: Vec2,
}

#[derive(Clone, Copy, New)]
pub struct Circle2D {
    pub c: Vec2,
    pub r: Scalar,
}
