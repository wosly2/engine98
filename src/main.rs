#![allow(dead_code)]

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

mod graphics;
mod math;

use graphics::{Buffer, ShapeDrawOptions};
use math::{Line, Shape, Vec2};

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

    buffer = buffer
        .shape(
            Shape::Line(Line::new(Vec2::new(10., 20.), Vec2::new(300., 100.))),
            ShapeDrawOptions::default(),
        )
        .triangle(
            Triangle::new(
                Vec2::new(10., 30.),
                Vec2::new(15., 80.),
                Vec2::new(90., 12.),
            ),
            0x00FF00,
        )
        .line(
            Line::new(Vec2::new(10., 20.), Vec2::new(20., 100.)),
            0xFF0000,
        )
        .rect(
            Rect::new(Vec2::new(300., 200.), Vec2::new(200., 160.)),
            0xFF0FF,
        );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer.inner, WIDTH, HEIGHT)
            .unwrap();
    }
}
