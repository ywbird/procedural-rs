use raylib::prelude::*;
use raylib::core::math::Vector2;
use raylib::consts::PI;
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

    let source = Arc::new(Mutex::new(Joint::new(320.0, 240.0, 0.0, 0.0)));

    let mut linkage: Vec<(Arc<Mutex<Joint>>, f32)> = Vec::new();

    let mut debug = false;

    for i in 1..=10 {
        let j1 = Arc::new(Mutex::new(Joint::new(320.0 + 40.0*i as f32, 240.0, 0.0, 20.0)));

        linkage.push((j1.clone(), 20.0));
    }

    while !rl.window_should_close() {
        {
            {
                let linkage = linkage.as_mut_slice();
                let source_ref = source.clone();
                for i in 0..linkage.len() {
                    let (t, l) = if i == 0 {
                        (Vector2::new(rl.get_mouse_x() as f32, rl.get_mouse_y() as f32), 0.0)
                    } else {
                        (linkage[i-1].0.lock().unwrap().pos, linkage[i-1].1)
                    };
                
                    let mut j = linkage[i].0.lock().unwrap();
            
                    j.pos = t + (j.pos - t).normalized() * l;
                }
                let max = linkage.len();
                for i in 0..linkage.len() {
                    let (t, l) = if i == 0 {
                        (source_ref.lock().unwrap(), 0.0)
                    } else {
                        (linkage[max-i].0.lock().unwrap(), linkage[max-i].1)
                    };
                
                    let mut j = linkage[max-i-1].0.lock().unwrap();

                    j.dir = (t.pos - j.pos).angle_to(Vector2::zero());
            
                    j.pos = t.pos + (j.pos - t.pos).normalized() * l;
                }
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            debug = !debug;
        }
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_text("FABRIK", 12, 12, 20, Color::BLACK);
        if debug { d.draw_text("DEBUG: ON", 12, 32, 20, Color::RED); }

        let mut line: Vec<Vector2> = linkage.iter().map(|a| a.0.lock().unwrap().pos).collect();
        line.push(source.lock().unwrap().pos);
        line.insert(0, line[0]);
        if debug { d.draw_spline_linear(&line, 4.0, Color::RED); }
        d.draw_spline_basis(&line, 2.0, Color::BLACK);
    }
}
