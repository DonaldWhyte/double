extern crate double;

use double::Mock;

fn calculate_factor(value: i32, weighting_fn: &Fn(i32) -> i32) -> i32 {
    weighting_fn(value * 2)
}

fn main() {
    let mock_weighting_fn = Mock::<i32, i32>::default();
    mock_weighting_fn.return_value(100);

    let result = calculate_factor(42, &|x: i32| mock_weighting_fn.call(x));

    assert_eq!(100, result);
    assert!(mock_weighting_fn.has_calls_exactly(
        vec!(84)  // input arg should be doubled by calculate_factor()
    ));
}

