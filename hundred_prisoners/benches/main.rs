use criterion::*;
use hundred_prisoners::*;

fn room_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("room");
    g.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for max in (1..8).map(|x| 2_usize.pow(x)) {
        g.throughput(Throughput::Elements(max as u64));

        let id = BenchmarkId::new("prison-solver", max);
        g.bench_with_input(id, &max, |b, &max| {
            b.iter_batched(
                || Room::shuffled(max),
                |room| (0..max).all(|i| room.prisoner_solved(i, max)),
                BatchSize::SmallInput,
            )
        });

        let id = BenchmarkId::new("prison-solver-best", max);
        g.bench_with_input(id, &max, |b, &max| {
            b.iter_batched(
                || Room::new(max),
                |room| (0..max).all(|i| room.prisoner_solved(i, max)),
                BatchSize::SmallInput,
            )
        });

        let id = BenchmarkId::new("prison-solver-worst", max);
        g.bench_with_input(id, &max, |b, &max| {
            b.iter_batched(
                || Room::shifted(max, 1),
                |room| (0..max).all(|i| room.prisoner_solved(i, max)),
                BatchSize::SmallInput,
            )
        });

        let id = BenchmarkId::new("linear-solver", max);
        g.bench_with_input(id, &max, |b, &max| {
            b.iter_batched(
                || Room::shuffled(max),
                |room| (0..max).all(|i| room.linear_solved(i, max)),
                BatchSize::SmallInput,
            )
        });

        let id = BenchmarkId::new("linear-solver-best", max);
        g.bench_with_input(id, &max, |b, &max| {
            b.iter_batched(
                || Room::new(max),
                |room| (0..max).all(|i| room.linear_solved(i, max)),
                BatchSize::SmallInput,
            )
        });
    }
}

fn attempt_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("attempter");
    // g.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for max in (1..8).map(|x| 2_usize.pow(x) * 10) {
        g.throughput(Throughput::Elements(max as u64));

        let id = BenchmarkId::new("prison-solver-input-size", max);
        g.bench_with_input(id, &max, |b, &max| b.iter(|| attempter(max, max / 2, 1)));

        let id = BenchmarkId::new("prison-solver-attempt-size", max);
        g.bench_with_input(id, &max, |b, &max| b.iter(|| attempter(100, 50, max)));
    }
}

criterion_group!(benches, attempt_bench);
criterion_main!(benches);
