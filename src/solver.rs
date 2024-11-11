use core::num;

use glam::Vec2;
// use rand::Rng;
use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};
use web_time::Instant;
use std::sync::atomic::{AtomicU32, Ordering};

use rayon::prelude::*;
use wasm_bindgen::prelude::*;
use std::f32::consts::PI;


pub struct State {
    pub num_particles: u32, 
    pub particles: Vec<Particle>, 
    pub field: Field, 
    pub cells: Cells, 
    pub counter: AtomicU32
}

#[derive(Clone)]
pub struct Particle {
    pub position: Vec2, 
    pub velocity: Vec2, 
    pub force: Vec2, 
    pub pressure: f32, 
    pub density: f32, 
    pub size: f32
}

pub struct Field {
    pub height: f32, 
    pub width: f32,
}

pub struct Cells {
    pub cells: Vec<Vec<u32>>, 
    pub nx: usize, 
    pub ny: usize, 
}

const DT: f32 = 0.0005;
const PARTICLE_SIZE: f32 = 0.005;
const KERNEL_RADIUS: f32 = 1.8 * PARTICLE_SIZE;
const KERNEL_RADIUS_SQ: f32 = KERNEL_RADIUS * KERNEL_RADIUS;
const KERNEL_RADIUS_POW4: f32 = KERNEL_RADIUS_SQ * KERNEL_RADIUS_SQ;
const KERNEL_RADIUS_POW5: f32 = KERNEL_RADIUS_POW4 * KERNEL_RADIUS;
const KERNEL_RADIUS_POW8: f32 = KERNEL_RADIUS_POW4 * KERNEL_RADIUS_POW4;
const TARGET_DENSITY: f32 = 500.0;
const STIFFNESS: f32 = 2.0;
const MASS: f32 = 3.0e-6;
const POLY6: f32 = 4.0 / (PI * KERNEL_RADIUS_POW8); 
const SPIKY_GRAD: f32 = -10.0 / (PI * KERNEL_RADIUS_POW5); 
const VISC_LAP: f32 = 40.0 / (PI * KERNEL_RADIUS_POW5); 
const VISCOSITY: f32 = 0.0002;
const EPS: f32 = 1e-30;
const GRV: Vec2 = Vec2::new(0.0, -9.8);

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

macro_rules! benchmark {
    ($code:block) => {{
        let start = Instant::now(); 
        $code
        start.elapsed().as_micros()
    }};
}

impl State {
    pub fn new(num_particles: u32, height: f32, width: f32, scale: f32) -> Self {
        let field = Field { height, width };
        let particles = Self::init_particles(num_particles, scale, &field);
        let cells = Cells::new(height, width, KERNEL_RADIUS);
        let counter = AtomicU32::new(0);
        Self { num_particles, particles, field, cells, counter }
    }

    pub fn step(&mut self) {
        let t1 = benchmark!({self.cells.register_cells(&self.particles)});
        let t2 = benchmark!({self.compute_density_pressure()});
        let t3 = benchmark!({self.compute_force()});
        let t4 = benchmark!({self.handle_boundary()});
        let s = format!("{}us, {}us, {}us, {}us", t1, t2, t3, t4);
        log(&s);
        // println!("{}", s);
        // self.cells.register_cells(&self.particles);
        // self.compute_density_pressure();
        // self.compute_force();
        // self.handle_boundary();
    }

    fn handle_boundary(&mut self) {
        let field_height = self.field.height;
        let field_width = self.field.width;

        self.particles.par_iter_mut().for_each(|particle|{
            particle.velocity += particle.force * (DT / particle.density);
            particle.position += particle.velocity * DT;

            if particle.position.y - KERNEL_RADIUS < 0.0 {
                particle.position.y = KERNEL_RADIUS;
                particle.velocity.y = -0.5;
            }
            if particle.position.y + 2.0 * KERNEL_RADIUS > field_height { 
                particle.position.y = field_height - 2.0 * KERNEL_RADIUS;
                particle.velocity.y = -0.5;
            }
            if particle.position.x - KERNEL_RADIUS < 0.0 {
                particle.position.x = KERNEL_RADIUS;
                particle.velocity.x *= -0.5;
            }
            if particle.position.x + 2.0 * KERNEL_RADIUS > field_width {
                particle.position.x = field_width - 2.0 * KERNEL_RADIUS;
                particle.velocity.x *= -0.5;
            }
        });
    }

    fn compute_density_pressure(&mut self) {
        let particles_copy = self.particles.clone();
        let cells = &self.cells;
        let counter = &self.counter;

        self.particles.par_iter_mut().enumerate().for_each(|(i, particle)| {
            let pi = &particles_copy[i];
            particle.density = 0.0;

            let grid_x = (pi.position.x / KERNEL_RADIUS) as i32;
            let grid_y = (pi.position.y / KERNEL_RADIUS) as i32;

            let xrange = std::cmp::max(grid_x - 1, 0) ..= std::cmp::min(grid_x + 1, cells.nx as i32 - 1);

            for gx in xrange {
                let yrange = std::cmp::max(grid_y - 1, 0) ..= std::cmp::min(grid_y + 1, cells.ny as i32 - 1);
                for gy in yrange {
                    let grid_id = gy as usize * cells.nx + gx as usize;
                    for j in &cells.cells[grid_id] {
                        // counter.fetch_add(1, Ordering::SeqCst);
                        let pj = &particles_copy[*j as usize];
                        let r = (pj.position - pi.position).length();
                        if r < KERNEL_RADIUS {
                            let a = KERNEL_RADIUS_SQ - r*r;
                            particle.density += MASS * POLY6 * a * a * a;
                        }
                    }
                }
            }

            particle.pressure = STIFFNESS * (particle.density - TARGET_DENSITY);
        });
    }

