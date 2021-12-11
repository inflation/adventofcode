use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! run {
    ($c:tt, $year:literal, $day:literal) => {
        paste::paste! {
            $c.bench_function(&format!("Advent of Code Year {} Day {}", $year, $day), |b| {
                let input = include_bytes!(concat!("../input/", $year, "/day", $day, ".txt"));

                b.iter(||adventofcode::[<y $year>]::[<day $day>]::solve(black_box(input)))
            })
        }
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    run!(c, 2021, 11);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
