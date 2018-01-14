extern crate lazysort;

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::iter::FromIterator;
use std::rc::Rc;
use self::lazysort::SortedBy;

type Ref<T> = Rc<RefCell<T>>;
type OptionalRef<T> = Rc<RefCell<Option<T>>>;

/// Used for tracking function call arguments and specifying a predetermined
/// return value or mock function.
///
/// See the crate documentation for more substantial examples, including some
/// that demonstrate how to use `Mock` for methods that have multiple arguments
/// as well as methods with argument or return types that do not implement
/// `Clone`.
#[derive(Clone)]
pub struct Mock<C, R>
    where C: Clone + Eq + Hash,
          R: Clone
{
    // Ordered from lowest precedence to highest
    default_return_value: Ref<R>,
    return_value_sequence: Ref<Vec<R>>,
    default_fn: OptionalRef<fn(C) -> R>,
    default_closure: OptionalRef<Box<Fn(C) -> R>>,
    return_values: Ref<HashMap<C, R>>,
    fns: Ref<HashMap<C, fn(C) -> R>>,
    closures: Ref<HashMap<C, Box<Fn(C) -> R>>>,

    calls: Ref<Vec<C>>,
}

impl<C, R> Mock<C, R>
    where C: Clone + Eq + Hash,
          R: Clone
{
    /// Creates a new `Mock` that will return `return_value`.
    pub fn new<T: Into<R>>(return_value: T) -> Self {
        Mock {
            default_return_value: Ref::new(RefCell::new(return_value.into())),
            return_value_sequence: Ref::new(RefCell::new(Vec::new())),
            default_fn: OptionalRef::new(RefCell::new(None)),
            default_closure: OptionalRef::new(RefCell::new(None)),
            return_values: Ref::new(RefCell::new(HashMap::new())),
            fns: Ref::new(RefCell::new(HashMap::new())),
            closures: Ref::new(RefCell::new(HashMap::new())),
            calls: Ref::new(RefCell::new(vec![])),
        }
    }

    /// Use the `Mock` to return a value, keeping track of the arguments used.
    ///
    /// If specific behaviour has been configured for a specific set of
    /// arguments, this will return (in this order of precedence):
    ///     1. the return value returned by the configured closure
    ///     2. the return value returned by the configured function
    ///     3. the configured return value
    /// If no specific behaviour has been configured for the input argument set,
    /// the mock falls back to default behaviour, in this order of precedence:
    ///     1. the return value returned by the default closure (if configured)
    ///     2. the return value returned by the default function (if configured)
    ///     3. next return value in default sequence (if sequence is not empty)
    ///     4. the default return value (always configured)
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<&str, _>::new("return value");
    /// assert_eq!(mock.call("something"), "return value");
    ///
    /// mock.return_value("different value");
    /// assert_eq!(mock.call("something"), "different value");
    ///
    /// mock.return_values(vec!("one", "two"));
    /// assert_eq!(mock.call("something"), "one");
    /// assert_eq!(mock.call("something"), "two");
    /// assert_eq!(mock.call("something"), "different value");
    ///
    /// mock.use_fn(str::trim);
    /// assert_eq!(mock.call("  test  "), "test");
    ///
    /// mock.use_closure(Box::new(|x| x.trim_left()));
    /// assert_eq!(mock.call("  test  "), "test  ");
    ///
    /// mock.use_fn(str::trim);
    /// assert_eq!(mock.call("  test  "), "test");
    ///
    /// mock.return_value_for("  banana", "tasty");
    /// assert_eq!(mock.call("  banana"), "tasty");
    ///
    /// mock.use_fn_for("  banana  ", str::trim);
    /// assert_eq!(mock.call("  banana  "), "banana");
    ///
    /// mock.use_closure_for("  banana  ", Box::new(|x| x.trim_left()));
    /// assert_eq!(mock.call("  banana  "), "banana  ");
    /// ```
    pub fn call(&self, args: C) -> R {
        self.calls.borrow_mut().push(args.clone());

        if let Some(ref closure) = self.closures.borrow().get(&args) {
            return closure(args)
        } else if let Some(ref function) = self.fns.borrow().get(&args) {
            return function(args)
        } else if let Some(return_value) = self.return_values.borrow().get(&args) {
            return return_value.clone()
        } else if let Some(ref default_fn) = *self.default_fn.borrow() {
            return default_fn(args);
        } else if let Some(ref default_closure) = *self.default_closure.borrow() {
            return default_closure(args);
        } else {
            // If there are no return values in the value sequence left, fall
            // back to the configured default value.
            let ref mut sequence = *self.return_value_sequence.borrow_mut();
            match sequence.pop() {
                Some(return_value) => return_value,
                None => self.default_return_value.borrow().clone()
            }
        }
    }

    /// Override the default return value.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<&str, &str>::new("original value");
    /// mock.return_value("new value");
    ///
    /// assert_eq!(mock.call("something"), "new value");
    /// ```
    pub fn return_value<T: Into<R>>(&self, value: T) {
        *self.default_return_value.borrow_mut() = value.into();
    }

    /// Provide a sequence of default return values. The specified are returned
    /// in the same order they are specified in `values`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<&str, &str>::new("default");
    /// mock.return_values(vec!("one", "two"));
    ///
    /// assert_eq!(mock.call("hello"), "one");
    /// assert_eq!(mock.call("bye"), "two");
    /// // ran out of values in the sequence, fall back to the default value
    /// assert_eq!(mock.call("farewell"), "default");
    /// ```
    pub fn return_values<T: Into<R>>(&self, values: Vec<T>) {
        // Reverse so efficient back pop() can be used to extract  the next
        // value in the sequence
        *self.return_value_sequence.borrow_mut() = values
            .into_iter()
            .map(|r| r.into())
            .rev()
            .collect();
    }

    /// Override the return value for a specific set of call arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<&str, &str>::new("original value");
    /// mock.return_value("new value");
    /// mock.return_value_for("banana", "tasty");
    ///
    /// assert_eq!(mock.call("something"), "new value");
    /// assert_eq!(mock.call("banana"), "tasty");
    /// ```
    pub fn return_value_for<S: Into<C>, T: Into<R>>(&self, args: S, return_value: T) {
        self.return_values.borrow_mut().insert(
            args.into(),
            return_value.into());
    }

    /// Specify a function to determine the `Mock`'s return value based on
    /// the arguments provided to `Mock::call`.
    ///
    /// Arguments of `Mock::call` are still tracked.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// fn add_two(x: i64) -> i64 {
    ///     x + 2
    /// }
    ///
    /// let mock = Mock::<i64, i64>::new(10);
    /// mock.use_fn(add_two);
    ///
    /// assert_eq!(mock.call(1), 3);
    /// assert_eq!(mock.call(10), 12);
    /// ```
    ///
    /// For functions with multiple arguments, use a tuple:
    ///
    /// ```
    /// use double::Mock;
    ///
    /// fn add((x, y, z): (i64, i64, i64)) -> i64 {
    ///     x + y + z
    /// }
    ///
    /// let mock = Mock::<(i64, i64, i64), i64>::default();
    /// mock.use_fn(add);
    ///
    /// assert_eq!(mock.call((1, 1, 1)), 3);
    /// assert_eq!(mock.call((1, 2, 3,)), 6);
    /// ```
    pub fn use_fn(&self, default_fn: fn(C) -> R) {
        *self.default_closure.borrow_mut() = None;
        *self.default_fn.borrow_mut() = Some(default_fn)
    }

    /// Specify a function to determine the `Mock`'s return value based on
    /// the arguments provided to `Mock::call`. This function will only be
    /// invoked if the arguments match the specified `args`.
    ///
    /// Arguments of `Mock::call` are still tracked.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// fn add_two(x: i64) -> i64 {
    ///     x + 2
    /// }
    ///
    /// let mock = Mock::<i64, i64>::new(10);
    /// mock.return_value(42);
    /// mock.use_fn_for(5, add_two);
    ///
    /// assert_eq!(mock.call(1), 42);  // uses default value
    /// assert_eq!(mock.call(5), 7);   // uses function since args match
    /// ```
    ///
    /// For functions with multiple arguments, use a tuple:
    ///
    /// ```
    /// use double::Mock;
    ///
    /// fn add((x, y, z): (i64, i64, i64)) -> i64 {
    ///     x + y + z
    /// }
    ///
    /// let mock = Mock::<(i64, i64, i64), i64>::default();
    /// mock.return_value(42);
    /// mock.use_fn_for((1, 2, 3), add);
    ///
    /// assert_eq!(mock.call((1, 1, 1)), 42);
    /// assert_eq!(mock.call((1, 2, 3)), 6);
    /// ```
    pub fn use_fn_for<T: Into<C>>(&self, args: T, function: fn(C) -> R) {
        self.fns.borrow_mut().insert(args.into(), function);
    }

    /// Specify a closure to determine the `Mock`'s return value based on
    /// the arguments provided to `Mock::call`.
    ///
    /// Arguments of `Mock::call` are still tracked.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<i64, i64>::new(10);
    /// let add_two = |x| x + 2;
    /// mock.use_closure(Box::new(add_two));
    ///
    /// assert_eq!(mock.call(1), 3);
    /// assert_eq!(mock.call(10), 12);
    /// ```
    ///
    /// For functions with multiple arguments, use a tuple:
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(i64, i64, i64), i64>::default();
    /// let add = |(x, y, z)| x + y + z;
    /// mock.use_closure(Box::new(add));
    ///
    /// assert_eq!(mock.call((1, 1, 1)), 3);
    /// assert_eq!(mock.call((1, 2, 3,)), 6);
    /// ```
    pub fn use_closure(&self, default_fn: Box<Fn(C) -> R>) {
        *self.default_fn.borrow_mut() = None;
        *self.default_closure.borrow_mut() = Some(default_fn)
    }

    /// Specify a closure to determine the `Mock`'s return value based on
    /// the arguments provided to `Mock::call`. This closure will only be
    /// invoked if the arguments match the specified `args`.
    ///
    /// Arguments of `Mock::call` are still tracked.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<i64, i64>::new(10);
    /// let add_two = |x| x + 2;
    /// mock.return_value(42);
    /// mock.use_closure_for(10, Box::new(add_two));
    ///
    /// assert_eq!(mock.call(1), 42);
    /// assert_eq!(mock.call(10), 12);
    /// ```
    ///
    /// For functions with multiple arguments, use a tuple:
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(i64, i64, i64), i64>::default();
    /// let add = |(x, y, z)| x + y + z;
    /// mock.return_value(42);
    /// mock.use_closure_for((1, 2, 3), Box::new(add));
    ///
    /// assert_eq!(mock.call((1, 1, 1)), 42);
    /// assert_eq!(mock.call((1, 2, 3)), 6);
    /// ```
    pub fn use_closure_for<T: Into<C>>(&self, args: T, function: Box<Fn(C) -> R>) {
        self.closures.borrow_mut().insert(args.into(), function);
    }

    /// Returns true if `Mock::call` has been called.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<i64, ()>::default();
    ///
    /// assert!(!mock.called());
    ///
    /// mock.call(10);
    ///
    /// assert!(mock.called());
    /// ```
    pub fn called(&self) -> bool {
        !self.calls.borrow().is_empty()
    }

    /// Returns the number of times `Mock::call` has been called.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<i64, i64>::new(0);
    ///
    /// assert_eq!(mock.num_calls(), 0);
    /// mock.call(5);
    /// assert_eq!(mock.num_calls(), 1);
    /// mock.call(10);
    /// assert_eq!(mock.num_calls(), 2);
    /// ```
    pub fn num_calls(&self) -> usize {
        self.calls.borrow().len()
    }

    /// Returns the arguments to `Mock::call` in order from first to last.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<&str, &str>::new("");
    ///
    /// mock.call("first");
    /// mock.call("second");
    /// mock.call("third");
    ///
    /// assert_eq!(mock.calls().as_slice(), ["first", "second", "third"]);
    /// ```
    pub fn calls(&self) -> Vec<C> {
        self.calls.borrow().clone()
    }

    /// Reset the call history for the `Mock`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<&str, &str>::default();
    ///
    /// mock.call("first");
    /// mock.call("second");
    ///
    /// assert!(mock.called());
    /// assert_eq!(mock.num_calls(), 2);
    /// assert!(mock.called_with("first"));
    /// assert!(mock.called_with("second"));
    ///
    /// mock.reset_calls();
    ///
    /// assert!(!mock.called());
    /// assert_eq!(mock.num_calls(), 0);
    /// assert!(!mock.called_with("first"));
    /// assert!(!mock.called_with("second"));
    /// ```
    pub fn reset_calls(&self) {
        self.calls.borrow_mut().clear()
    }
}

