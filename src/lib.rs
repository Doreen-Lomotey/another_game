pub mod early_exit_linear_solution;
pub mod full_scan_linear_solution;

// Export the early-exit solution as the default solution.
pub use early_exit_linear_solution::winner;