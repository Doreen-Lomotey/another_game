/// Returns "first" if any heap contains an odd number of coins.
///
/// Scans every heap, even if an odd heap has already been found.
pub fn winner(heaps: &[u64]) -> &'static str {
    let mut odd_found = false;

    for &heap in heaps {
        if heap % 2 == 1 {
            odd_found = true;
        }
    }

    if odd_found {
        "first"
    } else {
        "second"
    }
}