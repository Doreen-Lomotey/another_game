/// Determines the winner of the game.
///
/// Returns "first" if any heap contains an odd number of coins.
/// Otherwise, returns "second".
pub fn winner(heaps: &[u64]) -> &'static str {
    // Check whether at least one heap has an odd number of coins.
    if heaps.iter().any(|&x| x % 2 == 1) {
        "first"
    } else {
        "second"
    }
}