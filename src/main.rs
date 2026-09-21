#![allow(dead_code)]

mod gl;
mod math;
mod scene;
#[macro_use]
mod util;
use minifb::{Key, Window, WindowOptions};
use std::{f64::consts::PI, time};

use crate::{
    gl::image::{Image, color::Color, load_image},
    gl::{
        math::{bary_as_vec, perspective},
        model::{ConstModel, Model, TriangleIterator},
    },
    math::{
        matrix::Mat4,
        shape::{Triangle2D, Triangle3D},
        vector::{Vec2, Vec3},
    },
};

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;

fn main() {
    let mut color_buffer = Image::<Color>::new(WIDTH, HEIGHT, 0.into());
    let mut depth_buffer = Image::<f64>::new(WIDTH, HEIGHT, -f64::INFINITY);

    let test_image: Image<Color> = load_image("assets/textures/uv_checker.jpg".into())
        .expect("failed to load image")
        .scaled(100, 100, Image::sampled_nearest)
        .expect("failed to scale");

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

    // window.set_target_fps(60);

    let fov = PI / 3.;
    let scale = WIDTH as f64 / 2.;

    let center: Vec2 = [WIDTH as f64 / 2., HEIGHT as f64 / 2.].into();

    let mut rot: Vec3 = [PI, PI / 3., PI / 8.].into();
    let trans: Vec3 = [0., 0., -3.].into();

    let cube: Model = ConstModel::CUBE.into();

    let mut frame_time: f64 = 0.;
    let mut fps = 0.;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_time_start = time::Instant::now();

        color_buffer.fill(Color::shade(1.));
        depth_buffer.fill(f64::INFINITY);

        rot = rot.map(|ax| (ax + PI * frame_time / 8.) % (2. * PI));
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

        for (tpoints, _tnorms, tuvs) in TriangleIterator::from(&cube_projected) {
            let tri_proj = Triangle3D::new(tpoints.0.xyz(), tpoints.1.xyz(), tpoints.2.xyz());

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

            let inv_z = tri_proj.z_values().map(|z| 1. / z);

            _ = gl::shape::raster_over_triangle_area_by_edges(tri_screen, |x, y| {
                if color_buffer.is_on_image(x, y) {
                    let bary = tri_screen.barycentric_coord_double_area(
                        Vec2::new([x as f64, y as f64]),
                        double_area,
                    );

                    // not inside triangle
                    if bary.0 < 0. || bary.1 < 0. || bary.2 < 0. {
                        return;
                    }

                    // // this code doesn't work because it has no respect to depth
                    // let uv = (uvs[0] * bary.0) + (uvs[1] * bary.1) + (uvs[2] * bary.2);

                    // -=-=-=-
                    // i copied this code and don't understand it well!
                    // i need to learn how the math behind it works some more

                    let uv_over_z = (tuvs.0 * inv_z.x() * bary.0)
                        + (tuvs.1 * inv_z.y() * bary.1)
                        + (tuvs.2 * inv_z.z() * bary.2);

                    let inv_z_interp = bary.0 * inv_z.x() + bary.1 * inv_z.y() + bary.2 * inv_z.z();

                    let uv = uv_over_z * (1. / inv_z_interp);

                    // -=-=-=- end copied code

                    // println!(
                    //     "b1->{0}\nb1->{1}\nb1->{2}\nsm->{3}\nuv->{uv}",
                    //     bary.0,
                    //     bary.1,
                    //     bary.2,
                    //     bary.0 + bary.1 + bary.2
                    // );

                    // assert!((bary.0 + bary.1 + bary.2 - 1.).abs() < 0.1);

                    let depth = 1. / bary_as_vec(bary).dot(inv_z);

                    if let Ok(greater) = depth_buffer.set_if_less(x, y, depth)
                        && greater
                    {
                        // Color::vec_norm(bary_as_vec(bary))
                        _ = color_buffer.set(
                            x,
                            y,
                            test_image
                                .sampled_nearest(uv.x().abs(), uv.y().abs())
                                .expect("failed to sample"),
                        );
                    }
                }
            });

            // color_buffer.draw_triangle_outline(tri_screen, Color(0xFFFFFF));

            n_triangles += 1;
        }

        test_image.blit_to(&mut color_buffer, 50, 50);

        if false {
            print!(
                "->FRAME\n{0}FPS: {1:<5.2}\n{0}DLT: {2:<5.8}\n{0}TRI: {3:<5}\n\n",
                "        ", fps, frame_time, n_triangles
            );
        }

        window
            .update_with_buffer(Color::as_u32_slice(&color_buffer.inner), WIDTH, HEIGHT)
            .unwrap();

        fps = 1. / frame_time;
        frame_time = frame_time_start.elapsed().as_secs_f64();
    }
}
