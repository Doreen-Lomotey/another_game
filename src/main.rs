use std::io::{self, Read};
use another_game::winner;

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