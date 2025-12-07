#![allow(unused)]
use std::{f32::consts::PI, thread::sleep, time::Duration};

use clap::{Arg, Parser};
use rand::distr::{Uniform, slice::Choose};
use rand::prelude::*;

use random_walk::agent::Agent;
use raylib::prelude::*;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    num_agents: u8,
    #[arg(long, default_value_t = 800)]
    window_height: i32,
    #[arg(long, default_value_t = 800)]
    window_width: i32,
}
fn main() {
    let args = Args::parse();

    let num_agents = args.num_agents;
    let (mut rl, thread) = raylib::init()
        .size(args.window_width, args.window_height)
        .title("Random walker")
        .build();
    rl.set_target_fps(60);
    let mut rng = rand::rng();
    let mut agents: Vec<Agent> = (0..args.num_agents)
        .map(|_| Agent::random_init(args.window_height, args.window_width, &mut rng))
        .collect();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        for agent in &mut agents {
            agent.update(&mut rng);
            agent.draw(&mut d);
        }
    }
}
