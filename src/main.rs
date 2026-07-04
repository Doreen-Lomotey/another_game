use std::io::{self, Read};

/// Returns "first" if any heap contains an odd number of coins.
fn winner(heaps: &[u64]) -> &'static str {
    if heaps.iter().any(|&x| x % 2 == 1) {
        "first"
    } else {
        "second"
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();

        let mut heaps = Vec::with_capacity(n);

        for _ in 0..n {
            let x: u64 = iter.next().unwrap().parse().unwrap();
            heaps.push(x);
        }

        println!("{}", winner(&heaps));
    }
}