impl<C, R> Default for Mock<C, R>
    where C: Clone + Eq + Hash,
          R: Clone + Default
{
    /// Use `R::default()` as the initial return value.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<i64, i64>::default();
    /// assert_eq!(mock.call(10), 0);
    ///
    /// let mock = Mock::<(), String>::default();
    /// assert_eq!(&mock.call(()), "");
    ///
    /// let mock = Mock::<(i64, &str), Option<bool>>::default();
    /// assert_eq!(mock.call((10, "test")), None);
    /// ```
    fn default() -> Self {
        Self::new(R::default())
    }
}

impl<C, R> Mock<C, R>
    where C: Clone + Debug + Eq + Hash,
          R: Clone
{
    // ========================================================================
    // * Exact Argument Checks
    // ========================================================================

    /// Returns true if the specified argument has been used for `Mock::call`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<&str, ()>::new(());
    /// mock.call("foo");
    /// mock.call("bar");
    ///
    /// assert!(mock.called_with("foo"));
    /// assert!(mock.called_with("bar"));
    /// assert!(!mock.called_with("baz"));
    /// ```
    pub fn called_with<T: Into<C>>(&self, args: T) -> bool {
        let expected_calls: Vec<T> = vec!(args);
        self.get_match_info(expected_calls).expectations_matched()
    }

    /// Returns true if `Mock::call` has been called with all of the specified
    /// `calls`. The calls can be made in any order.  They don't have to be in
    /// the order specified by `calls`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<&str, ()>::new(());
    /// mock.call("foo");
    /// mock.call("bar");
    ///
    /// let expected_calls1 = vec!("foo", "bar");
    /// assert!(mock.has_calls(expected_calls1));
    /// let expected_calls2 = vec!("bar", "foo");
    /// assert!(mock.has_calls(expected_calls2));
    /// let expected_calls3 = vec!("foo");
    /// assert!(mock.has_calls(expected_calls3));
    /// let expected_calls4 = vec!("not_in_calls");
    /// assert!(!mock.has_calls(expected_calls4));
    /// let expected_calls5 = vec!("foo", "not_in_calls");
    /// assert!(!mock.has_calls(expected_calls5));
    /// ```
    pub fn has_calls<T: Into<C>>(&self, calls: Vec<T>) -> bool {
        self.get_match_info(calls).expectations_matched()
    }

    /// Returns true if `Mock::call` has been called with all of the specified
    /// `calls`. The `calls` must be made in the order they are specified in
    /// the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(i32, i32), ()>::new(());
    /// mock.call((42, 0));
    /// mock.call((42, 1));
    /// mock.call((42, 0));  // called with same args as first call!
    ///
    /// assert!(mock.has_calls_in_order(vec!( (42, 0) )));
    /// assert!(mock.has_calls_in_order(vec!( (42, 1) )));
    /// assert!(mock.has_calls_in_order(vec!( (42, 0), (42, 1) )));
    /// assert!(mock.has_calls_in_order(vec!( (42, 1), (42, 0) )));
    /// assert!(mock.has_calls_in_order(vec!( (42, 0), (42, 1), (42, 0) )));
    /// assert!(!mock.has_calls_in_order(vec!( (42, 0), (42, 0), (42, 1) )));
    /// assert!(!mock.has_calls_in_order(vec!( (84, 0) )));
    /// assert!(!mock.has_calls_in_order(vec!( (42, 0), (84, 0) )));
    /// ```
    pub fn has_calls_in_order<T: Into<C>>(&self, calls: Vec<T>) -> bool {
        self.get_match_info(calls).expectations_matched_in_order()
    }

    /// Returns true if `Mock::call` has been called with all of the specified
    /// `calls` and it has not been called any other times. The calls can be
    /// made in any order. They don't have to be in the order specified by
    /// `calls`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(i32, i32), ()>::new(());
    /// mock.call((42, 0));
    /// mock.call((42, 1));
    /// mock.call((42, 0));
    ///
    /// assert!(!mock.has_calls_exactly(vec!( (42, 0) )));
    /// assert!(!mock.has_calls_exactly(vec!( (42, 1) )));
    /// assert!(!mock.has_calls_exactly(vec!( (84, 0) )));
    /// assert!(!mock.has_calls_exactly(vec!( (42, 0), (42, 1) )));
    /// assert!(!mock.has_calls_exactly(vec!( (42, 1), (42, 0) )));
    /// assert!(mock.has_calls_exactly(vec!( (42, 0), (42, 0), (42, 1) )));
    /// assert!(mock.has_calls_exactly(vec!( (42, 0), (42, 1), (42, 0) )));
    /// assert!(!mock.has_calls_exactly(vec!( (42, 0), (42, 1), (84, 0) )));
    /// ```
    pub fn has_calls_exactly<T: Into<C>>(&self, calls: Vec<T>) -> bool {
        self.get_match_info(calls).expectations_matched_exactly()
    }

    /// Returns true if `Mock::call` has been called with all of the specified
    /// `calls` and it has not been called any other times. The calls must be
    /// made in the order they are specified in `calls`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<&str, ()>::new(());
    /// mock.call("foo");
    /// mock.call("bar");
    ///
    /// let expected_calls1 = vec!("foo", "bar");
    /// assert!(mock.has_calls_exactly_in_order(expected_calls1));
    /// let expected_calls2 = vec!("bar", "foo");
    /// assert!(!mock.has_calls_exactly_in_order(expected_calls2));
    /// let expected_calls3 = vec!("foo");
    /// assert!(!mock.has_calls_exactly_in_order(expected_calls3));
    /// let expected_calls4 = vec!("bar");
    /// assert!(!mock.has_calls_exactly_in_order(expected_calls4));
    pub fn has_calls_exactly_in_order<T: Into<C>>(&self, calls: Vec<T>) -> bool {
        self.get_match_info(calls).expectations_matched_in_order_exactly()
    }

    // ========================================================================
    // * Pattern Matching Argument Checks
    // ========================================================================

    // There are apparently plans for the Rust compiler to support associated
    // types in concrete `impl`s. This would allow the matcher function
    // signature to be aliased, like below:
    //
    // type Matcher = Fn(&C) -> bool;
    //
    // TODO: define the above type alias when possible and use that instead of
    // explicitly defining the function signature everywhere.

    /// Returns true if an argument set passed into `Mock::call` matches the
    /// specified `pattern`.
    ///
    /// TODO: explain what pattern is, or link to a place that does
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(i32, i32), ()>::new(());
    /// mock.call((42, 0));
    /// mock.call((42, 1));
    ///
    /// let pattern1 = |args: &(i32, i32)| args.0 == 42 && args.1 != 0;
    /// let pattern2 = |args: &(i32, i32)| args.0 == 42 && args.1 == 0;
    /// let pattern3 = |args: &(i32, i32)| args.0 == 84;
    ///
    /// assert!(mock.called_with_pattern(&pattern1));
    /// assert!(mock.called_with_pattern(&pattern2));
    /// assert!(!mock.called_with_pattern(&pattern3));
    /// ```
    pub fn called_with_pattern(&self, pattern: &Fn(&C) -> bool) -> bool {
        let patterns: Vec<&Fn(&C) -> bool> = vec!(pattern);
        self.get_match_info_pat(patterns).expectations_matched()
    }

    /// Returns true if `Mock::call` has been called with all of the specified
    /// `patterns`. The calls can be made in any order. They don't have to be
    /// in the order specified by `patterns`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(i32, i32), ()>::new(());
    /// mock.call((42, 0));
    /// mock.call((42, 1));
    ///
    /// let pattern1 = |args: &(i32, i32)| args.0 == 42 && args.1 != 0;
    /// let pattern2 = |args: &(i32, i32)| args.0 == 42 && args.1 == 0;
    /// let pattern3 = |args: &(i32, i32)| args.0 == 84;
    ///
    /// assert!(mock.has_patterns(vec!(&pattern1)));
    /// assert!(mock.has_patterns(vec!(&pattern2)));
    /// assert!(mock.has_patterns(vec!(&pattern1, &pattern2)));
    /// assert!(mock.has_patterns(vec!(&pattern2, &pattern1)));
    /// assert!(!mock.has_patterns(vec!(&pattern3)));
    /// assert!(!mock.has_patterns(vec!(&pattern1, &pattern3)));
    /// ```
    pub fn has_patterns(&self, patterns: Vec<&Fn(&C) -> bool>) -> bool {
        self.get_match_info_pat(patterns).expectations_matched()
    }

    /// Returns true if `Mock::call` has been called with all of the specified
    /// `patterns`. The `patterns` must be made in the order they are specified
    /// in the input vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(i32, i32), ()>::new(());
    /// mock.call((42, 0));
    /// mock.call((42, 1));
    /// mock.call((42, 0));  // called with same args as first call!
    ///
    /// let pattern1 = |args: &(i32, i32)| args.0 == 42 && args.1 != 0;
    /// let pattern2 = |args: &(i32, i32)| args.0 == 42 && args.1 == 0;
    /// let pattern3 = |args: &(i32, i32)| args.0 == 84;
    ///
    /// assert!(mock.has_patterns_in_order(vec!(&pattern1)));
    /// assert!(mock.has_patterns_in_order(vec!(&pattern2)));
    /// assert!(mock.has_patterns_in_order(vec!(&pattern1, &pattern2)));
    /// assert!(mock.has_patterns_in_order(vec!(&pattern2, &pattern1)));
    /// assert!(mock.has_patterns_in_order(vec!(&pattern1, &pattern2, &pattern1)));
    /// assert!(!mock.has_patterns_in_order(vec!(&pattern1, &pattern1, &pattern2)));
    /// assert!(!mock.has_patterns_in_order(vec!(&pattern3)));
    /// assert!(!mock.has_patterns_in_order(vec!(&pattern1, &pattern3)));
    /// ```
    pub fn has_patterns_in_order(&self, patterns: Vec<&Fn(&C) -> bool>) -> bool {
        self.get_match_info_pat(patterns).expectations_matched_in_order()
    }

    /// Returns true if `Mock::call` has been called with all of the specified
    /// `patterns` and it has not been called any other times. The calls can be
    /// made in any order. They don't have to be in the order specified by
    /// `patterns`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(i32, i32), ()>::new(());
    /// mock.call((42, 0));
    /// mock.call((42, 1));
    /// mock.call((42, 0));
    ///
    /// let pattern1 = |args: &(i32, i32)| args.0 == 42 && args.1 != 0;
    /// let pattern2 = |args: &(i32, i32)| args.0 == 42 && args.1 == 0;
    /// let pattern3 = |args: &(i32, i32)| args.0 == 84;
    ///
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern1)));
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern2)));
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern3)));
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern1, &pattern2)));
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern2, &pattern1)));
    /// assert!(mock.has_patterns_exactly(vec!(&pattern1, &pattern1, &pattern2)));
    /// assert!(mock.has_patterns_exactly(vec!(&pattern1, &pattern2, &pattern1)));
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern1, &pattern2, &pattern3)));
    /// ```
    pub fn has_patterns_exactly(&self, patterns: Vec<&Fn(&C) -> bool>) -> bool {
        self.get_match_info_pat(patterns).expectations_matched_exactly()
    }

    /// Returns true if `Mock::call` has been called with all of the specified
    /// `patterns` and it has not been called any other times. The calls must
    /// be made match the patterns in the same order as specified in the
    /// `patterns` vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(i32, i32), ()>::new(());
    /// mock.call((42, 0));
    /// mock.call((42, 1));
    /// mock.call((42, 0));
    ///
    /// let pattern1 = |args: &(i32, i32)| args.0 == 42 && args.1 != 0;
    /// let pattern2 = |args: &(i32, i32)| args.0 == 42 && args.1 == 0;
    /// let pattern3 = |args: &(i32, i32)| args.0 == 84;
    ///
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern1)));
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern2)));
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern3)));
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern1, &pattern2)));
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern2, &pattern1)));
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern1, &pattern1, &pattern2)));
    /// assert!(mock.has_patterns_exactly(vec!(&pattern1, &pattern2, &pattern1)));
    /// assert!(!mock.has_patterns_exactly(vec!(&pattern1, &pattern2, &pattern3)));
    /// ```
    pub fn has_patterns_exactly_in_order(&self, patterns: Vec<&Fn(&C) -> bool>) -> bool {
        self.get_match_info_pat(patterns).expectations_matched_in_order_exactly()
    }

    // ========================================================================
    // * Private Helpers
    // ========================================================================
    fn get_match_info<T: Into<C>>(&self, expected_calls: Vec<T>) -> MatchInfo {
        // TODO: explain
        let expected_calls_c: Vec<C> = expected_calls
            .into_iter()
            .map(|r| r.into())
            .collect();

        // TODO: explain this
        let mut expectation_to_matching_calls: HashMap<usize, Vec<usize>> =
            HashMap::new();
        for (expected_index, expected_args) in expected_calls_c.iter().enumerate() {
            for (call_index, call_args) in self.calls.borrow().iter().enumerate() {
                if call_args == expected_args {
                    expectation_to_matching_calls
                        .entry(expected_index)
                        .or_insert(vec!())
                        .push(call_index);
                }
            }
        }

        MatchInfo {
            num_expectations: expected_calls_c.len(),
            num_actual_calls: self.calls.borrow().len(),
            expectation_to_matching_calls: expectation_to_matching_calls,
        }
    }

    fn get_match_info_pat(&self, patterns: Vec<&Fn(&C) -> bool>) -> MatchInfo {
        let mut expectation_to_matching_calls: HashMap<usize, Vec<usize>> =
            HashMap::new();
        for (expected_index, pattern_fn) in patterns.iter().enumerate() {
            for (call_index, call_args) in self.calls.borrow().iter().enumerate() {
                if pattern_fn(call_args) {
                    expectation_to_matching_calls
                        .entry(expected_index)
                        .or_insert(vec!())
                        .push(call_index);
                }
            }
        }

        MatchInfo {
            num_expectations: patterns.len(),
            num_actual_calls: self.calls.borrow().len(),
            expectation_to_matching_calls: expectation_to_matching_calls,
        }
    }
}

