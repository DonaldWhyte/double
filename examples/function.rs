#[macro_use]
extern crate double;

fn generate_sequence(func: &Fn(i32) -> i32, min: i32, max: i32) -> Vec<i32> {
    // exclusive range
    (min..max).map(func).collect()
}

fn test_function_used_correctly() {
    // GIVEN:
    mock_func!(
        mock,
        mock_fn,
        i32,   // return value type
        i32);  // argument1 type
    mock.use_closure(Box::new(|x| x * 2));

    // WHEN:
    let sequence = generate_sequence(&mock_fn, 1, 5);

    // THEN:
    assert_eq!(vec!(2, 4, 6, 8), sequence);
    assert!(mock.has_calls_exactly_in_order(vec!(
      1, 2, 3, 4
    )));
}

fn test_function_with_custom_defaults() {
    // GIVEN:
    mock_func_no_default!(
        mock,
        mock_fn,
        i32,   // return value type
        42,    // default return value
        i32);  // argument1 type
    mock.use_closure_for(3, Box::new(|x| x * 2));

    // WHEN:
    let sequence = generate_sequence(&mock_fn, 1, 5);

    // THEN:
    assert_eq!(vec!(42, 42, 6, 42), sequence);
    assert!(mock.has_calls_exactly_in_order(vec!(
      1, 2, 3, 4
    )));
}

fn main() {
    test_function_used_correctly();
    test_function_with_custom_defaults();
}
