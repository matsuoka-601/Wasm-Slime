import { simd } from 'wasm-feature-detect';
import { Simulation } from './pkg/rust_fluid.js'

async function run() {
    const multithread = await import('./pkg/rust_fluid.js');
    await multithread.default();
    await multithread.initThreadPool(16);

    // const input = [];
    // const n = 10000000;
    // for (let i = 0; i < n; i++) {
    //     input.push(i);
    // }

    // let result = 0;
    // const start = performance.now();
    // for (let i = 0; i < 100; i++) {
    //     result += multithread.sum(input);
    // }
    // // for (let i = 0; i < 100; i++) {
    // //     for (let i = 0; i < n; i++) {
    // //         result += input[i];
    // //     }
    // // }
    // const end = performance.now();

    // console.log(result);
    // console.log(`${end - start}ms`);

    const canvas = /** @type {HTMLCanvasElement} */(document.getElementById('canvas'));

    let sim = new Simulation(canvas);
    sim.draw();

    function draw() {
        let start = performance.now();
        sim.step();
        let end = performance.now();
        console.log(`${end - start}ms`);
        sim.draw();
        requestAnimationFrame(draw);
    }

    draw();
}

run();