impl<C, S> Mock<C, Option<S>>
    where C: Clone + Eq + Hash,
          S: Clone
{
    /// Return `Some(return_value)` from `Mock::call`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(), Option<i64>>::new(None);
    /// mock.return_some(10);
    ///
    /// assert_eq!(mock.call(()), Some(10));
    /// ```
    pub fn return_some<T: Into<S>>(&self, return_value: T) {
        self.return_value(Some(return_value.into()))
    }

    /// Return `None` from `Mock::call`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(), Option<i64>>::new(Some(42));
    /// mock.return_none();
    ///
    /// assert_eq!(mock.call(()), None);
    /// ```
    pub fn return_none(&self) {
        self.return_value(None)
    }
}

impl<C, O, E> Mock<C, Result<O, E>>
    where C: Clone + Eq + Hash,
          O: Clone,
          E: Clone
{
    /// Return `Ok(return_value)` from `Mock::call`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(), Result<&str, &str>>::new(Err("oh no"));
    /// mock.return_ok("success");
    ///
    /// assert_eq!(mock.call(()), Ok("success"));
    /// ```
    pub fn return_ok<T: Into<O>>(&self, return_value: T) {
        self.return_value(Ok(return_value.into()))
    }

    /// Return `Err(return_value)` from `Mock::call`.
    ///
    /// # Examples
    ///
    /// ```
    /// use double::Mock;
    ///
    /// let mock = Mock::<(), Result<&str, &str>>::new(Ok("success"));
    /// mock.return_err("oh no");
    ///
    /// assert_eq!(mock.call(()), Err("oh no"));
    /// ```
    pub fn return_err<T: Into<E>>(&self, return_value: T) {
        self.return_value(Err(return_value.into()))
    }
}

