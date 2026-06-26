# Another Game (CSES)

## Deliverable Objective

The objective is to determine the winner of a two-player impartial game in which players repeatedly remove one coin from any chosen subset of non-empty heaps. The implementation applies the mathematical property of the game to produce an efficient solution that satisfies the CSES time and memory constraints.

**CSES Problem:** https://cses.fi/ckvo8q5wh/task/2208/

---

# Project Structure

```text
another_game
│
├── Cargo.toml
├── Cargo.lock
├── README.md
├── input.txt
│
├── benches
│   └── benchmark.rs
│
├── tests
│   └── another_game_tests.rs
│
└── src
    ├── lib.rs
    └── main.rs
```

---

# Algorithm Overview

The game can be solved using a simple mathematical observation.

If at least one heap contains an **odd** number of coins, the first player has a winning strategy.

If every heap contains an **even** number of coins, the second player wins assuming both players play optimally.

Instead of exploring every possible move, the algorithm performs a single scan through the list of heaps and checks the parity of each value. This produces a linear-time solution while using only constant extra memory. 
Time Complexity: O(n)
Space Complexity:O(1)

# Benchmark Summary

Criterion measured the algorithm in nanoseconds, confirming that it runs efficiently with very low overhead. The benchmark reflects the expected performance of a single linear scan through the heap list.

# Benchmark Analysis

The solution has **O(n)** time complexity because each heap is checked once, and **O(1)** extra space complexity since only a few variables are used. The measured performance matches the theoretical analysis, with runtime increasing linearly as the number of heaps grows. Minor differences between benchmark runs are expected due to system load, CPU scheduling, and compiler optimizations.

# Conclusion

This implementation satisfies the CSES requirements, passes all tests, includes benchmarking with Criterion, and demonstrates an efficient mathematical solution to the problem.
