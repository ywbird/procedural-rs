use raylib::prelude::*;
use raylib::core::math::Vector2;
use raylib::consts::PI;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Joint {
    pos: Vector2,
    dir: f32,
    radius: f32,
}

impl Joint {
    fn new(x: f32, y: f32, dir: f32, radius: f32) -> Self {
        Self {
            pos: Vector2::new(x,y),
            dir,
            radius
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_circle_lines(
            self.pos.x as i32,
            self.pos.y as i32,
            self.radius,
            Color::BLACK
        );

        d.draw_line(
            self.pos.x as i32,
            self.pos.y as i32,
            (self.pos.x - self.dir.cos()*self.radius) as i32,
            (self.pos.y - self.dir.sin()*self.radius) as i32,
            Color::BLACK
        );
    }
}

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

const MOUSE_ENABLED: bool = true;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Hello, World")
        .vsync()
        .build();

    let head = Arc::new(Mutex::new(Joint::new(320.0, 240.0, 0.0, 18.0)));

    let mut linkage: Vec<(Arc<Mutex<Joint>>, f32)> =
        vec![
            (head.clone(), 10.0)
        ];

    let segments: Vec<(f32, f32)> = vec![
        (20.0, 15.0),
        (23.0, 16.0),
        (22.0, 17.0),
        (20.0, 18.0),
        (18.0, 20.0),
        (14.0, 20.0),
        (12.0, 20.0),
    ];

    for (i, (segment, dist)) in segments.into_iter().enumerate() {
        let j = Arc::new(Mutex::new(Joint::new(320.0 + 40.0*i as f32, 240.0, 0.0, segment)));

        linkage.push((j.clone(), dist));
    }

    let mut debug = false;

