use crate::math::{matrix::Mat4, scalar::Scalar, shape::Triangle3D, vector::Vec3};

pub fn perspective(fov: Scalar) -> Mat4 {
    // the fov applies to the size of the whole viewing angle,
    // but the tangent operation produces a value that scales
    // each half of the frustum.
    let f = (fov / 2.).tan();

    Mat4::from([
        [f, 0., 0., 0.],
        [0., f, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 1., 0.],
    ])
}

impl Triangle3D {
    pub fn z_values(&self) -> Vec3 {
        Vec3 {
            axes: [self.v0.z(), self.v1.z(), self.v2.z()],
        }
    }
}

pub fn bary_as_vec(bary: (Scalar, Scalar, Scalar)) -> Vec3 {
    Vec3 {
        axes: [bary.0, bary.1, bary.2],
    }
}
