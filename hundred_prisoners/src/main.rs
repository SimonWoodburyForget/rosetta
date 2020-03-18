use hundred_prisoners::*;

fn main() {
    let attempts = 10_000_000;
    let boxes = 100;
    let peeks = 50;

    let solve = |f| {
        let mut room = Room::new(boxes);
        let avg = (0..attempts)
            .map(|_| {
                room.shuffle();
                (0..boxes).all(|i| f(i, peeks))
            })
            .map(|t| if t { 1 } else { 0 })
            .sum::<u64>();
    };

    let avg = solve(|i, max| room.prisoner_solved(i, peeks));
    println!(
        "prison-solver {}/{} {:.5}",
        avg,
        iterations,
        avg as f64 / iterations as f64
    );

    let avg = solve(|i, max| room.linear_solved(i, peeks));
    println!(
        "linear-solver {}/{} {:.5}",
        avg,
        iterations,
        avg as f64 / iterations as f64
    );
}
