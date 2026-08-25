#![allow(dead_code)]

mod graphics;
mod image;
mod math;
mod util;

use crate::{
    graphics::math::perspective,
    image::Image,
    math::{
        matrix::Mat4,
        shape::Triangle2D,
        vector::{Vec2, Vec3, Vec4},
    },
};

use minifb::{Key, Window, WindowOptions};

use std::f64::consts::PI;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn main() {
    let mut buffer = Image::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "test",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let cube_vertices = [
        (1., 1., 1.),
        (1., 1., -1.),
        (1., -1., 1.),
        (1., -1., -1.),
        (-1., 1., 1.),
        (-1., 1., -1.),
        (-1., -1., 1.),
        (-1., -1., -1.),
    ];

    let cube_triangles = [
        (0, 1, 3, 0x0000AA),
        (0, 3, 2, 0x00AA00),
        (4, 5, 7, 0x00AAAA),
        (4, 7, 6, 0xAA0000),
        (0, 1, 5, 0xAA00AA),
        (0, 5, 4, 0xAAAA00),
        (2, 3, 7, 0x0000BB),
        (2, 7, 6, 0x00BB00),
        (0, 2, 6, 0x00BBBB),
        (0, 6, 4, 0xBB0000),
        (1, 3, 7, 0xBB00BB),
        (1, 7, 5, 0xBBBB00),
    ];

    let fov = PI / 3.;
    let scale = WIDTH as f64 / 2.;

    let center = Vec2::new([WIDTH as f64 / 2., HEIGHT as f64 / 2.]);

    let mut rot = Vec3::new([PI, PI / 3., PI / 4.]);
    let trans = Vec3::new([0., 0., -3.]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer = buffer.fill(0);

        rot = rot.map(|ax| (ax + PI / 100.) % (2. * PI));
        // rot.set_y((rot.y() + PI / 300.) % (2. * PI));

        let matrix = perspective(fov)
            * Mat4::translate(trans)
            * Mat4::rotate_x(rot.x())
            * Mat4::rotate_y(rot.y())
            * Mat4::rotate_z(rot.x());

        let cube_vertices_projected: Vec<Vec4> = cube_vertices
            .iter()
            .map(|(x, y, z)| (matrix * Vec4::new([*x, *y, *z, 1.])).cartesian())
            .collect();

        for triangle in cube_triangles {
            let p0 = cube_vertices_projected[triangle.0].xyz().xy() * scale + center;
            let p1 = cube_vertices_projected[triangle.1].xyz().xy() * scale + center;
            let p2 = cube_vertices_projected[triangle.2].xyz().xy() * scale + center;

            let tri = Triangle2D::new(p0, p1, p2);

            _ = graphics::shape::raster_over_triangle_area_by_edges(tri, |x, y| {
                _ = buffer.set(x, y, triangle.3);
            });

            buffer.draw_triangle_outline(tri, 0xFFFFFF - triangle.3);
        }

        window
            .update_with_buffer(&buffer.inner, WIDTH, HEIGHT)
            .unwrap();
    }
}
