use rand::prelude::*;
use rayon::prelude::*;

/// Room is a (potentially) shuffled range of values (boxes).
#[derive(Debug, Clone)]
pub struct Room {
    boxes: Vec<usize>,
}

impl Room {
    pub fn new(size: usize) -> Self {
        let boxes = (0..size).collect();
        Self { boxes }
    }

    pub fn shifted(size: usize, shift: usize) -> Self {
        let boxes = (0..size).cycle().skip(shift).take(size).collect();
        Self { boxes }
    }

    pub fn shuffled(size: usize) -> Self {
        let mut boxes = Self::new(size);
        boxes.shuffle();
        boxes
    }

    pub fn get(&self, id: usize) -> usize {
        self.boxes[id]
    }

    pub fn shuffle(&mut self) {
        self.boxes.shuffle(&mut rand::thread_rng());
    }

    /// Returns an iterator which follows indices.
    pub fn index_walker<'a>(&'a self, state: usize) -> impl Iterator<Item = usize> + 'a {
        let mut inner_state = self.get(state);
        (0..).map(move |_| {
            let output = inner_state;
            inner_state = self.get(output);
            output
        })
    }

    /// Returns an iterator over the room which just iterates
    /// over elements linearly.
    pub fn linear_walker<'a>(&'a self, state: usize) -> impl Iterator<Item = usize> + 'a {
        self.boxes[state..]
            .iter()
            .chain(self.boxes[..state].iter())
            .cycle()
            .map(|&x| x)
    }

    /// Whether this room can be solved in max steps.
    pub fn prisoner_solved(&self, state: usize, max: usize) -> bool {
        self.index_walker(state).take(max).any(|y| y == state)
    }

    /// Whether this room can be solved by walking through `max`
    /// inline boxes.
    pub fn linear_solved(&self, state: usize, max: usize) -> bool {
        self.linear_walker(state).take(max).any(|y| y == state)
    }
}

/// Generate statistical probability of solution. As a result of this problem
/// being easy to run in parallel, I implemented threading on a best effort basis.
pub fn attempter(boxes: usize, peeks: usize, attempts: usize) -> usize {
    // work is split into 16 for no specific reason, other then it worked
    // better then splitting it exactly by the number of logical CPUs.
    let threads = if attempts >= 16 { 16 } else { 1 };
    let attempts = (attempts / threads) + attempts % 16;
    let is_one = |t| if t { 1 } else { 0 };

    let solver = |_| {
        let mut room = Room::new(boxes);
        (0..attempts)
            .map(|_| {
                room.shuffle();
                // thread boxes if boxes is high but attempt count is low
                if boxes > 1000 && attempts < 16 {
                    (0..boxes)
                        .into_par_iter()
                        .all(|i| room.prisoner_solved(i, peeks))
                } else {
                    (0..boxes).all(|i| room.prisoner_solved(i, peeks))
                }
            })
            .map(is_one)
            .sum::<usize>()
    };

    (0..threads).into_par_iter().map(solver).sum()
}

/// Checks whether the optimized attempter is within 30%,
/// which may potentially randomly fail.
#[test]
fn attempt_tester() {
    let att = 1_000;
    let res = attempter(100, 50, att);
    assert!((att / 4) < res);
    assert!((att / 2) > res);
}

#[test]
fn index_walker() {
    let room = Room::new(3);
    let mut w = room.index_walker(0);
    assert_eq!(w.next(), Some(0));
    assert_eq!(w.next(), Some(0));

    let room = Room::shifted(3, 1);
    let mut w = room.index_walker(0);
    assert_eq!(w.next(), Some(1));
    assert_eq!(w.next(), Some(2));
    assert_eq!(w.next(), Some(0));
    assert_eq!(w.next(), Some(1));

    let room = Room::shuffled(100);
    let w = room.index_walker(0);
    for id in w {
        if id == 0 {
            break;
        }
    }
}

#[test]
fn linear_walker() {
    let room = Room::new(3);
    let mut w = room.linear_walker(0);
    assert_eq!(w.next(), Some(0));
    assert_eq!(w.next(), Some(1));

    let room = Room::shifted(3, 1);
    let mut w = room.linear_walker(0);
    assert_eq!(w.next(), Some(1));
    assert_eq!(w.next(), Some(2));
    assert_eq!(w.next(), Some(0));
    assert_eq!(w.next(), Some(1));
    assert_eq!(w.next(), Some(2));
}

#[test]
fn solvable_rooms() {
    let room = Room::shifted(10, 1);
    println!("{:?}", room);
    assert!(!room.prisoner_solved(0, 1));
    assert!(!room.prisoner_solved(0, 9));
    assert!(room.prisoner_solved(0, 10));
}

#[test]
fn linear_solvables() {
    let room = Room::shifted(10, 1);
    println!("{:?}", room);

    assert!(!room.linear_solved(0, 5));
    // assert!(room.linear_solved(5, 5));
    assert!(!room.linear_solved(6, 5));
    assert!(room.linear_solved(0, 10));
}
