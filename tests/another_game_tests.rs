use another_game::winner;

// Verifies that a single odd heap gives the first player a winning strategy.
#[test]
fn one_odd_heap() {
    assert_eq!(winner(&[1]), "first");
}

// Verifies that a single even heap results in a second-player win.
#[test]
fn one_even_heap() {
    assert_eq!(winner(&[2]), "second");
}

// Tests the first sample case from the CSES problem statement.
#[test]
fn sample_case_1() {
    assert_eq!(winner(&[1, 2, 3]), "first");
}

// Tests the second sample case from the CSES problem statement.
#[test]
fn sample_case_2() {
    assert_eq!(winner(&[2, 2]), "second");
}

// Tests the third sample case from the CSES problem statement.
#[test]
fn sample_case_3() {
    assert_eq!(winner(&[5, 5, 4, 5]), "first");
}