impl<C, R> Debug for Mock<C, R>
    where C: Clone + Debug + Eq + Hash,
          R: Clone + Debug
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Mock")
            .field("default_return_value", &self.default_return_value)
            .field("return_value_sequence", &self.return_value_sequence)
            .field("return_values", &self.return_values)
            .field("calls", &self.calls)
            .finish()
    }
}

struct MatchInfo {
    num_expectations: usize,
    num_actual_calls: usize,
    // Maps pattern index to list of call indices that the pattern matches
    expectation_to_matching_calls: HashMap<usize, Vec<usize>>,
}

impl MatchInfo {

    pub fn expectations_matched(&self) -> bool {
        let expected_indices: HashSet<usize> = HashSet::from_iter(
            (0..self.num_expectations));
        let expected_indices_matched: HashSet<usize> = HashSet::from_iter(
            self.expectation_to_matching_calls
            .iter()
            .map(|x| x.0.clone()));
        let unmatched_expectation_indices: HashSet<usize> = HashSet::from_iter(
            expected_indices
            .difference(&expected_indices_matched)
            .map(|i| i.clone()));
        for index in unmatched_expectation_indices.iter() {
            println!(
                "No match found for expected call/pattern with index {}",
                index);
        }
        unmatched_expectation_indices.len() == 0
    }

