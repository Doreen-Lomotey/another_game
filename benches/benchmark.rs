use another_game::winner;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Benchmarks the winner function using a large input.
///
/// Criterion measures the execution time while `black_box` prevents
/// compiler optimizations from removing the computation.
fn benchmark_winner(c: &mut Criterion) {
    // Create a large test case to measure runtime consistently.
    let heaps = vec![1u64; 200_000];

    c.bench_function("another_game_linear", |b| {
        b.iter(|| {
            black_box(winner(black_box(&heaps)));
        })
    });
}

criterion_group!(benches, benchmark_winner);
criterion_main!(benches);