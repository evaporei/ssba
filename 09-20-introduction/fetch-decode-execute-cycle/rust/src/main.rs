use std::time::Instant;

const ITERS: u128 = 1_000_000_000;

fn main() {
    let now = Instant::now();
    let mut i = 0;
    loop {
        i += 1;
        if i > ITERS {
            break;
        }
    }
    let elapsed = now.elapsed();
    println!("elapsed: {:?}", elapsed);
    let secs = elapsed.as_secs_f64();
    let ops = ITERS as f64 / secs;
    println!("clock speed approx {} GHz", ops / 1_000_000_000.0);
}
