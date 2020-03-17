There are several ways of measuring time in Rust depending on exactly
what you want to do. Time in general is a complicated subject, so
there are a lot of ways of measuring it.

To measuring a function's runtime, `Instant` should generally be
used, as it guarantees every subsequent instants to always increase
time. -- Primarly it's purpose is to measure monotonic time; which
means seconds/nanos and but not hours/days. 

It is internally split into two integers (`u32` and `u64`) for
reliably and accurately measuring long and very short periods of
time, such that there is no floating point errors to deal with.

All `Instant` types created can be compared to each other but not
directly with `SystemTime`, because system time doesn't specifically
promise anything about how time will behave. In other words all
`Instant` types will usually have been created by calling
`Instant::now()` directly or by adding it with a `Duration` type.

To measure a function in Rust, you must ensure you use the resulting
output of this function, or mutate the input of said function,
otherwise the compiler may optimize away the function entirely. To do
so Rust provides a `black_box` which is tries to ensure the compiler
doesn't optimize said code away entirely.

The following will calculate the `Duration` of the `function`,
ensuring it doesn't get optimized away with `black_box` such that it
can't be optimized away as unused code.

    use std::{test::black_box, time::Instant};
    fn main() {
        let function = |n| (0..n).sum();
        let now = Instant::now();
        black_box(function(100));
        let delta = now.elapsed();
        println!("{:?}", delta.as_nanos());
    }
    
If executed thes could would print the result to stdout as
nanoseconds, representing them as an integer.

----------------------------------------------------------
    
You can measure down to a nanosecond using using `Duration` though
this doesn't guarantee anything about whether your function is being
blocked by the operating system, so in order to benchmark a function
you need to do statistical analysis, by measuring it repeatedly and
accounting for system noise.

It's possible to measure down to a few hundred picoseconds of accuracy
and in Rust this would be done with a crate like Criterion.

    use criterion::*;
    use my_crate::function;

    fn bench_function(c: &mut Criterion) {
        c.bench_function("function", |b| b.iter(function(10)));
    }

    criterion_group!(benches, bench_function);
    criterion_main!(benches);

Obviously no system clock goes down to picoseconds, but the function
is called repeatedly, such that it can be measured accurately all the
way down to the CPU clock cycles if you needed it.
