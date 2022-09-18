// const SIZE: usize = 2048;// 'cargo run' terminated by signal SIGSEGV (Address boundary error)
const SIZE: usize = 1024;

fn iterate_ij() {
    let mut m = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        for j in 0..SIZE {
            m[i][j] += 1;
        }
    }
}

fn iterate_ji() {
    let mut m = [[0; SIZE]; SIZE];
    for j in 0..SIZE {
        for i in 0..SIZE {
            m[i][j] += 1;
        }
    }
}

// fn works() {
//     let mut m = [[0; 2]; 2];
//
//     for i in 0..2 {
//         for j in 0..2 {
//             m[i][j] += 1;
//             println!("i {i} j {j}");
//         }
//     }
// }

use std::time::Instant;

fn main() {
    // works();

    let ij = Instant::now();
    iterate_ij();
    println!("{:?}", ij.elapsed());// 32.590666ms

    let ji = Instant::now();
    iterate_ji();
    println!("{:?}", ji.elapsed());// 21.997625ms
}