    fn compute_force(&mut self) {
        let mut forces = vec![Vec2::new(0.0, 0.0); self.num_particles as usize];

        let counter = &self.counter;

        forces.par_iter_mut().enumerate().for_each(|(i, force)|{
            let mut fpress = Vec2::new(0.0, 0.0);
            let mut fvisc = Vec2::new(0.0, 0.0);
            let pi = &self.particles[i];

            let grid_x = (pi.position.x / KERNEL_RADIUS) as i32;
            let grid_y = (pi.position.y / KERNEL_RADIUS) as i32;

            for gx in std::cmp::max(grid_x - 1, 0) ..= std::cmp::min(grid_x + 1, self.cells.nx as i32 - 1) {
                for gy in std::cmp::max(grid_y - 1, 0) ..= std::cmp::min(grid_y + 1, self.cells.ny as i32 - 1) {
                    let grid_id = gy as usize * self.cells.nx + gx as usize;
                    for j in &self.cells.cells[grid_id] {
                        // counter.fetch_add(1, Ordering::SeqCst);
                        if i == *j as usize {
                            continue;
                        }
                        let pj = &self.particles[*j as usize];
                        let rij = pj.position - pi.position;
                        let r = rij.length();
        
                        if EPS < r && r < KERNEL_RADIUS {
                            let a = KERNEL_RADIUS - r;
                            let shared_pressure = (pi.pressure + pj.pressure) / 2.0;
                            let press_coeff = -MASS * shared_pressure * SPIKY_GRAD * a * a * a / pj.density;
                            fpress += press_coeff * rij.normalize();
                            let visc_coeff = VISCOSITY * MASS * VISC_LAP * a / pj.density;
                            let relative_speed = pj.velocity - pi.velocity;
                            fvisc += visc_coeff * relative_speed;
                        }
                    }
                }
            }
            
            let fgrv = pi.density * GRV;
            *force = fpress + fvisc + fgrv;
        });

        self.particles.par_iter_mut().enumerate().for_each(|(i, particle)| {
            particle.force = forces[i];
        });
    }

    fn init_particles(num_particles: u32, scale: f32, field: &Field) -> Vec<Particle> {
        let mut particles = Vec::new();
        particles.reserve(num_particles as usize);

        let seed = 12345; 
        let mut rng = StdRng::seed_from_u64(seed);

        let mut y = 2.0 * KERNEL_RADIUS;
        loop {
            let mut x = field.width * 0.1;
            loop {
                let position = Vec2::new(x, y);
                let velocity = Vec2::new(0.0, 0.0);
                let force = Vec2::new(0.0, 0.0);
                let pressure = 0.0;
                let density = 0.0;
                let size = PARTICLE_SIZE * scale;
                particles.push(Particle{ position, velocity, force, pressure, density, size });
                x += 1.3 * PARTICLE_SIZE + 0.0001 * rng.gen::<f32>();
                if x > field.width * 0.9 {
                    break;
                }
                if particles.len() == num_particles as usize {
                    break;
                }
            }
            if particles.len() == num_particles as usize {
                break;
            }
            y += KERNEL_RADIUS;
        }

        particles
    }
}

impl Cells {
    pub fn new(height: f32, width: f32, radius: f32) -> Self {
        let ny = (height / radius).ceil() as usize;
        let nx = (width / radius).ceil() as usize;
        let cells = vec![Vec::new(); nx * ny];
        Cells { cells, nx, ny }
    }

    fn cell_position_to_id(&self, ix: usize, iy: usize) -> usize {
        self.nx * iy + ix
    }

    pub fn register_cells(&mut self, particles: &Vec<Particle>) {
        self.cells.iter_mut().for_each(|v| v.clear());
        particles.iter().enumerate().for_each(|(i, particle)|{
            let ix = (particle.position.x / KERNEL_RADIUS) as usize;
            let iy = (particle.position.y / KERNEL_RADIUS) as usize;
            let cell_id = self.cell_position_to_id(ix, iy);
            self.cells[cell_id].push(i as u32);
        });
    }

    pub fn neighbors(&self, particle: &Particle, radius: f32) -> Vec<u32> {
        let ix = (particle.position.x / radius) as i32;
        let iy = (particle.position.y / radius) as i32;
        let dx_ = [-1, 0, 1];
        let dy_ = [-1, 0, 1];

        let mut v = Vec::new();
        for dx in dx_ {
            for dy in dy_ {
                let jx = ix + dx;
                let jy  = iy + dy;
                if 0 <= jx && jx < self.nx as i32 && 0 <= jy && jy < self.ny as i32 {
                    v.extend_from_slice(&self.cells[self.cell_position_to_id(jx as usize, jy as usize)]);
                }
            }
        }
        v
    }
}
