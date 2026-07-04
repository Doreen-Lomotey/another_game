use another_game::early_exit_linear_solution;
use another_game::full_scan_linear_solution;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_early_exit(c: &mut Criterion) {
    let heaps = vec![1u64; 200_000];

    c.bench_function("early_exit_linear", |b| {
        b.iter(|| {
            black_box(
                early_exit_linear_solution::winner(black_box(&heaps))
            );
        })
    });
}

fn benchmark_full_scan(c: &mut Criterion) {
    let heaps = vec![1u64; 200_000];

    c.bench_function("full_scan_linear", |b| {
        b.iter(|| {
            black_box(
                full_scan_linear_solution::winner(black_box(&heaps))
            );
        })
    });
}

criterion_group!(
    benches,
    benchmark_early_exit,
    benchmark_full_scan
);

criterion_main!(benches);