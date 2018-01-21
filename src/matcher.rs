extern crate float_cmp;

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
    arg.approx_eq_ulps(&target_val, 2)
}

/// Matcher that matches if `arg` is equal to `target_val`. This uses
/// approximate floating point equality, as defined by the `float-cmp` crate.
pub fn f64_eq(arg: &f64, target_val: f64) -> bool {
    arg.approx_eq_ulps(&target_val, 2)
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

/// Matcher that matches if `arg` contains the substring specified by `string`.
pub fn contains(arg: &str, string: &str) -> bool {
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
    arg.to_lowercase() == string
}

// ============================================================================
// * Container Matchers
// ============================================================================

// TODO


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

    // TODO: write tests for all existing matchers

}
