#![allow(dead_code)]

mod graphics;
mod image;
mod math;
mod util;

use crate::{
    graphics::math::{bary_as_vec, perspective},
    image::{Image, color::Color},
    math::{
        matrix::Mat4,
        shape::{Triangle2D, Triangle3D},
        vector::{Vec2, Vec3, Vec4},
    },
};

use minifb::{Key, Window, WindowOptions};

use std::f64::consts::PI;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn main() {
    let mut color_buffer = Image::<Color>::new(WIDTH, HEIGHT, 0.into());
    let mut depth_buffer = Image::<f64>::new(WIDTH, HEIGHT, -f64::INFINITY);

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
        // top
        (6, 4, 0),
        (6, 0, 2),
        // bottom (reverse winding)
        (7, 3, 1),
        (7, 1, 5),
        //
        (3, 2, 0),
        (3, 0, 1),
        //
        (1, 0, 4),
        (1, 4, 5),
        //
        (5, 4, 6),
        (5, 6, 7),
        //
        (7, 6, 2),
        (7, 2, 3),
    ];

    let fov = PI / 3.;
    let scale = WIDTH as f64 / 2.;

    let center = Vec2::new([WIDTH as f64 / 2., HEIGHT as f64 / 2.]);

    let mut rot = Vec3::new([PI, PI / 3., PI / 8.]);
    let trans = Vec3::new([0., 0., -3.]);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        color_buffer.fill(Color(0));
        depth_buffer.fill(f64::INFINITY);

        rot = rot.map(|ax| (ax + PI / 100.) % (2. * PI));
        // rot.set_z((rot.z() + PI / 100.) % (2. * PI));

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
            let tri_proj = Triangle2D::new(
                cube_vertices_projected[triangle.0].xyz().xy() * scale + center,
                cube_vertices_projected[triangle.1].xyz().xy() * scale + center,
                cube_vertices_projected[triangle.2].xyz().xy() * scale + center,
            );
            let tri_world = Triangle3D::new(
                cube_vertices_projected[triangle.0].xyz(),
                cube_vertices_projected[triangle.1].xyz(),
                cube_vertices_projected[triangle.2].xyz(),
            );

            let double_area = tri_proj.double_area();

            if double_area > 0. {
                // does not work when winding is wrong
                continue;
            }

            let depths = tri_world.z_values().map(|z| 1. / z);

            _ = graphics::shape::raster_over_triangle_area_by_edges(tri_proj, |x, y| {
                let bary = tri_proj
                    .barycentric_coord_double_area(Vec2::new([x as f64, y as f64]), double_area);

                let depth = 1. / bary_as_vec(bary).dot(depths);

                if let Ok(greater) = depth_buffer.set_if_less(x, y, depth)
                    && greater
                {
                    _ = color_buffer.set(x, y, Color::vec_norm(bary_as_vec(bary)));
                }
            });

            //color_buffer.draw_triangle_outline(tri_proj, Color(0xFFFFFF));
        }

        window
            .update_with_buffer(Color::as_u32_slice(&color_buffer.inner), WIDTH, HEIGHT)
            .unwrap();
    }
}
