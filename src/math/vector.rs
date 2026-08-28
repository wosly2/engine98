use crate::math::{INF_CUTOFF, scalar::Scalar};
use std::{
    fmt,
    ops::{Add, Mul, Neg, Sub},
};

use paste::paste;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vecn<const N: usize> {
    pub axes: [Scalar; N],
}

impl<const N: usize> Vecn<N> {
    pub const EMPTY: Self = Self { axes: [0.0; N] };

    pub const fn new(axes: [Scalar; N]) -> Self {
        Self { axes }
    }

    /// Update every axis in a `Vecn` with an operation
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(Scalar) -> Scalar,
    {
        Self {
            axes: self.axes.map(f),
        }
    }

    pub fn map_index<F>(&self, f: F) -> Self
    where
        F: Fn(Scalar, usize) -> Scalar,
    {
        let mut out = [0.0; N];
        let mut index = 0;
        for axis in self.axes {
            out[index] = f(axis, index);
            index += 1;
        }

        Vecn { axes: out }
    }

    pub fn clamp(self: Self, low: Self, high: Self) -> Self {
        self.map_index(|axis, index| axis.clamp(low.axes[index], high.axes[index]))
    }

    /// Create a `Vecn` with every value initialized to the same `Scalar`
    pub const fn splat(scalar: Scalar) -> Self {
        Self { axes: [scalar; N] }
    }

    /// Perform a `Vecn::scale()` by the inverse of the last axis of a `Vecn`.
    /// In homogeneous coordinate vectors, this is equivalent to finding the
    /// cartesian representation of the vector with respect to the homogeneous
    /// factor (such as `Vec4.w()` or `Vec3.z()`).
    ///
    /// If the inverse scalar is `f64::NAN` or greater than or equal to `INF_CUTOFF`,
    /// it will be reassigned the value `f64::INFINITY`.
    pub fn cartesian(self: Self) -> Self {
        let attempt = 1. / self.axes[N - 1];

        self.scale(
            if attempt >= INF_CUTOFF || attempt <= -INF_CUTOFF || attempt.is_nan() {
                f64::INFINITY
            } else {
                attempt
            },
        )
    }

    pub fn cartesian_z(self: Self) -> (Self, Scalar) {
        (self.cartesian(), self.axes[N - 1])
    }

    pub fn scale(self, scalar: Scalar) -> Self {
        Self {
            axes: self.axes.map(|axis| axis * scalar),
        }
    }

    pub fn add(self, other: Self) -> Self {
        Self {
            axes: std::array::from_fn(|i| self.axes[i] + other.axes[i]),
        }
    }

    pub fn sub(self, other: Self) -> Self {
        Self {
            axes: std::array::from_fn(|i| self.axes[i] - other.axes[i]),
        }
    }

    pub fn negative(self) -> Self {
        Self {
            axes: self.axes.map(|axis| -axis),
        }
    }

    pub fn dot(self, other: Self) -> Scalar {
        let scaled: [Scalar; N] = std::array::from_fn(|i| self.axes[i] * other.axes[i]);
        scaled.iter().sum()
    }
}

impl<const N: usize> Default for Vecn<N> {
    fn default() -> Self {
        Self {
            axes: [Scalar::default(); N],
        }
    }
}

impl<const N: usize> fmt::Display for Vecn<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.axes
                .iter()
                .map(|axis| axis.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl<const N: usize> Add<Vecn<N>> for Vecn<N> {
    type Output = Vecn<N>;

    fn add(self, rhs: Vecn<N>) -> Self::Output {
        self.add(rhs)
    }
}

impl<const N: usize> Sub<Vecn<N>> for Vecn<N> {
    type Output = Vecn<N>;

    fn sub(self, rhs: Vecn<N>) -> Self::Output {
        self.sub(rhs)
    }
}

impl<const N: usize> Neg for Vecn<N> {
    type Output = Vecn<N>;

    fn neg(self) -> Self::Output {
        self.negative()
    }
}

impl<const N: usize> Mul<Vecn<N>> for Scalar {
    type Output = Vecn<N>;

    fn mul(self, rhs: Vecn<N>) -> Self::Output {
        rhs.scale(self)
    }
}

impl<const N: usize> Mul<Scalar> for Vecn<N> {
    type Output = Vecn<N>;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self.scale(rhs)
    }
}

macro_rules! vector_accessors {
    ($vec_type:ty, $($name:ident: $index:tt),+) => {
        impl $vec_type {
            $(
                pub const fn $name(self) -> Scalar {
                    self.axes[$index]
                }

                paste! {
                    pub const fn [<set_ $name>](&mut self, scalar: Scalar) {
                        self.axes[$index] = scalar;
                    }
                }
            )+
        }
    };
}

vector_accessors!(Vec2, x: 0, y: 1);
vector_accessors!(Vec3, x: 0, y: 1, z: 2);
vector_accessors!(Vec4, x: 0, y: 1, z: 2, w: 2);

pub type Vec2 = Vecn<2>;
pub type Vec3 = Vecn<3>;
pub type Vec4 = Vecn<4>;

impl Vec2 {
    pub const ZERO: Self = Self { axes: [0., 0.] };
    pub const UP: Self = Self { axes: [0., 1.] };
    pub const DOWN: Self = Self { axes: [0., -1.] };
    pub const LEFT: Self = Self { axes: [-1., 0.] };
    pub const RIGHT: Self = Self { axes: [0., 1.] };
}

impl Vec3 {
    pub const ZERO: Self = Self { axes: [0., 0., 0.] };
    pub const UP: Self = Self { axes: [0., 0., 1.] };
    pub const DOWN: Self = Self {
        axes: [0., 0., -1.],
    };
    pub const RIGHT_X: Self = Self { axes: [1., 0., 0.] };
    pub const LEFT_X: Self = Self {
        axes: [-1., 0., 0.],
    };
    pub const RIGHT_Y: Self = Self { axes: [0., 1., 0.] };
    pub const LEFT_Y: Self = Self {
        axes: [0., -1., 0.],
    };

    pub const fn homog(self: Self, w: Scalar) -> Vec4 {
        Vec4 {
            axes: [self.x(), self.y(), self.z(), w],
        }
    }

    pub fn xy(self: Self) -> Vec2 {
        Vec2 {
            axes: [self.x(), self.y()],
        }
    }
}

impl Vec4 {
    pub fn xyz(self) -> Vec3 {
        Vec3 {
            axes: [self.x(), self.y(), self.z()],
        }
    }
}
