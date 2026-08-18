use crate::math::{scalar::Scalar, vector::Vec2};
use new_macro::New;

#[derive(Clone, Copy)]
pub enum Shape {
    Point(Vec2),
    Line(Line),
    Triangle(Triangle),
    Rect(Rect),
    Circle(Circle),
}

#[derive(Clone, Copy, New)]
pub struct Line {
    pub a: Vec2,
    pub b: Vec2,
}

#[derive(Clone, Copy, New)]
pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}

#[derive(Clone, Copy, New)]
pub struct Rect {
    pub a: Vec2,
    pub b: Vec2,
}

#[derive(Clone, Copy, New)]
pub struct Circle {
    pub c: Vec2,
    pub r: Scalar,
}
