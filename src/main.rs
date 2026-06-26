use std::io::{self, Read};

/// Determines the winner of the game.
///
/// Returns "first" if any heap contains an odd number of coins.
/// Otherwise, returns "second".
fn winner(heaps: &[u64]) -> &'static str {
    // Check whether at least one heap has an odd number of coins.
    if heaps.iter().any(|&x| x % 2 == 1) {
        "first"
    } else {
        "second"
    }
}

fn main() {
    // Read the complete input from standard input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Split the input into whitespace-separated values.
    let mut iter = input.split_whitespace();

    // Read the number of test cases.
    let t: usize = iter.next().unwrap().parse().unwrap();

    // Process each test case independently.
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();

        // Store the heap sizes.
        let mut heaps = Vec::with_capacity(n);

        for _ in 0..n {
            let x: u64 = iter.next().unwrap().parse().unwrap();
            heaps.push(x);
        }

        // Print the winner for the current test case.
        println!("{}", winner(&heaps));
    }
}