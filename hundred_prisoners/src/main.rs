use hundred_prisoners::*;

fn main() {
    let attempts = 100_000_000;
    let boxes = 50;
    let peeks = 25;
    let is_one = |t| if t { 1 } else { 0 };
    let mut room = Room::new(boxes);

    let avg = (0..attempts)
        .map(|_| {
            room.shuffle();
            (0..boxes).all(|i| room.prisoner_solved(i, peeks))
        })
        .map(is_one)
        .sum::<u64>();

    println!(
        "prison-solver {}/{} {:.5}",
        avg,
        attempts,
        avg as f64 / attempts as f64
    );

    let avg = (0..attempts)
        .map(|_| {
            room.shuffle();
            (0..boxes).all(|i| room.linear_solved(i, peeks))
        })
        .map(is_one)
        .sum::<u64>();

    println!(
        "linear-solver {}/{} {:.5}",
        avg,
        attempts,
        avg as f64 / attempts as f64
    );
}