    pub fn expectations_matched_in_order(&self) -> bool {
        self.expectations_matched() && self.matches_are_in_order()
    }

    pub fn expectations_matched_exactly(&self) -> bool {
        self.expectations_matched() &&
            self.num_expectations_equal_num_actual_calls()
    }

    pub fn expectations_matched_in_order_exactly(&self) -> bool {
        self.expectations_matched_in_order() &&
            self.num_expectations_equal_num_actual_calls()
    }

    fn matches_are_in_order(&self) -> bool {
        /*
        println!("matches_are_in_order");
        if self.expectations_matched() {
            // TODO: explain algo if works
            let sorted_by_pattern_index: Vec<(usize, Vec<usize>)> =
                self.expectation_to_matching_calls
                .iter()
                .map(|(pattern_index, matched_calls)| {
                    (pattern_index.clone(), matched_calls.clone())
                })
                .sorted_by(|a, b| a.0.cmp(&b.0))
                .collect();
            for window in sorted_by_pattern_index.as_slice().windows(2) {
                let ref prev_call_indices = window[0].1;
                let ref next_call_indices = window[1].1;
                println!(
                    "\tprev_call_indices={:?}, next_call_indices={:?}",
                    prev_call_indices, next_call_indices);
                let mut found_next_in_sequence = false;
                for prev_index in prev_call_indices.into_iter() {
                    for next_index in next_call_indices.into_iter() {
                        println!(
                            "\t\tprev_index={}, next_index={}", prev_index, next_index);
                        if next_index > prev_index {
                            println!("\t\tfound_next_in_sequence!");
                            found_next_in_sequence = true;
                            break;
                        }
                    }
                    if !found_next_in_sequence {
                        return false;
                    }
                }
            }

            true
        } else {
            false
        }*/
        false
    }

    fn num_expectations_equal_num_actual_calls(&self) -> bool {
        if self.num_expectations != self.num_actual_calls {
            println!(
                "Mock was called {:?} times, not {:?}",
                self.num_actual_calls,
                self.num_expectations);
            false
        } else {
            true
        }
    }

}
