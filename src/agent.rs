use std::f32::consts::PI;

use rand::{distr::Uniform, prelude::*};
use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::polar::Polar;

pub struct Agent {
    color: Color,
    size: Vector2,
    current: Vector2,
    prev: Vector2,
}

impl Agent {
    pub fn new(color: Color, size: Vector2, current: Vector2) -> Self {
        Agent {
            color,
            size,
            current,
            prev: current,
        }
    }
    pub fn random_init(window_height: i32, window_width: i32, rng: &mut ThreadRng) -> Self {
        let color_dist = Uniform::new(0u8, 255u8).unwrap();

        let color = Color::new(
            rng.sample(color_dist),
            rng.sample(color_dist),
            rng.sample(color_dist),
            255,
        );

        let size_dist = Uniform::new(2f32, 10f32).unwrap();
        let size = rng.sample(size_dist);
        let size = Vector2::new(size, size);
        let current = Vector2::new(window_width as f32 / 2.0, window_height as f32 / 2.0);

        Self::new(color, size, current)
    }
    pub fn update(&mut self, rng: &mut ThreadRng) {
        let rho_distribution = Uniform::new(0., 15.).unwrap();
        let angles = [0., PI / 2., PI, 3. / 2. * PI];
        let delta: Vector2 = Polar {
            rho: rng.sample(rho_distribution),
            // theta:#rng.sample(dist),
            theta: *(angles.choose(rng).unwrap()),
        }
        .into();
        self.prev = self.current;
        self.current += delta;
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(self.current, self.size, self.color);
        d.draw_line_v(
            self.prev + self.size / 2.,
            self.current + self.size / 2.,
            self.color,
        );
    }
}
