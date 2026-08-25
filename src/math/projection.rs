use crate::math::{matrix::Mat4, scalar::Scalar};

/// CAMERA FORWARD IS `(0, 0, 1)`!!!!

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
