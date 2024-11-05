mod utils;

pub use wasm_bindgen_rayon::init_thread_pool;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn sum(input: &[i32]) -> i32 {
    input.par_iter().map(|&x| x).sum()
    // return 0;
}