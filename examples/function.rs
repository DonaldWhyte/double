#[macro_use]
extern crate double;

use double::Mock;

fn generate_sequence(func: &Fn(i32) -> i32, min: i32, max: i32) -> Vec<i32> {
    // exclusive range
    (min..max).map(func).collect()
}

fn test_function_used_correctly() {
    // GIVEN:
    let mock = Mock::<(i32), i32>::default();
    mock.use_closure(Box::new(|x| x * 2));

    // WHEN:
    let sequence = generate_sequence(
        &mock_func!(mock, i32, i32),
        1,
        5);

    // THEN:
    assert_eq!(vec!(2, 4, 6, 8), sequence);
    assert!(mock.has_calls_exactly(vec!(
      1, 2, 3, 4
    )));
}

fn main() {
    test_function_used_correctly();
}
