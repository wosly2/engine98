#![allow(dead_code)]

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

mod graphics;
mod math;

use graphics::{Buffer, ShapeDrawOptions};
use math::{Line, Shape, Vec2, Vec3};

use crate::math::{Rect, Triangle};

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

    let cart_point = Vec3::DOWN;

    println!("3d point: {}", cart_point);

    let projected_point = cart_point.cartesian().xy();

    println!("projected point: {}", projected_point);

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

    buffer = buffer
        .draw_shape(
            Shape::Line(Line::new(Vec2::new([10., 20.]), Vec2::new([300., 100.]))),
            ShapeDrawOptions::default(),
        )
        .draw_triangle_outline(
            Triangle::new(
                Vec2::new([10., 30.]),
                Vec2::new([15., 80.]),
                Vec2::new([90., 12.]),
            ),
            0x00FF00,
        )
        .draw_line(
            Line::new(Vec2::new([10., 20.]), Vec2::new([20., 100.])),
            0xFF0000,
        )
        .draw_rect_outline(
            Rect::new(
                projected_point,
                projected_point + (Vec2::RIGHT + Vec2::DOWN) * 10.,
            ),
            0xFF0FF,
        );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer.inner, WIDTH, HEIGHT)
            .unwrap();
    }
}
