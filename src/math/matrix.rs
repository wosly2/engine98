use crate::math::{
    scalar::Scalar,
    vector::{Vec2, Vec3, Vec4, Vecn},
};
use new_macro::New;
use paste::paste;
use std::{array, ops::Mul};

#[derive(Clone, Copy, PartialEq, New)]
pub struct Matn<const N: usize> {
    pub bases: [Vecn<N>; N],
}

impl<const N: usize> Matn<N> {
    /// Builds the matrix from the representation in the form
    /// ```
    /// [[IX, JX, KX, WX ...N],
    ///  [IY, JY, KY, WY ...N],
    ///  [IZ, JZ, KZ, WZ ...N],
    ///  [IW, JW, KW, WW ...N]...N]
    /// ```
    /// for easier implementation and readability.
    pub fn from(repr: [[Scalar; N]; N]) -> Self {
        Self {
            bases: array::from_fn(|col| Vecn::new(array::from_fn(|row| repr[row][col]))),
        }
    }

    /// Performs the matrix composition `AB` where
    /// `self` represents `B` and `other` represents `A`.
    pub fn compose_by(self, other: Self) -> Self {
        Self {
            bases: array::from_fn(|i| other * self.bases[i]),
        }
    }

    /// Performs the matrix transformation of the `Vecn<N>` `vector`:
    /// `Mv` where `self` represents `M` and `vector` represents `v`.
    pub fn transform(self: Self, vector: Vecn<N>) -> Vecn<N> {
        let scaled: [Vecn<N>; N] = array::from_fn(|i| self.bases[i] * vector.axes[i]);
        scaled.iter().fold(Vecn::EMPTY, |acc, v| acc + *v)
    }

    pub fn get_id() -> Self {
        Self {
            bases: array::from_fn(|i| {
                let mut id = Vecn::splat(0.);
                id.axes[i] = 1.;
                id
            }),
        }
    }
}

impl<const N: usize> Default for Matn<N> {
    fn default() -> Self {
        Self {
            bases: [Vecn::default(); N],
        }
    }
}

// overloading

impl<const N: usize> Mul<Matn<N>> for Matn<N> {
    type Output = Matn<N>;

    fn mul(self, rhs: Matn<N>) -> Self::Output {
        rhs.compose_by(self)
    }
}

impl<const N: usize> Mul<Vecn<N>> for Matn<N> {
    type Output = Vecn<N>;

    fn mul(self, rhs: Vecn<N>) -> Self::Output {
        self.transform(rhs)
    }
}

macro_rules! matrix_accessors {
    ($mat_type:ty, $vec_type:ty, $($name:ident: $index:tt),+) => {
        impl $mat_type {
            $(
                pub const fn $name(self) -> $vec_type {
                    self.bases[$index]
                }

                paste! {
                    pub const fn [<set_ $name>](&mut self, vector: $vec_type) {
                        self.bases[$index] = vector;
                    }
                }
            )+
        }
    };
}

matrix_accessors!(Mat2, Vec2, i: 0, j: 1);
matrix_accessors!(Mat3, Vec3, i: 0, j: 1, k: 2);
matrix_accessors!(Mat4, Vec4, i: 0, j: 1, k: 2, w: 3);

// Type Aliases!

pub type Mat2 = Matn<2>;
pub type Mat3 = Matn<3>;
pub type Mat4 = Matn<4>;

impl Mat4 {
    pub fn translate(t: Vec3) -> Mat4 {
        Mat4::from([
            [1., 0., 0., t.x()],
            [0., 1., 0., t.y()],
            [0., 0., 1., t.z()],
            [0., 0., 0., 1.0],
        ])
    }

    pub fn rotate(rot: Vec3, order: Order) -> Mat4 {
        let transforms = order.sort([
            Mat4::rotate_x(rot.x()),
            Mat4::rotate_y(rot.y()),
            Mat4::rotate_z(rot.z()),
        ]);

        transforms[2] * transforms[1] * transforms[0]
    }

    pub fn rotate_x(alpha: Scalar) -> Mat4 {
        Mat4::from([
            [1., 0., 0., 0.],
            [0., alpha.cos(), -alpha.sin(), 0.],
            [0., alpha.sin(), alpha.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotate_y(beta: Scalar) -> Mat4 {
        Mat4::from([
            [beta.cos(), 0., beta.sin(), 0.],
            [0., 1., 0., 0.],
            [-beta.sin(), 0., beta.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotate_z(gamma: Scalar) -> Mat4 {
        Mat4::from([
            [gamma.cos(), -gamma.sin(), 0., 0.],
            [gamma.sin(), gamma.cos(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }
}

// XYZ Ordering
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Order {
    XYZ,
    XZY,
    YXZ,
    YZX,
    ZXY,
    ZYX,
}

impl Order {
    fn sort<T: Copy>(self, items: [T; 3]) -> [T; 3] {
        match self {
            Order::XYZ => [items[0], items[1], items[2]],
            Order::XZY => [items[0], items[2], items[1]],
            Order::YXZ => [items[1], items[0], items[2]],
            Order::YZX => [items[1], items[2], items[0]],
            Order::ZXY => [items[2], items[0], items[1]],
            Order::ZYX => [items[2], items[1], items[0]],
        }
    }
}
