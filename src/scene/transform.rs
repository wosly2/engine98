use std::any::Any;

use crate::math::{
    matrix::{Mat4, Order},
    vector::Vec3,
};

pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    order: Order,
    bases: Mat4,
}

pub struct Transformed<T: Any> {
    pub value: T,
    pub transform: Transform,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, order: Order) -> Self {
        Self {
            rotation,
            position,
            order,
            bases: Mat4::get_id(),
        }
    }

    pub fn position(&self) -> &Vec3 {
        &self.position
    }

    pub fn rotation(&self) -> &Vec3 {
        &self.rotation
    }

    pub fn update_rotation<F>(&mut self, mut f: F)
    where
        F: FnMut(Vec3) -> Vec3,
    {
        self.rotation = f(self.rotation);
    }

    pub fn update_position<F>(&mut self, mut f: F)
    where
        F: FnMut(Vec3) -> Vec3,
    {
        self.position = f(self.position);
    }

    pub fn update_order(&mut self, order: Order) {
        self.order = order;
    }

    pub fn update_bases(&mut self) {
        self.bases = Mat4::rotate(*self.rotation(), self.order)
    }
}
