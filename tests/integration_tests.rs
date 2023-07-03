use rust_code_wars::count_by::count_by;
use rust_code_wars::square_sum::square_sum;

#[test]
fn count_by_sample_tests() {
    assertion_count_by(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], (1, 10));
    assertion_count_by(vec![2, 4, 6, 8, 10], (2, 5));
    assertion_count_by(vec![3, 6, 9, 12, 15, 18, 21], (3, 7));
    assertion_count_by(vec![50, 100, 150, 200, 250], (50, 5));
    assertion_count_by(vec![100, 200, 300, 400, 500, 600], (100, 6));
}

fn assertion_count_by(expected: Vec<u32>, inputs: (u32, u32)) {
    let actual = count_by(inputs.0, inputs.1);

    assert_eq!(
        expected, actual,
        "\nTest failed!\n expected: [{}]\n actual: [{}]\n x: {}\n n: {}\n",
        expected.iter().map(|&n| n.to_string()).collect::<Vec<String>>().join(", "),
        actual.iter().map(|&n| n.to_string()).collect::<Vec<String>>().join(", "),
        inputs.0,
        inputs.1
    );
}

#[test]
fn square_sum_returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    assert_eq!(square_sum(vec![]), 0);
}
