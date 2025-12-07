#![allow(unused)]
use std::{f32::consts::PI, thread::sleep, time::Duration};

use rand::distr::slice::Choose;
use rand::prelude::*;
use rand_distr::{Normal, Uniform};

use raylib::prelude::*;

struct Polar {
    rho: f32,
    theta: f32,
}
impl From<Vector2> for Polar {
    fn from(value: Vector2) -> Self {
        let x = value.x;
        let y = value.y;
        Polar {
            rho: (x.powi(2) + y.powi(2)).sqrt(),
            theta: f32::atan2(y, x),
        }
    }
}
impl Into<Vector2> for Polar {
    fn into(self) -> Vector2 {
        Vector2 {
            x: self.rho * self.theta.cos(),
            y: self.rho * self.theta.sin(),
        }
    }
}

fn main() {
    let num_agents = 5;
    let (mut rl, thread) = raylib::init().size(400, 400).title("Random walker").build();

    let mut rng = rand::rng();
    let mut current = Vector2::new(400. / 2., 400. / 2.);
    let size = Vector2::new(2., 2.);
    let dist = Uniform::new(0., 15.).unwrap();
    let angles = [0., PI / 2., PI, 3. / 2. * PI];
    // let dist = Choose::new().unwrap();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // d.clear_background(Color::WHITE);
        let delta: Vector2 = Polar {
            rho: rng.sample(dist),
            // theta:#rng.sample(dist),
            theta: *(angles.choose(&mut rng).unwrap()),
        }
        .into();
        println!("{:?}", delta);
        let mut prev = current.clone();
        current += Into::<Vector2>::into(delta);
        d.draw_rectangle_v(current, size, Color::RED);
        d.draw_line_v(prev, current, Color::RED);
        sleep(Duration::from_millis(100));
    }
}
