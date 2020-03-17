use rand::prelude::*;
use the_algorithms_rust::sorting::*;

/// Wraps a named algorithm.
pub struct Algorithm<T> {
    pub name: &'static str,
    pub f: T,
}

impl<T> Algorithm<T> {
    pub fn new(name: &'static str, f: T) -> Self {
        Self { name, f }
    }
}

type Sort = Algorithm<&'static dyn Fn(&mut [u32])>;
type Gens = Algorithm<&'static dyn Fn(u32) -> Vec<u32>>;

/// Returns vector of sorting algorithms.
pub fn sorting_algos() -> Vec<Sort> {
    vec![
        Sort::new("builtin-sort", &|xs| xs.sort()),
        Sort::new("bubble-sort", &|mut xs| bubble_sort(&mut xs)),
        Sort::new("quick-sort", &|mut xs| quick_sort(&mut xs)),
        Sort::new("heap-sort", &|mut xs| heap_sort(&mut xs)),
        Sort::new("generic-counting-sort", &|mut xs| {
            let max = xs.len();
            generic_counting_sort(&mut xs, max);
        }),
        Sort::new("counting-sort", &|mut xs| {
            let max = xs.len();
            counting_sort(&mut xs, max);
        }),
        Sort::new("selection-sort", &|mut xs| selection_sort(&mut xs)),
        Sort::new("insert-sort", &|mut xs| {
            insertion_sort(&mut xs);
        }),
    ]
}

/// Returns vector of sequence generation algorithms.
pub fn generation_algos() -> Vec<Gens> {
    vec![
        Gens::new("ones", &|n| vec![1; n as usize]),
        Gens::new("sorted-range", &|n| (0..n).collect()),
        Gens::new("rev-range", &|n| (0..n).rev().collect()),
        Gens::new("shuffle-range", &|n| {
            let mut v: Vec<u32> = (0..n).collect();
            v.shuffle(&mut thread_rng());
            v
        }),
    ]
}
