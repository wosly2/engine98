#![allow(dead_code)]

mod graphics;
mod image;
mod math;

use crate::{
    image::Image,
    math::{
        matrix::Mat4,
        projection::perspective,
        shape::{Line2D, Triangle2D},
        vector::{Vec2, Vec3, Vec4, Vecn},
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
        (0, 1, 3),
        (0, 3, 2),
        (4, 5, 7),
        (4, 7, 6),
        (0, 1, 5),
        (0, 5, 4),
        (2, 3, 7),
        (2, 7, 6),
        (0, 2, 6),
        (0, 6, 4),
        (1, 3, 7),
        (1, 7, 5),
    ];

    let fov = PI / 3.;
    let scale = WIDTH as f64 / 2.;

    let center = Vec2::new([WIDTH as f64 / 2., HEIGHT as f64 / 2.]);

    let mut rot = Vec3::new([PI, PI / 2., PI / 4.]);
    let trans = Vec3::new([0., 0., -3.]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer = buffer.fill(0);

        _ = graphics::shape::raster_over_triangle_area_by_edges(
            Triangle2D::new(
                Vec2::new([20., 100.]),
                Vec2::new([90., 50.]),
                Vec2::new([16., 30.]),
            ),
            |x, y| {_ = buffer.set(x, y, 0xBB0000);},
        );

        rot = rot.map(|ax| (ax + PI / 100.) % (2. * PI));

        let matrix = perspective(fov)
            * Mat4::translate(trans)
            * Mat4::rotate_x(rot.x())
            * Mat4::rotate_y(rot.y())
            * Mat4::rotate_z(rot.x());

        let cube_vertices_projected: Vec<Vec4> = cube_vertices
            .iter()
            .map(|(x, y, z)| (matrix * Vec4::new([*x, *y, *z, 1.])).cartesian())
            .collect();

        for tri in cube_triangles {
            let p0 = cube_vertices_projected[tri.0].xyz().xy() * scale + center;
            let p1 = cube_vertices_projected[tri.1].xyz().xy() * scale + center;
            let p2 = cube_vertices_projected[tri.2].xyz().xy() * scale + center;

            let tri = Triangle2D::new(p0, p1, p2);

            buffer.draw_triangle_outline(tri, 0xFFFFFF);
        }

        window
            .update_with_buffer(&buffer.inner, WIDTH, HEIGHT)
            .unwrap();
    }
}
