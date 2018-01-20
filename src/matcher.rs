include!(concat!(env!("OUT_DIR"), "/matcher_generated.rs"));

/// Macro for conviniently binding matcher parameters to pattern matching
/// functions.
///
/// This macro takes a function of the form
/// `fn foo<T>(arg: &T, matcher_params...)` and a series of values for
/// `matcher_params...`. It returns a function which is bound to the
/// `matcher_params...` specified in the macro call. The returned function
/// takes a single argument, `arg`, which is the actual argument value being
/// checked.
///
/// ```
/// # #[macro_use] extern crate double;
/// use double::matcher::*;
///
/// trait TaskManager {
///    fn set_max_threads(&mut self, max_threads: u32);
/// }
///
/// mock_trait!(
///     MockTaskManager,
///     set_max_threads(u32) -> ()
/// );
///
/// impl TaskManager for MockTaskManager {
///     mock_method!(set_max_threads(&mut self, max_threads: u32));
/// }
///
/// # fn main() {
/// let mut mock = MockTaskManager::default();
///
/// mock.set_max_threads(42);
///
/// // should match:
/// assert!(mock.set_max_threads.called_with_pattern(p!(le, 42)));
/// assert!(mock.set_max_threads.called_with_pattern(p!(ge, 42)));
/// assert!(mock.set_max_threads.called_with_pattern(p!(eq, 42)));
/// // should not match:
/// assert!(!mock.set_max_threads.called_with_pattern(p!(lt, 42)));
/// assert!(!mock.set_max_threads.called_with_pattern(p!(gt, 42)));
/// # }
/// ```
#[macro_export]
macro_rules! rp {
    ( $func:ident ) => (
        &|ref potential_match| -> bool { $func(potential_match) }
    );

    ( $func:ident, $arg:expr ) => (
        &|ref potential_match| -> bool { $func(potential_match, $arg) }
    );
}

#[macro_export]
macro_rules! p {
    ( $func:ident ) => (
        &|potential_match| -> bool { $func(potential_match) }
    );

    ( $func:ident, $arg:expr ) => (
        &|potential_match| -> bool { $func(potential_match, $arg) }
    );
}


// ============================================================================
// * Comparison Matchers
// ============================================================================

/// Argument matcher that matches any arg value.
pub fn any<T>(_: &T) -> bool {
    true
}

/// Argument matcher that matches if the input `arg` is equal to `target_val`.
pub fn eq<T: PartialEq>(arg: &T, target_val: T) -> bool {
    *arg == target_val
}

/// Argument matcher that matches if the input `arg` is not equal to
/// `target_val`.
pub fn ne<T: PartialEq>(arg: &T, target_val: T) -> bool {
    *arg != target_val
}

/// Argument matcher that matches if the input `arg` is less than `target_val`.
pub fn lt<T: PartialOrd>(arg: &T, target_val: T) -> bool {
    *arg < target_val
}

/// Argument matcher that matches if the input `arg` is less than or equal to
/// `target_val`.
pub fn le<T: PartialEq + PartialOrd>(arg: &T, target_val: T) -> bool {
    *arg <= target_val
}

/// Argument matcher that matches if the input `arg` is greater than
/// `target_val`.
pub fn gt<T: PartialOrd>(arg: &T, target_val: T) -> bool {
    *arg > target_val
}

/// Argument matcher that matches if the input `arg` is greater than or equal
/// to `target_val`.
pub fn ge<T: PartialEq + PartialOrd>(arg: &T, target_val: T) -> bool {
    *arg >= target_val
}

/// Argument matcher that matches if the input `arg` is a populated `Option`
/// whose stored value matches the specified `pattern`.
pub fn is_some<T>(arg: &Option<T>, pattern: &Fn(&T) -> bool) -> bool {
    match *arg {
        Some(ref x) => pattern(x),
        None => false
    }
}

/// TODO
///
/// ```
/// # #[macro_use] extern crate double;
/// use double::matcher::*;
///
/// # fn main() {
/// let matcher = p!(is_ok, p!(ge, 50));
/// assert!( p!(is_ok, p!(ge, 50))(&Ok(42)) );
/// // assert!(!matcher(&Ok(42)));
/// // assert!(!matcher(&Ok(50)));
/// // assert!(matcher(&Ok(80)));
/// // assert!(!matcher(&Err("an error occurred")));
/// # }
/// ```
pub fn is_ok<T, U>(arg: &Result<T, U>, pattern: &Fn(&T) -> bool) -> bool {
    match *arg {
        Ok(ref x) => pattern(x),
        Err(_) => false
    }
}

/// TODO
pub fn is_err<T, U>(arg: &Result<T, U>, pattern: &Fn(&U) -> bool) -> bool {
    match *arg {
        Ok(_) => false,
        Err(ref x) => pattern(x)
    }
}


// ============================================================================
// * Float Matchers
// ============================================================================

/// Argument matcher that matches if the input `arg` is equal to `target_val`.
/// This uses approximate floating point equality, as defined by the `TODO`
/// crate.
pub fn f32_near(arg: &f32, target_val: f32) -> bool {
    // TODO
    *arg == target_val
}

/// Argument matcher that matches if the input `arg` is equal to `target_val`.
/// This uses approximate floating point equality, as defined by the `TODO`
/// crate.
pub fn f64_near(arg: &f64, target_val: f64) -> bool {
    // TODO
    *arg == target_val
}

/// Argument matcher that matches if the input `arg` is equal to `target_val`.
/// This uses approximate floating point equality, as defined by the `TODO`
/// crate.
pub fn nan_sensitive_f32_near(arg: &f32, target_val: f32) -> bool {
    // TODO
    *arg == target_val
}

/// Argument matcher that matches if the input `arg` is equal to `target_val`.
/// This uses approximate floating point equality, as defined by the `TODO`
/// crate.
pub fn nan_sensitive_f64_near(arg: &f64, target_val: f64) -> bool {
    // TODO
    *arg == target_val
}


// ============================================================================
// * Composite Matchers
// ============================================================================

/// TODO
pub fn not<T>(arg: &T, pattern: &Fn(&T) -> bool) -> bool {
    !pattern(arg)
}

/// TODO
pub fn all_of<T>(arg: &T, patterns: Vec<&Fn(&T) -> bool>) -> bool {
    for pat in patterns {
        if !pat(arg) {
            return false
        }
    }
    true
}

/// TODO
pub fn any_of<T>(arg: &T, patterns: Vec<&Fn(&T) -> bool>) -> bool {
    for pat in patterns {
        if pat(arg) {
            return true
        }
    }
    false
}

// ============================================================================
// * String Matchers
// ============================================================================

/// TODO
pub fn contains(arg: &str, substring: &str) -> bool {
    arg.contains(substring)
}

/// TODO
pub fn starts_with(arg: &str, substring: &str) -> bool {
    arg.starts_with(substring)
}

/// TODO
pub fn ends_with(arg: &str, substring: &str) -> bool {
    arg.ends_with(substring)
}

/// TODO
pub fn eq_nocase(arg: &str, string: &str) -> bool {
    arg.to_lowercase() == string
}

/// TODO
pub fn ne_nocase(arg: &str, string: &str) -> bool {
    arg.to_lowercase() == string
}

// ============================================================================
// * Container Matchers
// ============================================================================

// TODO
