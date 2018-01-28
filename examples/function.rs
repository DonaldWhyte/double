#[macro_use]
extern crate double;

use double::Mock;

fn calculate_factor(value: i32, weighting_fn: &Fn(i32) -> i32) -> i32 {
    weighting_fn(value * 2)
}

fn main() {
    let mock = Mock::<i32, i32>::default();
    mock.return_value(100);

    let result = calculate_factor(42, &mock_function!(mock, i32, i32));

    assert_eq!(100, result);
    // Input argument should have be doubled by calculate_factor(), before it
    // was passed into the weighting function.
    assert!(mock.has_calls_exactly(
        vec!(84)
    ));
}

