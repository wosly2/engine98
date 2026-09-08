#![allow(dead_code)]

mod graphics;
mod math;
mod scene;
mod util;

use crate::{
    graphics::image::{Image, color::Color},
    graphics::{
        math::{bary_as_vec, perspective},
        model::{ConstModel, Model, TriangleIterator},
    },
    math::{
        matrix::Mat4,
        shape::{Triangle2D, Triangle3D},
        vector::{Vec2, Vec3},
    },
};

use minifb::{Key, Window, WindowOptions};

use std::{f64::consts::PI, time};

const WIDTH: usize = 16 * 50;
const HEIGHT: usize = 9 * 50;

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

    let fov = PI / 3.;
    let scale = WIDTH as f64 / 2.;

    let center: Vec2 = [WIDTH as f64 / 2., HEIGHT as f64 / 2.].into();

    let mut rot: Vec3 = [PI, PI / 3., PI / 8.].into();
    let trans: Vec3 = [0., 0., -3.].into();

    let cube: Model = ConstModel::CUBE.into();

    let mut frame_time: time::Duration = time::Duration::ZERO;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_time_start = time::Instant::now();

        color_buffer.fill(Color(0));
        depth_buffer.fill(f64::INFINITY);

        rot = rot.map(|ax| (ax + PI / 100.) % (2. * PI));
        // rot.set_z((rot.z() + PI / 100.) % (2. * PI));

        let matrix = perspective(fov)
            * Mat4::translate(trans)
            * Mat4::rotate_x(rot.x())
            * Mat4::rotate_y(rot.y())
            * Mat4::rotate_z(rot.z());

        // let cube_vertices_projected: Vec<Vec4> = cube_vertices
        //     .iter()
        //     .map(|vertex| (matrix * Vec3::new(*vertex).homog(1.)).cartesian())
        //     .collect();

        let mut cube_projected = matrix * cube.clone();
        cube_projected.make_cartesian();

        let mut n_triangles = 0;

        for triangle in TriangleIterator::from(&cube_projected) {
            let tri_proj = Triangle3D::new(triangle.0.xyz(), triangle.1.xyz(), triangle.2.xyz());

            let tri_screen = Triangle2D::new(
                tri_proj.v0.xy() * scale + center,
                tri_proj.v1.xy() * scale + center,
                tri_proj.v2.xy() * scale + center,
            );

            let double_area = tri_screen.double_area();

            if double_area > 0. {
                // does not work when winding is wrong
                continue;
            }

            let depths = tri_proj.z_values().map(|z| 1. / z);

            _ = graphics::shape::raster_over_triangle_area_by_edges(tri_screen, |x, y| {
                if color_buffer.is_on_image(x, y) {
                    let bary = tri_screen.barycentric_coord_double_area(
                        Vec2::new([x as f64, y as f64]),
                        double_area,
                    );

                    let depth = 1. / bary_as_vec(bary).dot(depths);

                    if let Ok(greater) = depth_buffer.set_if_less(x, y, depth)
                        && greater
                    {
                        _ = color_buffer.set(x, y, Color::vec_norm(bary_as_vec(bary)));
                    }
                }
            });

            color_buffer.draw_triangle_outline(tri_screen, Color(0xFFFFFF));

            n_triangles += 1;
        }

        println!(
            "->FRAME
        FPS:       {:<5.2}
        TRIANGLES: {:<5}\n",
            1. / (frame_time.as_millis() as f64 / 1000.),
            n_triangles
        );

        window
            .update_with_buffer(Color::as_u32_slice(&color_buffer.inner), WIDTH, HEIGHT)
            .unwrap();

        frame_time = frame_time_start.elapsed();
    }
}
