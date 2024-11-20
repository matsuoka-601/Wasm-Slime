# wasm-fluid-simulation

A 2d fluid simulator implemented with Rust + wasm-bindgen-rayon. 

![img](img/demo.gif)

**Check the demo here! (needs a browser that supports SharedArrayBuffer)** : https://fluid-simulation-test.netlify.app/

The following is the brief description of the simulation.
- Solver is implemented in Rust and compiled to WASM. 
- The simulation is parallelized by multi-threading using [wasm-bindgen-rayon](https://github.com/RReverser/wasm-bindgen-rayon). 
- The simulation is based on SPH method described in [Particle-Based Fluid Simulation for Interactive Applications](https://matthias-research.github.io/pages/publications/sca03.pdf) by Müller et al. 
    - A technique called near-density and near-pressure is also implemented which is described in [Particle-based Viscoelastic Fluid Simulation](https://www.ljll.fr/~frey/papers/levelsets/Clavet%20S.,%20Particle-based%20viscoelastic%20fluid%20simulation.pdf) by Clavet et al. This technique is useful to realize surface tension. 
- This project is inspired by Sebastian Lague's video : [Coding Adventure: Simulating Fluids](https://www.youtube.com/watch?v=rSKMYc1CQHE).
