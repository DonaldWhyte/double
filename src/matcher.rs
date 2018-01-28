extern crate float_cmp;

use std::collections::HashSet;
use std::f32;
use std::f64;
use self::float_cmp::ApproxEqUlps;


include!(concat!(env!("OUT_DIR"), "/matcher_generated.rs"));


// ============================================================================
// * Comparison Matchers
// ============================================================================

/// Matcher that matches any arg value.
pub fn any<T>(_: &T) -> bool {
    true
}

/// Matcher that matches if `arg` is equal to `target_val`.
pub fn eq<T: PartialEq>(arg: &T, target_val: T) -> bool {
    *arg == target_val
}

/// Matcher that matches if `arg` is not equal to `target_val`.
pub fn ne<T: PartialEq>(arg: &T, target_val: T) -> bool {
    *arg != target_val
}

/// Matcher that matches if `arg` is less than `target_val`.
pub fn lt<T: PartialOrd>(arg: &T, target_val: T) -> bool {
    *arg < target_val
}

/// Matcher that matches if `arg` is less than or equal to `target_val`.
pub fn le<T: PartialEq + PartialOrd>(arg: &T, target_val: T) -> bool {
    *arg <= target_val
}

/// Matcher that matches if `arg` is greater than `target_val`.
pub fn gt<T: PartialOrd>(arg: &T, target_val: T) -> bool {
    *arg > target_val
}

/// Matcher that matches if `arg` is greater than or equal to `target_val`.
pub fn ge<T: PartialEq + PartialOrd>(arg: &T, target_val: T) -> bool {
    *arg >= target_val
}

/// Matcher that matches if `arg` is between the exclusive range `(low,high)`.
pub fn between_exc<T: PartialOrd>(arg: &T, low: T, high: T) -> bool {
    low < *arg && *arg < high
}

/// Matcher that matches if `arg` is between the inclusive range `[low,high]`.
pub fn between_inc<T: PartialEq + PartialOrd>(arg: &T, low: T, high: T) -> bool {
    low <= *arg && *arg <= high
}

/// Matcher that matches if `arg` is a populated `Option` whose stored value
/// matches the specified `matcher`.
pub fn is_some<T>(arg: &Option<T>, matcher: &Fn(&T) -> bool) -> bool {
    match *arg {
        Some(ref x) => matcher(x),
        None => false
    }
}

/// Matcher that matches if `arg` is a `Result::Ok` whose stored value matches
/// the specified `matcher`.
pub fn is_ok<T, U>(arg: &Result<T, U>, matcher: &Fn(&T) -> bool) -> bool {
    match *arg {
        Ok(ref x) => matcher(x),
        Err(_) => false
    }
}

/// Matcher that matches if `arg` is a `Result::Err` whose stored value matches
/// the specified `matcher`.
pub fn is_err<T, U>(arg: &Result<T, U>, matcher: &Fn(&U) -> bool) -> bool {
    match *arg {
        Ok(_) => false,
        Err(ref x) => matcher(x)
    }
}


// ============================================================================
// * Float Matchers
// ============================================================================

/// Matcher that matches if `arg` is equal to `target_val`. This uses
/// approximate floating point equality, as defined by the `float-cmp` crate.
pub fn f32_eq(arg: &f32, target_val: f32) -> bool {
    if target_val.is_nan() && arg.is_nan() {
        false
    } else {
        arg.approx_eq_ulps(&target_val, 2)
    }
}

/// Matcher that matches if `arg` is equal to `target_val`. This uses
/// approximate floating point equality, as defined by the `float-cmp` crate.
pub fn f64_eq(arg: &f64, target_val: f64) -> bool {
    if target_val.is_nan() && arg.is_nan() {
        false
    } else {
        arg.approx_eq_ulps(&target_val, 2)
    }
}

/// Matcher that matches if `arg` is equal to `target_val`. This uses
/// approximate floating point equality, as defined by the `float-cmp` crate.
///
/// Unlike `f32_eq`, this matcher returns `true` if both the actual `arg` and
/// the `target_val` are NaN.
pub fn nan_sensitive_f32_eq(arg: &f32, target_val: f32) -> bool {
    if target_val.is_nan() && arg.is_nan() {
        true
    } else {
        arg.approx_eq_ulps(&target_val, 2)
    }
}

