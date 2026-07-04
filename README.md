# Another Game (CSES)

## Deliverable Objective

This project solves the CSES “Another Game” problem, which determines the winner of a two-player impartial game based on the parity of heap sizes.

The rule is:

- If at least one heap contains an **odd** number of coins → **first player wins**
- If all heaps contain **even** numbers → **second player wins**

The solution is derived from a mathematical observation and does not require simulating game moves.

---

## Project Structure

```text
another_game
│
├── Cargo.toml
├── Cargo.lock
├── README.md
│
├── benches
│   └── benchmark.rs
│
├── tests
│   └── another_game_tests.rs
│
└── src
    ├── main.rs
    ├── lib.rs
    ├── early_exit_linear_solution.rs
    ├── full_scan_linear_solution.rs

---

#  Algorithm Overview

Instead of exploring every possible move, the algorithm performs a single scan through the list of heaps and checks the parity of each value. This produces a linear-time solution while using only constant extra memory. 
Time Complexity: O(n)
Space Complexity:O(1)  
 
 1. Early Exit Linear Scan (Short-Circuit Evaluation)

This approach stops immediately when an odd heap is found.

Uses iterator .any() for short-circuit behavior
Avoids unnecessary computation in best-case scenarios
Time Complexity: O(n) worst case, often faster in practice

2. Full Scan Linear Traversal

This approach checks every heap even after detecting an odd value.
Iterates through the entire array
Does not terminate early
Time Complexity: O(n) always


# Complexity Analysis:

Early Exit Scan:
Time Complexity: O(n)
Space Complexity: O(1)

Full Scan Traversal:
Time Complexity: O(n)
Space Complexity: O(1)

# Benchmark

Criterion benchmarks show that the early exit version is faster in practice due to short-circuiting, while the full scan always processes the entire input.

#  Test

The project includes unit tests covering:

Single heap cases
Even-only heap cases

# Conclusion

Both implementations are correct and run in linear time. The early exit version performs better in practice, while the full scan provides consistent behavior across all inputs.