use crate::math::{Mat4, Scalar};

pub fn perspective(fov: Scalar) -> Mat4 {
    let f = (fov / 2.).tan();

    Mat4::from([
        [f, 0., 0., 0.],
        [0., f, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 1., 0.],
    ])
}
