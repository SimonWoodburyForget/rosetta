use criterion::*;
use mylibrary::function;

fn bench_function(c: &mut Criterion) {
    c.bench_function("function", || function());
}

criterion_group!(benches, bench_function);
criterion_main!(benches);
