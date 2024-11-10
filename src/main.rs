mod solver;

use solver::State;
use std::time::{Instant, Duration};

fn main() {
    let mut state = State::new(8000, 1.5, 1.5, 900.0);

    let mut cnt = 0;
    loop {
        let start = Instant::now();
        for i in 0..10 {
            state.step();
        }
        let duration = start.elapsed();

        let check_sum : f32 = state.particles.iter().map(|p|p.position.y).sum();

        println!("{:?}ms, {:?}, {:?}", duration.as_millis(), check_sum, cnt);
        cnt = cnt + 1;
    }
}