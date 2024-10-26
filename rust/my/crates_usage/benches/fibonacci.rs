use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci group");

    // Now we can perform benchmarks with this group.
    group.bench_function("fibonacci_number_recursive", |b| {
        b.iter(|| my_practices::fibonacci::number_recursive(black_box(20)))
    });
    group.bench_function("fibonacci_number_iterative", |b| {
        b.iter(|| my_practices::fibonacci::number_iterative(black_box(20)))
    });
    group.bench_function("fibonacci_sequence", |b| {
        b.iter(|| my_practices::fibonacci::sequence(black_box(20)))
    });

    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
