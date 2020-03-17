use std::{test::black_box, time::Instant};

fn function(x: u32) -> u32 {
    (0..n).sum()
}

fn main() {
    let now = Instant::now();
    black_box(function(100));
    let delta = now.elapsed();
    println!("{}", delta);
}
