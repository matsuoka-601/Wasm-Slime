import { simd } from 'wasm-feature-detect';
import { start } from './pkg/rust_fluid.js'

async function run() {
    const multithread = await import('./pkg/rust_fluid.js');
    await multithread.default();

    // Experimentally, performance degrades when # of threads is larger than 12.
    // But this naive setting of numThreads should be improved.
    const numThreads = Math.min(12, navigator.hardwareConcurrency);
    // const numThreads = navigator.hardwareConcurrency;
    // const numThreads = 1;
    await multithread.initThreadPool(numThreads);

    const threadCountElement = document.getElementById('thread-count');
    threadCountElement.textContent = `Running with ${numThreads} threads`;

    start();
}

run();



