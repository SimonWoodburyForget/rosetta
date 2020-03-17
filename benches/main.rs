use criterion::*;
use rosetta_sorting_performance::*;
use std::time::Duration;

fn sort_bench(c: &mut Criterion) {
    for input in generation_algos() {
        let config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
        let mut g = c.benchmark_group(input.name);
        g.plot_config(config);
        for sorter in sorting_algos() {
            for i in (1..=32_u32).map(|x| x * 32) {
                g.throughput(Throughput::Elements(i as u64));
                g.warm_up_time(Duration::from_millis((i * 2 + 10) as u64));
                g.measurement_time(Duration::from_millis((i * 24 + 20) as u64));
                let id = BenchmarkId::new(sorter.name, i);
                g.bench_with_input(id, &i, |b, &i| {
                    b.iter_batched(
                        || (input.f)(i),
                        |mut data| (sorter.f)(&mut data),
                        BatchSize::SmallInput,
                    )
                });
            }
        }
        g.finish();
    }
}

criterion_group!(benches, sort_bench);
criterion_main!(benches);
