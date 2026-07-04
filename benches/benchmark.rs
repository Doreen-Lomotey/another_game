/// PERFORMANCE ANALYSIS:
/// early_exit_linear: faster in practice due to short-circuit evaluation (.any())
/// full_scan_linear: slower because it always scans entire input
///
/// Both algorithms are O(n), but runtime differs due to control flow:
/// - Early exit may stop at first odd heap
/// - Full scan always processes all elements
/// This shows that Big-O does not fully describe real performance.
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use another_game::early_exit_linear_solution;
use another_game::full_scan_linear_solution;

// Benchmark 1: Early exit linear scan (short-circuit)
fn bench_early_exit(c: &mut Criterion) {
    c.bench_function("early_exit_linear", |b| b.iter(|| {
        let heaps = vec![1u64; 200_000];
        early_exit_linear_solution::winner(black_box(&heaps));
    }));
}

// Benchmark 2: Full scan linear traversal (no early exit)
fn bench_full_scan(c: &mut Criterion) {
    c.bench_function("full_scan_linear", |b| b.iter(|| {
        let heaps = vec![1u64; 200_000];
        full_scan_linear_solution::winner(black_box(&heaps));
    }));
}

criterion_group!(
    benches,
    bench_early_exit,
    bench_full_scan
);

criterion_main!(benches);