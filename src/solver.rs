use core::num;

use glam::Vec2;
use rand::Rng;

use rayon::prelude::*;
use wasm_bindgen::prelude::*;
use std::f32::consts::PI;


pub struct State {
    pub num_particles: u32, 
    pub particles: Vec<Particle>
}

pub struct Particle {
    pub position: Vec2, 
    pub velocity: Vec2, 
    pub force: Vec2, 
    pub pressure: f32, 
    pub density: f32, 
    pub size: f32
}

const DT: f32 = 0.001;
const PARTICLE_SIZE: f32 = 0.0085;
const KERNEL_RADIUS: f32 = 1.2 * PARTICLE_SIZE;
const KERNEL_RADIUS_SQ: f32 = KERNEL_RADIUS * KERNEL_RADIUS;
const KERNEL_RADIUS_POW4: f32 = KERNEL_RADIUS_SQ * KERNEL_RADIUS_SQ;
const KERNEL_RADIUS_POW5: f32 = KERNEL_RADIUS_POW4 * KERNEL_RADIUS;
const KERNEL_RADIUS_POW8: f32 = KERNEL_RADIUS_POW4 * KERNEL_RADIUS_POW4;
const TARGET_DENSITY: f32 = 300.0;
const STIFFNESS: f32 = 3.0;
const MASS: f32 = 2.5 / 600.0 / 600.0;
const POLY6: f32 = 4.0 / (PI * KERNEL_RADIUS_POW8); 
const SPIKY_GRAD: f32 = -10.0 / (PI * KERNEL_RADIUS_POW5); 
const VISC_LAP: f32 = 40.0 / (PI * KERNEL_RADIUS_POW5); 
const VISCOSITY: f32 = 0.0001;
const EPS: f32 = 1e-9;
const GRV: Vec2 = Vec2::new(0.0, -9.8);

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl State {
    pub fn new(num_particles: u32) -> Self {
        let particles = Self::init_particles(num_particles);
        Self { num_particles, particles }
    }

    pub fn step(&mut self) {
        self.compute_density_pressure();
        self.compute_force();

        self.particles.par_iter_mut().for_each(|particle|{
            particle.velocity += particle.force * (DT / particle.density);
            particle.position += particle.velocity * DT;

            if particle.position.y - KERNEL_RADIUS < 0.0 {
                particle.position.y = KERNEL_RADIUS;
                particle.velocity.y = -0.5;
            }
            if particle.position.y + 2.0 * KERNEL_RADIUS > 1.0 { // TODO : fix
                particle.position.y = 1.0 - 2.0 * KERNEL_RADIUS;
                particle.velocity.y = -0.5;
            }
            if particle.position.x - KERNEL_RADIUS < 0.0 {
                particle.position.x = KERNEL_RADIUS;
                particle.velocity.x *= -0.5;
            }
            if particle.position.x + 2.0 * KERNEL_RADIUS > 1.0 {
                particle.position.x = 1.0 - 2.0 * KERNEL_RADIUS;
                particle.velocity.x *= -0.5;
            }
        });
    }

    fn compute_density_pressure(&mut self) {
        let mut densities = vec![0.0 as f32; self.num_particles as usize];

        densities.par_iter_mut().enumerate().for_each(|(i, density)| {
            let pi = &self.particles[i];
            for pj in &self.particles {
                let r = (pj.position - pi.position).length();
                if r < KERNEL_RADIUS {
                    *density += MASS * POLY6 * (KERNEL_RADIUS_SQ - r*r).powf(3.0);
                }
            }
        });

        self.particles.par_iter_mut().enumerate().for_each(|(i, particle)| {
            particle.density = densities[i];
            particle.pressure = STIFFNESS * (densities[i] - TARGET_DENSITY);
        });
    }

    fn compute_force(&mut self) {
        let mut forces = vec![Vec2::new(0.0, 0.0); self.num_particles as usize];

        forces.par_iter_mut().enumerate().for_each(|(i, force)|{
            let mut fpress = Vec2::new(0.0, 0.0);
            let mut fvisc = Vec2::new(0.0, 0.0);
            let pi = &self.particles[i];
            for (j, pj) in self.particles.iter().enumerate() {
                if i == j {
                    continue;
                }
                let mut rij = pj.position - pi.position;
                let mut r = rij.length();

                // if r < EPS {
                //     rij = Vec2::new(EPS, EPS); // TODO : fix
                //     r = rij.length();
                // }

                if r < KERNEL_RADIUS {
                    // log(&r.to_string());
                    let shared_pressure = (pi.pressure + pj.pressure) / 2.0;
                    let press_coeff = -MASS * shared_pressure * SPIKY_GRAD * (KERNEL_RADIUS - r).powf(3.0) / pj.density;
                    fpress += press_coeff * rij.normalize();
                    let visc_coeff = VISCOSITY * MASS * VISC_LAP * (KERNEL_RADIUS - r) / pj.density;
                    let relative_speed = pj.velocity - pi.velocity;
                    fvisc += visc_coeff * relative_speed;
                }
            }
            let fgrv = pi.density * GRV;
            *force = fpress + fvisc + fgrv;
        });

        self.particles.par_iter_mut().enumerate().for_each(|(i, particle)| {
            particle.force = forces[i];
        });
    }

    fn init_particles(num_particles: u32) -> Vec<Particle> {
        let mut particles = Vec::new();
        particles.reserve(num_particles as usize);
        let mut rng = rand::thread_rng();

        // for _ in 0..num_particles {
        //     let position = Vec2::new(rng.gen(), rng.gen());
        //     let velocity = Vec2::new(0.0, 0.0);
        //     let force = Vec2::new(0.0, 0.0);
        //     let pressure = 0.0;
        //     let density = 0.0;
        //     let size = 8.0;
        //     particles.push(Particle{ position, velocity, force, pressure, density, size });
        // }

        let mut y = 10.0 * PARTICLE_SIZE;
        loop {
            let mut x = 1.0 / 8.0;
            loop {
                let position = Vec2::new(x, y);
                let velocity = Vec2::new(0.0, 0.0);
                let force = Vec2::new(0.0, 0.0);
                let pressure = 0.0;
                let density = 0.0;
                let size = 8.0;
                particles.push(Particle{ position, velocity, force, pressure, density, size });
                x += 1.5 * PARTICLE_SIZE + 0.001 * rng.gen::<f32>();
                if x > 7.0 / 8.0 {
                    break;
                }
                if particles.len() == num_particles as usize {
                    break;
                }
            }
            if particles.len() == num_particles as usize {
                break;
            }
            y += 1.5 * PARTICLE_SIZE;
        }

        particles
    }
}