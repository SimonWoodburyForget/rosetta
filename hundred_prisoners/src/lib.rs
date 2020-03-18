use rand::prelude::*;

/// Room is a (potentially) shuffled range of values (boxes).
#[derive(Debug)]
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
    pub fn index_walker(&self, state: usize) -> IndexWalker<'_> {
        IndexWalker::new(self.get(state), &self)
    }

    /// Returns an iterator over the room which just iterates
    /// over elements linearly.
    pub fn linear_walker<'a>(&'a self, state: usize) -> impl Iterator<Item = usize> + 'a {
        self.boxes[..state]
            .iter()
            .chain(self.boxes[state..].iter())
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

/// An iterator over the room as defined by the 100-prisoners solution.
pub struct IndexWalker<'a> {
    boxes: &'a Room,
    state: usize,
}

impl IndexWalker<'_> {
    fn new(state: usize, boxes: &Room) -> IndexWalker<'_> {
        IndexWalker { boxes, state }
    }
}

impl Iterator for IndexWalker<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let old_state = self.state;
        self.state = self.boxes.get(self.state);
        Some(old_state)
    }
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
    assert!(room.linear_solved(5, 5));
    assert!(!room.linear_solved(6, 5));
    assert!(room.linear_solved(0, 10));
}
