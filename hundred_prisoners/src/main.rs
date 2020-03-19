use hundred_prisoners::*;
use rayon::prelude::*;

fn main() {
    let attempts = 10_000_000;
    let boxes = 50;
    let peeks = 25;
    let threads = 6;
    let is_one = |t| if t { 1 } else { 0 };

    let count = attempter(boxes, peeks, attempts);

    println!(
        "prison-solver {}/{} {:.5}",
        count,
        attempts * threads,
        count as f64 / (attempts * threads) as f64
    );

    let avg = (0_u64..threads)
        .into_par_iter()
        .map(|_| {
            let mut room = Room::new(boxes);
            (0..attempts)
                .map(|_| {
                    room.shuffle();
                    (0..boxes).all(|i| room.linear_solved(i, peeks))
                })
                .map(is_one)
                .sum::<u64>()
        })
        .sum::<u64>();

    println!(
        "linear-solver {}/{} {:.5}",
        avg,
        attempts * threads,
        avg as f64 / (attempts * threads) as f64
    );
}
