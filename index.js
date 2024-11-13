import { simd } from 'wasm-feature-detect';
import { start } from './pkg/rust_fluid.js'

async function run() {
    const multithread = await import('./pkg/rust_fluid.js');
    await multithread.default();
    await multithread.initThreadPool(12);

    start();
}

run();