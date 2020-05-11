use comprehensions::*;
use criterion::*;

fn numbers() -> impl Iterator<Item = u32> {
    vec![0, 2, 100, 200, 300].into_iter()
}

fn bench_pts(c: &mut Criterion) {
    let mut g = c.benchmark_group("pts");
    g.sample_size(100);
    for n in numbers() {
        g.bench_with_input(BenchmarkId::new("0", n), &n, |b, &n| {
            b.iter(|| pts1(n).fold(0, |acc, tri| tri.iter().sum::<u32>() + acc))
        });

        g.bench_with_input(BenchmarkId::new("1", n), &n, |b, &n| {
            b.iter(|| {
                let mut sum = 0;
                for x in 1..n + 1 {
                    for y in x..n + 1 {
                        for z in y..n + 1 {
                            if x.pow(2) + y.pow(2) == z.pow(2) {
                                sum += [x, y, z].iter().sum::<u32>();
                            }
                        }
                    }
                }
                sum
            })
        });
    }
}

criterion_group!(benches, bench_pts);
criterion_main!(benches);