/// Matcher that matches if `arg` is equal to `target_val`. This uses
/// approximate floating point equality, as defined by the `float-cmp` crate.
///
/// Unlike `f64_eq`, this matcher returns `true` if both the actual `arg` and
/// the `target_val` are NaN.
pub fn nan_sensitive_f64_eq(arg: &f64, target_val: f64) -> bool {
    if target_val.is_nan() && arg.is_nan() {
        true
    } else {
        arg.approx_eq_ulps(&target_val, 2)
    }
}


// ============================================================================
// * String Matchers
// ============================================================================

/// Matcher that matches if `has_substr` contains the substring specified by
/// `string`.
pub fn has_substr(arg: &str, string: &str) -> bool {
    arg.contains(string)
}

/// Matcher that matches if `arg` starts with the specified `prefix`.
pub fn starts_with(arg: &str, prefix: &str) -> bool {
    arg.starts_with(prefix)
}

/// Matcher that matches if `arg` ends with the specified `suffix`.
pub fn ends_with(arg: &str, suffix: &str) -> bool {
    arg.ends_with(suffix)
}

/// Matcher that matches if `arg` is equal to `string` after ignoring case.
pub fn eq_nocase(arg: &str, string: &str) -> bool {
    arg.to_lowercase() == string
}

/// Matcher that matches if `arg` is not equal to `string`, even after ignoring
/// case.
pub fn ne_nocase(arg: &str, string: &str) -> bool {
    arg.to_lowercase() != string
}


// ============================================================================
// * Container Matchers
// ============================================================================

// TODO: comment on intoitertor + clone thing

/// TODO
pub fn is_empty<T: Clone + IntoIterator>(arg: &T) -> bool {
    let elements: Vec<T::Item> = arg
        .clone()
        .into_iter()
        .map(|e| e.into())
        .collect();
    elements.is_empty()
}

/// TODO
pub fn is_length<T: Clone + IntoIterator>(
    arg: &T,
    matcher: &Fn(&usize) -> bool) -> bool
{
    let elements: Vec<T::Item> = arg
        .clone()
        .into_iter()
        .map(|e| e.into())
        .collect();
    matcher(&elements.len())
}

/// TODO
pub fn contains<T: Clone + IntoIterator>(
    arg: &T,
    matcher: &Fn(&T::Item) -> bool) -> bool
{
    let actual: Vec<T::Item> = arg
        .clone()
        .into_iter()
        .map(|e| e.into())
        .collect();
    for elem in actual.iter() {
        if !matcher(&elem) {
            return true;
        }
    }
    false
}

/// TODO
pub fn each<T: Clone + IntoIterator>(
    arg: &T,
    matcher: &Fn(&T::Item) -> bool) -> bool
{
    let actual: Vec<T::Item> = arg
        .clone()
        .into_iter()
        .map(|e| e.into())
        .collect();
    for elem in actual.iter() {
        if !matcher(&elem) {
            return false;
        }
    }
    true
}

/// TODO
pub fn unordered_elements_are<T: Clone + IntoIterator>(
    arg: &T,
    expected_elems: Vec<T::Item>) -> bool
    where T::Item: Eq
{
    let actual: Vec<T::Item> = arg
        .clone()
        .into_iter()
        .map(|e| e.into())
        .collect();
    if actual.len() == expected_elems.len() {
        let mut matched_indices: HashSet<usize> = HashSet::new();
        for actual_idx in 0..actual.len() {
            for expected_idx in 0..expected_elems.len() {
                if !matched_indices.contains(&expected_idx) {
                    if actual[actual_idx] == expected_elems[expected_idx] {
                        matched_indices.insert(expected_idx);
                    }
                }
            }
        }
        matched_indices.len() == actual.len()
    } else {
        false
    }
}

/// TODO
pub fn when_sorted<T: Clone + IntoIterator>(
    arg: &T,
    expected: Vec<T::Item>) -> bool
    where T::Item: Ord
{
    let mut actual: Vec<T::Item> = arg
        .clone()
        .into_iter()
        .map(|e| e.into())
        .collect();
    actual.sort();
    actual == expected
}