    let head_ref = head.clone();
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        
        {
            let mut head = head_ref.lock().unwrap();
            if !(MOUSE_ENABLED && Vector2::new(rl.get_mouse_x() as f32, rl.get_mouse_y() as f32).distance_to(head.pos) <= 30.0) {
                let mut movement = Vector2::zero();
                if MOUSE_ENABLED {
                    movement = Vector2::new(rl.get_mouse_x() as f32, rl.get_mouse_y() as f32) - head.pos;
                } else {
                    if rl.is_key_down(KeyboardKey::KEY_UP) {
                        movement += Vector2::new(0.0,-1.0);
                    }
                    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
                        movement += Vector2::new(0.0,2.0);
                    }
                    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                        movement += Vector2::new(1.0,0.0);
                    }
                    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                        movement += Vector2::new(-1.0,0.0);
                    }
                }
            
                if movement.length_sqr() != 0.0 {
                    head.dir = movement.normalized().angle_to(Vector2::zero());
                }
                head.pos += movement.normalized()*300.0*dt;
            }
        }
        

        for i in 1..linkage.len() {
            let l = linkage.as_mut_slice();            
            let h = l[i-1].0.lock().unwrap();
            let r = l[i-1].1;
            let mut t = l[i].0.lock().unwrap();
            
            let t_pos = t.pos.clone();
            
            t.dir = (h.pos - t.pos).angle_to(Vector2::zero());
            t.pos = h.pos + (t_pos - h.pos).normalized()*r;
            
            if i == l.len()-1 { continue; }
            
            let ang = (PI as f32 + h.dir - t.dir).rem_euclid(2.0*PI as f32);

            if ang < (PI*11.0/12.0) as f32 {
                // t.pos = h.pos + (t.pos - h.pos).rotated((ang - PI as f32 *5.0/6.0)*20.0*dt);
                t.pos = h.pos + (t.pos - h.pos).rotated(-10.0*dt);
            } else if ang > (PI*13.0/12.0) as f32 {
                // t.pos = h.pos + (t.pos - h.pos).rotated((ang - PI as f32 *7.0/6.0)*20.0*dt);
                t.pos = h.pos + (t.pos - h.pos).rotated(10.0*dt);
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            debug = !debug;
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("SNAKE:  Circle Constrait", 12, 12, 20, Color::BLACK);
        if debug { d.draw_text("DEBUG: ON", 12, 32, 20, Color::RED); }

        let mut line_points_r: Vec<Vector2> = Vec::new();
        let mut line_points_l: Vec<Vector2> = Vec::new();
        for i in 0..linkage.len() {
            let j_a = linkage[i].0.lock().unwrap();

            if debug {
                d.draw_circle_lines(j_a.pos.x as i32, j_a.pos.y as i32, j_a.radius, Color::RED);
            }
            // if i == linkage.len()-1 {
            //     d.draw_circle_lines(j_a.pos.x as i32, j_a.pos.y as i32, j_a.radius, Color::BLACK);
            //     continue;
            // }
            
            // let j_b = linkage[i+1].0.lock().unwrap();
            
            let ldir_a = j_a.dir + PI as f32 / 2.0;
            // let ldir_b = j_b.dir + PI as f32 / 2.0;
            let lp = {
                j_a.pos + Vector2::new(ldir_a.cos()*j_a.radius, ldir_a.sin()*j_a.radius)
            };
            // let Vector2 {x: lbx, y: lby} = {
            //     j_b.pos + Vector2::new(ldir_b.cos()*j_b.radius, ldir_b.sin()*j_b.radius)
            // };

            let rdir_a = j_a.dir - PI as f32 / 2.0;
            // let rdir_b = j_b.dir - PI as f32 / 2.0;
            let rp = {
                j_a.pos + Vector2::new(rdir_a.cos()*j_a.radius, rdir_a.sin()*j_a.radius)
            };
            // let Vector2 {x: rbx, y: rby} = {
            //     j_b.pos + Vector2::new(rdir_b.cos()*j_b.radius, rdir_b.sin()*j_b.radius)
            // };
            
            // d.draw_line(lax as i32, lay as i32, lbx as i32, lby as i32, Color::BLACK);
            // d.draw_line(rax as i32, ray as i32, rbx as i32, rby as i32, Color::BLACK);

            if i == 0 || i == linkage.len()-1 {
                line_points_r.push(rp);
                line_points_l.push(lp);
            }
            line_points_l.push(lp);
            line_points_r.push(rp);
        }

        let head = linkage[0].0.lock().unwrap();
        let dir = head.dir + PI as f32;
        let p1_h = head.pos + Vector2::new((dir - PI as f32 /2.0).cos(),(dir - PI as f32 /2.0).sin()) * head.radius;
        let p2_h = head.pos + Vector2::new((dir - PI as f32 /4.0).cos(),(dir - PI as f32 /4.0).sin()) * head.radius;
        let p3_h = head.pos + Vector2::new( dir.cos(),dir.sin() ) * head.radius;
        let p4_h = head.pos + Vector2::new((dir + PI as f32 /4.0).cos(),(dir + PI as f32 /4.0).sin()) * head.radius;
        let p5_h = head.pos + Vector2::new((dir + PI as f32 /2.0).cos(),(dir + PI as f32 /2.0).sin()) * head.radius;

        let tail = linkage[linkage.len()-1].0.lock().unwrap();
        let dir = tail.dir;
        let p1_t = tail.pos + Vector2::new((dir - PI as f32 /2.0).cos(),(dir - PI as f32 /2.0).sin()) * tail.radius;
        let p2_t = tail.pos + Vector2::new((dir - PI as f32 /4.0).cos(),(dir - PI as f32 /4.0).sin()) * tail.radius;
        let p3_t = tail.pos + Vector2::new( dir.cos(),dir.sin() ) * tail.radius;
        let p4_t = tail.pos + Vector2::new((dir + PI as f32 /4.0).cos(),(dir + PI as f32 /4.0).sin()) * tail.radius;
        let p5_t = tail.pos + Vector2::new((dir + PI as f32 /2.0).cos(),(dir + PI as f32 /2.0).sin()) * tail.radius;

        // d.draw_spline_basis(&vec![p1,p1,p2,p3,p4,p5,p5], 2.0, Color::BLACK);

        let mut line_points: Vec<Vector2> = line_points_l.iter().copied().rev().collect();
        line_points.append(&mut vec![p1_h,p1_h,p2_h,p3_h,p4_h,p5_h,p5_h]);
        line_points.append(&mut line_points_r);
        line_points.append(&mut vec![p1_t,p1_t,p2_t,p3_t,p4_t,p5_t,p5_t]);
        line_points.push(line_points[0].clone());
        line_points.push(line_points[1].clone());
        line_points.push(line_points[2].clone());
        
        d.draw_spline_basis(&line_points, 2.0, Color::BLACK);
    }
}
