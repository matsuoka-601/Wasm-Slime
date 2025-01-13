# wasm-slime

A 2d fluid simulator implemented with Rust + wasm-bindgen-rayon. 

![img](img/demo.gif)

**Check the demo here! (needs a browser that supports SharedArrayBuffer)** : https://fluid-simulation-test.netlify.app/

The following is the brief description of the simulation.
- Solver is implemented in Rust and compiled to WASM. 
- The simulation is parallelized by multi-threading using [wasm-bindgen-rayon](https://github.com/RReverser/wasm-bindgen-rayon). 
- The simulation is based on SPH method described in [Particle-Based Fluid Simulation for Interactive Applications](https://matthias-research.github.io/pages/publications/sca03.pdf) by Müller et al. 
    - Techniques called near-density and near-pressure are also implemented which is described in [Particle-based Viscoelastic Fluid Simulation](https://www.ljll.fr/~frey/papers/levelsets/Clavet%20S.,%20Particle-based%20viscoelastic%20fluid%20simulation.pdf) by Clavet et al. This technique is useful to realize surface tension. 
- This project is inspired by Sebastian Lague's video : [Coding Adventure: Simulating Fluids](https://www.youtube.com/watch?v=rSKMYc1CQHE).

## How to run locally
Basically, you can run the simulation locally by running the following commands.
```
npm install
npm run build
npm run serve
```
But in some environments, webpack seems to cause some errors (see [this issue](https://github.com/matsuoka-601/wasm-fluid-simulation/issues/1)). In that case, you can build the repo without webpack with the following steps.
- Change `"build"` in `package.json` like below.
	- before: `"build": "cross-env RUSTUP_TOOLCHAIN=nightly wasm-pack build --target web && webpack build ./index.js --mode production -o dist --output-filename index.js && shx cp index.html dist/",`
	- after: `"build": "cross-env RUSTUP_TOOLCHAIN=nightly wasm-pack build --target web",`
- Change the code in `Cargo.toml` like below.
	- before: `wasm-bindgen-rayon = { version = "1.2" }`
	- after: `wasm-bindgen-rayon = { version = "1.2", features = ["no-bundler"] }`
- Change the code in `server.js` like below.
	- before: `app.use(express.static(__dirname + '/dist/'));`
	- after: `app.use(express.static(__dirname));`
- Remove the line `import { simd } from 'wasm-feature-detect';` in `index.js`
## References
- [Coding Adventure: Simulating Fluids](https://www.youtube.com/watch?v=rSKMYc1CQHE&t=877s)
	- This is a video that gave me an motivation to implement this project.
- [Particle-Based Fluid Simulation for Interactive Applications, Müller et al. 2003](https://matthias-research.github.io/pages/publications/sca03.pdf)
	- Basically, the fluid simulation in this project is based on this paper.
- [Particle-based Viscoelastic Fluid Simulation, Clavet et al. 2005](https://www.ljll.fr/~frey/papers/levelsets/Clavet%20S.,%20Particle-based%20viscoelastic%20fluid%20simulation.pdf)
	- Techniques called near density and near pressure are also implemented that is presented in this paper. These techniques are useful to realize a force like a surface tension.
- SPH implementations and articles by Lucas V. Schuermann helped me a lot.
	- [Implementing SPH in 2D](https://lucasschuermann.com/writing/implementing-sph-in-2d)
 	- [mueller-sph](https://github.com/lucas-schuermann/mueller-sph)	
