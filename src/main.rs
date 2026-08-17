#![allow(dead_code)]

use std::f64::consts::PI;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

mod graphics;
mod math;

use crate::{
    graphics::{buffer::Buffer, projection::perspective},
    math::{Line, Mat4, Vec2, Vec3, Vec4},
};

fn main() {
    let mut buffer = Buffer::new(WIDTH, HEIGHT);

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

    let cube_quads = [
        (0, 1, 3, 2),
        (4, 5, 7, 6),
        (0, 1, 5, 4),
        (2, 3, 7, 6),
        (0, 2, 6, 4),
        (1, 3, 7, 5),
    ];

    let fov = PI / 3.;
    let scale = WIDTH as f64 / 2.;

    let center = Vec2::new([WIDTH as f64 / 2., HEIGHT as f64 / 2.]);

    let mut rot = Vec3::new([PI, PI / 2., PI / 4.]);
    let trans = Vec3::new([0., 0., -3.]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer = buffer.fill(0);

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

        for cube in cube_quads {
            let p0 = cube_vertices_projected[cube.0].xyz().xy() * scale + center;
            let p1 = cube_vertices_projected[cube.1].xyz().xy() * scale + center;
            let p2 = cube_vertices_projected[cube.2].xyz().xy() * scale + center;
            let p3 = cube_vertices_projected[cube.3].xyz().xy() * scale + center;
            buffer = buffer
                .draw_line(Line::new(p0, p1), 0xFFFFFF)
                .draw_line(Line::new(p1, p2), 0xFFFFFF)
                .draw_line(Line::new(p2, p3), 0xFFFFFF)
                .draw_line(Line::new(p3, p0), 0xFFFFFF);
        }

        window
            .update_with_buffer(&buffer.clone().inner, WIDTH, HEIGHT)
            .unwrap();
    }
}
