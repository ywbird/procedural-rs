use raylib::prelude::*;
use raylib::core::math::Vector2;
use raylib::consts::KeyboardKey;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Joint {
    pos: Vector2,
    dir: f32,
    length: f32,
}

impl Joint {
    fn new(x: f32, y: f32, dir: f32, length: f32) -> Self {
        Self {
            pos: Vector2::new(x,y),
            dir,
            length
        }
    }
}

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Hello, World")
        .vsync()
        .build();

    let mut debug = false;
    let mut render = true;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            debug = !debug;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            render = !render;
        }
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_text("Physics", 12, 12, 20, Color::BLACK);
        if debug { d.draw_text("DEBUG: ON", 12, 32, 20, Color::RED); }
        if render { d.draw_text("RENDER: ON", 12, 52, 20, Color::RED); }
    }
}
