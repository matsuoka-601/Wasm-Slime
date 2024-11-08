use glam::Vec2;
use rand::Rng;

use rayon::prelude::*;
use wasm_bindgen::prelude::*;

pub struct State {
    pub num_particles: u32, 
    pub particles: Vec<Particle>
}

pub struct Particle {
    pub position: Vec2, 
    pub velocity: Vec2, 
    pub size: f32
}

const DT: f32 = 0.01;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

impl State {
    pub fn new(num_particles: u32) -> Self {
        let particles = Self::init_particles(num_particles);
        Self { num_particles, particles }
    }

    pub fn step(&mut self) {
        self.particles.par_iter_mut().for_each(|particle|{
            particle.position += particle.velocity * DT;
        });
    }

    fn init_particles(num_particles: u32) -> Vec<Particle> {
        let mut particles = Vec::new();
        particles.reserve(num_particles as usize);
        let mut rng = rand::thread_rng();

        for _ in 0..num_particles {
            let position = Vec2::new(rng.gen(), rng.gen());
            let velocity = Vec2::new(0.0, 0.0);
            let size = 8.0;
            particles.push(Particle{ position, velocity, size });
        }

        particles
    }
}