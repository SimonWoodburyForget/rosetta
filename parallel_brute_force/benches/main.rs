use criterion::*;
use prallel_brute_force::*;

fn brute_bench(c: &mut Criterion) {}

criterion_group!(benches, brute_bench);
criterion_main!(benches);