// ============================================================================
// * Composite Matchers
// ============================================================================

/// Matcher that matches if `arg` does _not_ match the specified `matcher`.
pub fn not<T>(arg: &T, matcher: &Fn(&T) -> bool) -> bool {
    !matcher(arg)
}

/// Matcher that matches if `arg` matches *all* of the specified `matchers`. If
/// at least one of `matchers` doesn't match with `arg`, this matcher doesn't
/// match.
pub fn all_of<T>(arg: &T, matchers: Vec<&Fn(&T) -> bool>) -> bool {
    for matcher in matchers {
        if !matcher(arg) {
            return false
        }
    }
    true
}

/// Matcher that matches if `arg` matches *any* of the specified `matchers`. If
/// none of the `matchers` match with `arg`, this matcher doesn't match.
pub fn any_of<T>(arg: &T, matchers: Vec<&Fn(&T) -> bool>) -> bool {
    for matcher in matchers {
        if matcher(arg) {
            return true
        }
    }
    false
}


// ============================================================================
// * Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_matcher() {
        assert!(any(&1));
        assert!(any(&Some(42)));
        assert!(any(&42.2));
        assert!(any(&vec!(1, 2, 3, 4, 5)));
    }

    #[test]
    fn eq_matcher() {
        let matcher1 = p!(eq, 1);
        assert!(matcher1(&1));
        assert!(!matcher1(&2));

        let matcher2 = p!(eq, "hello");
        assert!(matcher2(&"hello"));
        assert!(!matcher2(&"bye_bye"));

        let vec_arg1 = vec!(1, 2, 3, 4);
        let vec_arg2 = vec!(1, 2, 3, 5);
        let matcher3 = p!(eq, vec!(1, 2, 3, 4));
        assert!(matcher3(&vec_arg1));
        assert!(!matcher3(&vec_arg2));
    }

    #[test]
    fn ne_matcher() {
        let matcher1 = p!(ne, 1);
        assert!(!matcher1(&1));
        assert!(matcher1(&2));

        let matcher2 = p!(ne, "hello");
        assert!(!matcher2(&"hello"));
        assert!(matcher2(&"bye_bye"));

        let vec_arg1 = vec!(1, 2, 3, 4);
        let vec_arg2 = vec!(1, 2, 3, 5);
        let matcher3 = p!(ne, vec!(1, 2, 3, 4));
        assert!(!matcher3(&vec_arg1));
        assert!(matcher3(&vec_arg2));
    }

    #[test]
    fn lt_matcher() {
        let matcher1 = p!(lt, 10);
        assert!(matcher1(&9));
        assert!(!matcher1(&10));
        assert!(!matcher1(&11));

        let matcher2 = p!(lt, "hello1");
        assert!(matcher2(&"hello0"));
        assert!(!matcher2(&"hello1"));
        assert!(!matcher2(&"hello2"));
    }

    #[test]
    fn le_matcher() {
        let matcher1 = p!(le, 10);
        assert!(matcher1(&9));
        assert!(matcher1(&10));
        assert!(!matcher1(&11));

        let matcher2 = p!(le, "hello1");
        assert!(matcher2(&"hello0"));
        assert!(matcher2(&"hello1"));
        assert!(!matcher2(&"hello2"));
    }

    #[test]
    fn gt_matcher() {
        let matcher1 = p!(gt, 10);
        assert!(!matcher1(&9));
        assert!(!matcher1(&10));
        assert!(matcher1(&11));

        let matcher2 = p!(gt, "hello1");
        assert!(!matcher2(&"hello0"));
        assert!(!matcher2(&"hello1"));
        assert!(matcher2(&"hello2"));
    }

    #[test]
    fn ge_matcher() {
        let matcher1 = p!(ge, 10);
        assert!(!matcher1(&9));
        assert!(matcher1(&10));
        assert!(matcher1(&11));

        let matcher2 = p!(ge, "hello1");
        assert!(!matcher2(&"hello0"));
        assert!(matcher2(&"hello1"));
        assert!(matcher2(&"hello2"));
    }

    #[test]
    fn between_exc_matcher() {
        let matcher = p!(between_exc, 9, 11);
        assert!(!matcher(&8));
        assert!(!matcher(&9));
        assert!(matcher(&10));
        assert!(!matcher(&11));
        assert!(!matcher(&12));
    }

    #[test]
    fn between_inc_matcher() {
        let matcher = p!(between_inc, 9, 11);
        assert!(!matcher(&8));
        assert!(matcher(&9));
        assert!(matcher(&10));
        assert!(matcher(&11));
        assert!(!matcher(&12));
    }

    #[test]
    fn is_some_matcher() {
        let matcher = p!(is_some, p!(gt, 5));
        assert!(matcher(&Some(10)));
        assert!(!matcher(&Some(3)));
        assert!(!matcher(&None));
    }

    #[test]
    fn is_ok_matcher() {
        let matcher = p!(is_ok, p!(gt, 5));
        assert!(matcher(&Ok(10)));
        assert!(!matcher(&Ok(3)));
        assert!(!matcher(&Err("boo")));
    }

    #[test]
    fn is_err_matcher() {
        let matcher = p!(is_err, p!(gt, 0));
        assert!(matcher(&Err(8)));
        assert!(!matcher(&Err(0)));
        assert!(!matcher(&Ok(150.75)));
    }

    #[test]
    fn f32_eq_matcher() {
        let matcher = p!(f32_eq, 42.5572f32);
        assert!(!matcher(&0.0f32));
        assert!(!matcher(&42.0f32));
        assert!(!matcher(&42.55f32));
        assert!(matcher(&42.5572f32));

        let nan_matcher = p!(f32_eq, f32::NAN);
        assert!(!nan_matcher(&0.0f32));
        assert!(!nan_matcher(&42.0f32));
        assert!(!nan_matcher(&f32::NAN));
    }

    #[test]
    fn f64_eq_matcher() {
        let matcher = p!(f64_eq, 42.5572f64);
        assert!(!matcher(&0.0f64));
        assert!(!matcher(&42.0f64));
        assert!(!matcher(&42.55f64));
        assert!(matcher(&42.5572f64));

        let nan_matcher = p!(f64_eq, f64::NAN);
        assert!(!nan_matcher(&0.0f64));
        assert!(!nan_matcher(&42.0f64));
        assert!(!nan_matcher(&f64::NAN));
    }

    #[test]
    fn nan_sensitive_f32_eq_matcher() {
        let matcher = p!(nan_sensitive_f32_eq, 42.5572f32);
        assert!(!matcher(&0.0f32));
        assert!(!matcher(&42.0f32));
        assert!(!matcher(&42.55f32));
        assert!(matcher(&42.5572f32));

        let nan_matcher = p!(nan_sensitive_f32_eq, f32::NAN);
        assert!(!nan_matcher(&0.0f32));
        assert!(!nan_matcher(&42.0f32));
        assert!(nan_matcher(&f32::NAN));
    }

    #[test]
    fn nan_sensitive_f64_eq_matcher() {
        let matcher = p!(nan_sensitive_f64_eq, 42.5572f64);
        assert!(!matcher(&0.0f64));
        assert!(!matcher(&42.0f64));
        assert!(!matcher(&42.55f64));
        assert!(matcher(&42.5572f64));

        let nan_matcher = p!(nan_sensitive_f64_eq, f64::NAN);
        assert!(!nan_matcher(&0.0f64));
        assert!(!nan_matcher(&42.0f64));
        assert!(nan_matcher(&f64::NAN));
    }

    #[test]
    fn ne_nocase_matcher() {
        let matcher = p!(ne_nocase, "foo");
        assert!(matcher(""));
        assert!(!matcher("FOo"));
        assert!(!matcher("FOO"));
        assert!(!matcher("foo"));
        assert!(matcher("barfoo"));
        assert!(matcher("barFOO"));
    }

    #[test]
    fn is_empty_matcher() {
        let empty_vec: Vec<i32> = vec!();
        assert!(is_empty(&empty_vec));
        assert!(!is_empty(&vec!(42)));
        assert!(!is_empty(&vec!(42, 84, 100)));
    }

    /*#[test]
    fn is_length_matcher() {
        let matcher = p!(is_length, p!(gt, 1usize));
        let empty_vec: Vec<i32> = vec!();
        assert!(!matcher(&empty_vec));
        assert!(!matcher(&vec!(1)));
        assert!(matcher(&vec!(1, 2)));
        assert!(matcher(&vec!(1, 2, 3)));
    }*/

    #[test]
    fn contains_matcher() {
        let matcher = p!(contains, p!(ge,   5));
        assert!(!matcher(&vec!(1, 2, 3)));  // 0 matches
        assert!(matcher(&vec!(1, 5, 3)));   // 1 match
        assert!(matcher(&vec!(5, 6, 7)));   // > 1 matches
    }
