/// Returns "first" if any heap contains an odd number of coins.
///
/// Stops immediately when an odd heap is found.
pub fn winner(heaps: &[u64]) -> &'static str {
    if heaps.iter().any(|&x| x % 2 == 1) {
        "first"
    } else {
        "second"
    }
}