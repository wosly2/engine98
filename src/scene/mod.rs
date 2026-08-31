use std::any::Any;

pub mod node;

use crate::math::vector::Vecn;

pub struct Transform<const N: usize> {
    rotation: Vecn<N>,
    position: Vecn<N>,
}

pub struct Transformed<const N: usize, T: Any> {
    pub value: T,
    pub transform: Transform<N>,
}

impl<const N: usize> Transform<N> {
    pub fn new(rotation: Vecn<N>, position: Vecn<N>) -> Self {
        Self { rotation, position }
    }

    pub fn rotation(&self) -> &Vecn<N> {
        &self.rotation
    }

    pub fn position(&self) -> &Vecn<N> {
        &self.position
    }
}

pub trait RelativePosition {
    type Position;

    fn origin(&self) -> Self::Position;

    fn position(&self) -> Self::Position;
    fn global_position(&self) -> Self::Position;
    fn rotation(&self) -> Self::Position;
    fn global_rotation(&self) -> Self::Position;

    fn update_position<F: FnMut(Self::Position)>(&self, f: F);
    fn update_global_position<F: FnMut(Self::Position)>(&self, f: F);
    fn update_rotation<F: FnMut(Self::Position)>(&self, f: F);
    fn update_global_rotation<F: FnMut(Self::Position)>(&self, f: F);
}