/*
    #[test]
    fn each_matcher() {
        let matcher = p!(each, p!(ge, 5));
        assert!(!matcher(&vec!(1, 2, 3)));  // 0 matches
        assert!(!matcher(&vec!(1, 5, 3)));   // 1 match
        assert!(matcher(&vec!(5, 6, 7)));   // all matches
    }


unordered_elements_are
    - different number of elements
    - same number of elements, diff values
    - same number, same values, diff order
    - same number, same values, same order
when_sorted
    - already sorted, equal
    - not sorted, equal
    - already sorted, not equal
    - not sorted, not equal
*/

    #[test]
    fn has_substr_matcher() {
        let empty_matcher = p!(has_substr, "");
        assert!(empty_matcher(""));
        assert!(empty_matcher("foo"));
        assert!(empty_matcher("barfooban"));
        assert!(empty_matcher("ban"));

        let matcher = p!(has_substr, "foo");
        assert!(!matcher(""));
        assert!(matcher("foo"));
        assert!(matcher("barfooban"));
        assert!(!matcher("ban"));
    }

    #[test]
    fn starts_with_matcher() {
        let empty_matcher = p!(starts_with, "");
        assert!(empty_matcher(""));
        assert!(empty_matcher("foo"));
        assert!(empty_matcher("barfooban"));
        assert!(empty_matcher("ban"));

        let matcher = p!(starts_with, "foo");
        assert!(!matcher(""));
        assert!(matcher("foo"));
        assert!(!matcher("barfooban"));
        assert!(!matcher("ban"));
    }
    #[test]
    fn ends_with_matcher() {
        let empty_matcher = p!(ends_with, "");
        assert!(empty_matcher(""));
        assert!(empty_matcher("foo"));
        assert!(empty_matcher("barfooban"));
        assert!(empty_matcher("ban"));

        let matcher = p!(ends_with, "ban");
        assert!(!matcher(""));
        assert!(!matcher("banfoo"));
        assert!(matcher("barfooban"));
        assert!(matcher("ban"));
    }

    #[test]
    fn eq_nocase_matcher() {
        let matcher = p!(eq_nocase, "foo");
        assert!(!matcher(""));
        assert!(matcher("FOo"));
        assert!(matcher("FOO"));
        assert!(matcher("foo"));
        assert!(!matcher("barfoo"));
        assert!(!matcher("barFOO"));
    }

    #[test]
    fn not_matcher() {
        let matcher = p!(not, p!(eq, 10));
        assert!(matcher(&0));
        assert!(matcher(&5));
        assert!(!matcher(&10));
        assert!(matcher(&15));
    }

    #[test]
    fn all_of_matcher() {
        let matcher = p!(all_of, vec!(
            p!(ge, 0),
            p!(le, 10)
        ));
        assert!(!matcher(&-5));
        assert!(matcher(&0));
        assert!(matcher(&5));
        assert!(matcher(&10));
        assert!(!matcher(&15));
    }

    #[test]
    fn any_of_matcher() {
        let matcher = p!(any_of, vec!(
            p!(eq, 26),
            p!(le, 40)
        ));
        assert!(matcher(&0));    // matches one
        assert!(matcher(&26));   // matches both
        assert!(matcher(&30));   // matches one
        assert!(!matcher(&42));  // matches none
    }

}
