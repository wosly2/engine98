use std::ops::Mul;

use crate::{
    graphics::shape::raster_over_triangle_area_by_edges,
    math::{
        matrix::Mat4,
        shape::Triangle2D,
        vector::{Vec2, Vec3, Vec4},
    },
};

pub struct ConstModel {
    pub mesh: &'static [[(usize, usize, usize); 3]],
    pub points: &'static [Vec4],
    pub normals: &'static [Vec4],
    pub uvs: &'static [Vec2],
}

#[derive(Clone)]
pub struct Model {
    pub mesh: Vec<[(usize, usize, usize); 3]>,
    pub points: Vec<Vec4>,
    pub normals: Vec<Vec4>,
    pub uvs: Vec<Vec2>,
}

pub struct TriangleIterator<'a> {
    model: &'a Model,
    triangle_index: usize,
}

impl Model {
    pub fn make_cartesian(&mut self) {
        self.points = self
            .points
            .iter()
            .map(|vertex| vertex.cartesian())
            .collect();
    }

    pub fn projected(self, matrix: &Mat4) -> Self {
        let mut projected = *matrix * self;
        projected.make_cartesian();
        return projected;
    }

    pub fn raster_over_triangles<Ft, Fp>(&self, mut triangle_fun: Ft, mut pixel_fun: Fp)
    where
        Ft: FnMut(&(Vec4, Vec4, Vec4), &Triangle2D),
        Fp: FnMut(i64, i64),
    {
        for triangle_4d in TriangleIterator::from(self) {
            let triangle_2d = Triangle2D {
                a: triangle_4d.0.xyz().xy(),
                b: triangle_4d.1.xyz().xy(),
                c: triangle_4d.2.xyz().xy(),
            };

            triangle_fun(&triangle_4d, &triangle_2d);

            // FIXME ! what do i do with this result? \/ ?

            _ = raster_over_triangle_area_by_edges(triangle_2d, |x, y| pixel_fun(x, y));
        }
    }
}

impl<'a> From<&'a Model> for TriangleIterator<'a> {
    fn from(value: &'a Model) -> Self {
        TriangleIterator {
            model: value,
            triangle_index: 0,
        }
    }
}

impl<'a> Iterator for TriangleIterator<'a> {
    type Item = (Vec4, Vec4, Vec4);

    fn next(&mut self) -> Option<Self::Item> {
        self.triangle_index += 1;

        if self.triangle_index - 1 < self.model.mesh.len() {
            let tup = self.model.mesh[self.triangle_index - 1];
            return Some((
                self.model.points[tup[0].0],
                self.model.points[tup[1].0],
                self.model.points[tup[2].0],
            ));
        } else {
            return None;
        }
    }
}

impl Mul<Vec<Vec4>> for Mat4 {
    type Output = Vec<Vec4>;

    fn mul(self, rhs: Vec<Vec4>) -> Self::Output {
        rhs.iter().map(|v| self * *v).collect()
    }
}

impl Mul<Homog> for Mat4 {
    type Output = Vec<Vec4>;

    fn mul(self, rhs: Homog) -> Self::Output {
        rhs.0.iter().map(|v| self * (*v).homog(rhs.1)).collect()
    }
}

pub struct Homog(Vec<Vec3>, f64);

impl Mul<Model> for Mat4 {
    type Output = Model;

    fn mul(self, rhs: Model) -> Self::Output {
        Model {
            mesh: rhs.mesh,
            points: self * rhs.points,
            normals: self * rhs.normals,
            uvs: rhs.uvs,
        }
    }
}

impl From<ConstModel> for Model {
    fn from(value: ConstModel) -> Self {
        Self {
            mesh: value.mesh.into(),
            points: value.points.into(),
            normals: value.normals.into(),
            uvs: value.uvs.into(),
        }
    }
}

impl ConstModel {
    pub const CUBE: Self = Self {
        mesh: &[
            [(6, 6, 0), (4, 4, 0), (0, 0, 0)],
            [(6, 6, 0), (0, 0, 0), (2, 2, 0)],
            [(7, 7, 0), (3, 3, 0), (1, 1, 0)],
            [(7, 7, 0), (1, 1, 0), (5, 5, 0)],
            [(3, 3, 0), (2, 2, 0), (0, 0, 0)],
            [(3, 3, 0), (0, 0, 0), (1, 1, 0)],
            [(1, 1, 0), (0, 0, 0), (4, 4, 0)],
            [(1, 1, 0), (4, 4, 0), (5, 5, 0)],
            [(5, 5, 0), (4, 4, 0), (6, 6, 0)],
            [(5, 5, 0), (6, 6, 0), (7, 7, 0)],
            [(7, 7, 0), (6, 6, 0), (2, 2, 0)],
            [(7, 7, 0), (2, 2, 0), (3, 3, 0)],
        ],
        points: &[
            Vec3::new([1., 1., 1.]).homog(1.),
            Vec3::new([1., 1., -1.]).homog(1.),
            Vec3::new([1., -1., 1.]).homog(1.),
            Vec3::new([1., -1., -1.]).homog(1.),
            Vec3::new([-1., 1., 1.]).homog(1.),
            Vec3::new([-1., 1., -1.]).homog(1.),
            Vec3::new([-1., -1., 1.]).homog(1.),
            Vec3::new([-1., -1., -1.]).homog(1.),
        ],
        normals: &[
            Vec3::new([1., 1., 1.]).homog(0.),
            Vec3::new([1., 1., -1.]).homog(0.),
            Vec3::new([1., -1., 1.]).homog(0.),
            Vec3::new([1., -1., -1.]).homog(0.),
            Vec3::new([-1., 1., 1.]).homog(0.),
            Vec3::new([-1., 1., -1.]).homog(0.),
            Vec3::new([-1., -1., 1.]).homog(0.),
            Vec3::new([-1., -1., -1.]).homog(0.),
        ],
        uvs: &[Vec2::ZERO],
    };
}
