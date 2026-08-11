use std::{fmt, ops::*};

use new_macro::New;

pub type Scalar = f64;

// --- Shapes ---

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

// --- Vec2 ---

#[derive(Clone, Copy, PartialEq, New)]
pub struct Vec2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0., y: 0. };
    pub const UP: Self = Self { x: 0., y: 1. };
    pub const DOWN: Self = Self { x: 0., y: -1. };
    pub const LEFT: Self = Self { x: -1., y: 0. };
    pub const RIGHT: Self = Self { x: 1., y: 0. };

    pub fn double(scalar: Scalar) -> Self {
        Self {
            x: scalar,
            y: scalar,
        }
    }

    pub fn scale(self, scalar: Scalar) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn negative(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    pub fn transform_by(self: Self, other: Mat2) -> Self {
        (other.ihat * self.x) + (other.jhat * self.y)
    }

    pub fn dot(self: Self, other: Self) -> Scalar {
        (self.x * other.x) + (self.y * other.y)
    }
}

// --- Vec2 pretty print ---

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// --- Vec2 op overloads ---

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        self.add(rhs)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        self.sub(rhs)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        self.negative()
    }
}

impl Mul<Vec2> for Scalar {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs.scale(self)
    }
}

impl Mul<Scalar> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self.scale(rhs)
    }
}

// --- Mat2 ---

#[derive(Clone, Copy, PartialEq, New)]
pub struct Mat2 {
    pub ihat: Vec2,
    pub jhat: Vec2,
}

impl Mat2 {
    /// ```
    /// [0 ix, 1 jx
    ///  2 iy, 3 jy]
    /// ```
    pub fn from_list(scalars: [Scalar; 4]) -> Self {
        Self {
            ihat: Vec2 {
                x: scalars[0],
                y: scalars[2],
            },
            jhat: Vec2 {
                x: scalars[1],
                y: scalars[3],
            },
        }
    }

    fn compose_by(self, other: Self) -> Self {
        Self {
            ihat: other * self.ihat,
            jhat: other * self.jhat,
        }
    }
}

// --- Mat2 pretty print ---

impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} {}  {} {}]",
            self.ihat.x, self.ihat.y, self.jhat.x, self.jhat.y
        )
    }
}

// --- Mat2 op overloads ---

impl Mul<Mat2> for Mat2 {
    type Output = Mat2;

    fn mul(self, rhs: Mat2) -> Self::Output {
        rhs.compose_by(self)
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs.transform_by(self)
    }
}

// --- Vec3 ---

#[derive(Clone, Copy, PartialEq, New)]
pub struct Vec3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    pub const UP: Self = Self {
        x: 0.,
        y: 0.,
        z: 1.,
    };
    pub const DOWN: Self = Self {
        x: 0.,
        y: 0.,
        z: -1.,
    };
    pub const RIGHT_X: Self = Self {
        x: 1.,
        y: 0.,
        z: 0.,
    };
    pub const LEFT_X: Self = Self {
        x: -1.,
        y: 0.,
        z: 0.,
    };
    pub const RIGHT_Y: Self = Self {
        x: 0.,
        y: 1.,
        z: 0.,
    };
    pub const LEFT_Y: Self = Self {
        x: 0.,
        y: -1.,
        z: 0.,
    };

    pub fn triple(scalar: Scalar) -> Self {
        Self {
            x: scalar,
            y: scalar,
            z: scalar,
        }
    }

    pub fn scale(self, scalar: Scalar) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn negative(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn transform_by(self: Self, other: Mat3) -> Self {
        (other.ihat * self.x) + (other.jhat * self.y) + (other.khat * self.z)
    }

    pub fn dot(self: Self, other: Self) -> Scalar {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

// --- Vec3 pretty print ---

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// --- Vec3 op overloads ---

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self.add(rhs)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self.sub(rhs)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self.negative()
    }
}

impl Mul<Vec3> for Scalar {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.scale(self)
    }
}

impl Mul<Scalar> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self.scale(rhs)
    }
}

// --- Mat3 ---

#[derive(Clone, Copy, PartialEq, New)]
pub struct Mat3 {
    pub ihat: Vec3,
    pub jhat: Vec3,
    pub khat: Vec3,
}

impl Mat3 {
    /// ```
    /// [0 ix, 1 jx, 2 kx
    ///  3 iy, 4 jy, 5 ky
    ///  6 iz, 7 jz, 8 kz]
    /// ```
    pub fn from_list(scalars: [Scalar; 9]) -> Self {
        Self {
            ihat: Vec3 {
                x: scalars[0],
                y: scalars[3],
                z: scalars[6],
            },
            jhat: Vec3 {
                x: scalars[1],
                y: scalars[4],
                z: scalars[7],
            },
            khat: Vec3 {
                x: scalars[2],
                y: scalars[5],
                z: scalars[8],
            },
        }
    }

    fn compose_by(self, other: Self) -> Self {
        Self {
            ihat: other * self.ihat,
            jhat: other * self.jhat,
            khat: other * self.khat,
        }
    }
}

// --- Mat3 pretty print ---

impl fmt::Display for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} {} {}  {} {} {}  {}, {}, {}]",
            self.ihat.x,
            self.ihat.y,
            self.ihat.z,
            self.jhat.x,
            self.jhat.y,
            self.jhat.z,
            self.khat.x,
            self.khat.y,
            self.khat.z,
        )
    }
}

// --- Mat3 op overloads ---

impl Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        rhs.compose_by(self)
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.transform_by(self)
    }
}

// --- Transform3D ---

pub struct Transform3D {
    pub basis: Mat3,
    pub translation: Vec3,
}
