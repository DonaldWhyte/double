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
    default_closure: OptionalRef<Box<dyn Fn(C) -> R>>,
    return_values: Ref<HashMap<C, R>>,
    fns: Ref<HashMap<C, fn(C) -> R>>,
    closures: Ref<HashMap<C, Box<dyn Fn(C) -> R>>>,

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
    pub fn use_closure(&self, default_fn: Box<dyn Fn(C) -> R>) {
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
    pub fn use_closure_for<T: Into<C>>(&self, args: T, function: Box<dyn Fn(C) -> R>) {
        self.closures.borrow_mut().insert(args.into(), function);
    }

    /// Returns true if `Mock::call` has been called.
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
    // type Matcher = dyn Fn(&C) -> bool;
    //
    // TODO: define the above type alias when possible and use that instead of
    // explicitly defining the function signature everywhere.

    /// Returns true if an argument set passed into `Mock::call` matches the
    /// specified `pattern`.
    ///
    /// A `pattern` is defined a function that receives a tuple containing
    /// all of a single call's arguments, checks the values of the arguments
    /// and returns `true` if the args "matched" the pattern and `false`
    /// otherwise. See the
    /// [double repository's README.md](https://github.com/DonaldWhyte/double)
    /// for more information on this.
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
    pub fn called_with_pattern(&self, pattern: &dyn Fn(&C) -> bool) -> bool {
        let patterns: Vec<&dyn Fn(&C) -> bool> = vec!(pattern);
        self.get_match_info_pattern(patterns).expectations_matched()
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
    pub fn has_patterns(&self, patterns: Vec<&dyn Fn(&C) -> bool>) -> bool {
        self.get_match_info_pattern(patterns).expectations_matched()
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
    /// assert!(mock.has_patterns_in_order(vec!(&pattern2, &pattern1, &pattern2)));
    /// assert!(!mock.has_patterns_in_order(vec!(&pattern1, &pattern2, &pattern1)));
    /// assert!(!mock.has_patterns_in_order(vec!(&pattern1, &pattern1, &pattern2)));
    /// assert!(!mock.has_patterns_in_order(vec!(&pattern2, &pattern2, &pattern1)));
    /// assert!(!mock.has_patterns_in_order(vec!(&pattern3)));
    /// assert!(!mock.has_patterns_in_order(vec!(&pattern1, &pattern3)));
    /// ```
    pub fn has_patterns_in_order(&self, patterns: Vec<&dyn Fn(&C) -> bool>) -> bool {
        self.get_match_info_pattern(patterns).expectations_matched_in_order()
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
    pub fn has_patterns_exactly(&self, patterns: Vec<&dyn Fn(&C) -> bool>) -> bool {
        self.get_match_info_pattern(patterns).expectations_matched_exactly()
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
    /// mock.call((42, 0));  // called with same args as first call!
    ///
    /// let pattern1 = |args: &(i32, i32)| args.0 == 42 && args.1 != 0;
    /// let pattern2 = |args: &(i32, i32)| args.0 == 42 && args.1 == 0;
    /// let pattern3 = |args: &(i32, i32)| args.0 == 84;
    ///
    /// assert!(!mock.has_patterns_exactly_in_order(vec!(&pattern1)));
    /// assert!(!mock.has_patterns_exactly_in_order(vec!(&pattern2)));
    /// assert!(!mock.has_patterns_exactly_in_order(vec!(&pattern1, &pattern2)));
    /// assert!(!mock.has_patterns_exactly_in_order(vec!(&pattern2, &pattern1)));
    /// assert!(mock.has_patterns_exactly_in_order(vec!(&pattern2, &pattern1, &pattern2)));
    /// assert!(!mock.has_patterns_exactly_in_order(vec!(&pattern1, &pattern2, &pattern1)));
    /// assert!(!mock.has_patterns_exactly_in_order(vec!(&pattern1, &pattern1, &pattern2)));
    /// assert!(!mock.has_patterns_exactly_in_order(vec!(&pattern2, &pattern2, &pattern1)));
    /// assert!(!mock.has_patterns_exactly_in_order(vec!(&pattern3)));
    /// assert!(!mock.has_patterns_exactly_in_order(vec!(&pattern1, &pattern3)));
    /// ```
    pub fn has_patterns_exactly_in_order(&self, patterns: Vec<&dyn Fn(&C) -> bool>) -> bool {
        self.get_match_info_pattern(patterns).expectations_matched_in_order_exactly()
    }

    // ========================================================================
    // * Private Helpers
    // ========================================================================
    fn get_match_info<T: Into<C>>(&self, expected_calls: Vec<T>) -> MatchInfo {
        let expected_calls_c: Vec<C> = expected_calls
            .into_iter()
            .map(|r| r.into())
            .collect();

        // Build map from expected arg tuple (its index) to the indices of the
        // actual calls made to the mock whose args match that tuple exactly.
        let mut pattern_index_to_match_indices: HashMap<usize, Vec<usize>> =
            HashMap::new();
        for (call_index, call_args) in self.calls.borrow().iter().enumerate() {
            for (expected_index, expected_args) in expected_calls_c.iter().enumerate() {
                if call_args == expected_args {
                    pattern_index_to_match_indices
                        .entry(expected_index)
                        .or_insert(vec!())
                        .push(call_index);
                }
            }
        }

        MatchInfo {
            num_expectations: expected_calls_c.len(),
            num_actual_calls: self.calls.borrow().len(),
            pattern_index_to_match_indices: pattern_index_to_match_indices,
        }
    }

    fn get_match_info_pattern(&self, patterns: Vec<&dyn Fn(&C) -> bool>) -> MatchInfo {
        // Build map from pattern (its index) to the indices of the actual
        // calls made to the mock whose args match that pattern.
        let mut pattern_index_to_match_indices: HashMap<usize, Vec<usize>> =
            HashMap::new();
        for (call_index, call_args) in self.calls.borrow().iter().enumerate() {
            for (expected_index, pattern_fn) in patterns.iter().enumerate() {
                if pattern_fn(call_args) {
                    pattern_index_to_match_indices
                        .entry(expected_index)
                        .or_insert(vec!())
                        .push(call_index);
                }
            }
        }

        MatchInfo {
            num_expectations: patterns.len(),
            num_actual_calls: self.calls.borrow().len(),
            pattern_index_to_match_indices: pattern_index_to_match_indices,
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
    // Maps actual call index to the indices of patterns that match the call
    pattern_index_to_match_indices: HashMap<usize, Vec<usize>>,
}

impl MatchInfo {
    pub fn expectations_matched(&self) -> bool {
        let expected_indices: HashSet<usize> = HashSet::from_iter(
            0..self.num_expectations);
        let expected_indices_matched = HashSet::from_iter(
            self.pattern_index_to_match_indices
            .keys()
            .map(|k| k.clone()));
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
        // If all the expectations are met, use the indices of all matching
        // calls (for each pattern) to determine if the calls were made in
        // the order specified by the expectated patterns.
        //
        // This is more difficult than one might think. Each expected pattern
        // can match multiple calls. Additionally, the total set of
        // expectations can be smaller than the total number of calls. Both of
        // two aspects make this problem tricky.
        //
        // The following algorithm is used for the check:
        //
        // 1. For each pattern, construct a list containing the indices of the
        //    calls that match it
        // 2. Generate all permutations of the sequence of actual calls that
        //    matched each of the N patterns (uses the lists from (1))
        // 3. For each permutation, check if the call indices in the
        //    permutation are strictly increasing. If so, we've found a
        //    permutation that occurred where the call order and the expected
        //    pattern order match. This means the expectations were indeed
        //    matched in order and return true.
        // 4. If none of the permutations are strictly increasing, the
        //    expected patterns were matched, but not in the expected order.
        //    Return false.
        //
        //
        // The complexity is O(N!), where N is the number of patterns in the
        // expected sequence. The factorial complexity is caused by the
        // generation of all permutations of matching call index sequences in.
        // step (2). The O(N!) complexity is currently not a concern for two
        // reasons:
        //
        // * Most ordered checks run by clients involve less than 5 patterns,
        //   so the upper bound typically won't exceed 5!.
        // * The constant factor is almost always very low (most of the time
        //   a pattern will only ever match one call arg, meaning the number
        //   of permutations is very small, even if N is high).
        //
        // This algorithm will only be revised if a legitmate performance issue
        // is found.
        if self.expectations_matched() {
            let permutation_constraints = self.pattern_index_to_match_indices
                .iter()
                .sorted_by(|a, b| a.0.cmp(&b.0))
                .map(
                    |(_, matching_call_indices)| matching_call_indices.clone())
                .collect();
            for permutation in generate_permutations(&permutation_constraints) {
                if is_strictly_increasing(permutation.as_slice()) {
                    return true;
                }
            }
            false
        } else {
            false
        }
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

fn generate_permutations(constraints: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut output: Vec<Vec<usize>> = vec!();
    if !constraints.is_empty() {
        let mut permutation_buffer: Vec<usize> = vec!();
        permutation_buffer.resize(constraints.len(), 0);

        generate_permutations_impl(
            &mut output, &mut permutation_buffer, constraints, 0);
    }
    output
}

fn generate_permutations_impl(
    output_permutations: &mut Vec<Vec<usize>>,
    permutation_buffer: &mut Vec<usize>,
    constraints: &Vec<Vec<usize>>,
    current_index: usize)
{
    if current_index < permutation_buffer.len() {
        for val in &constraints[current_index] {
            permutation_buffer[current_index] = val.clone();
            generate_permutations_impl(
                output_permutations,
                permutation_buffer,
                constraints,
                current_index + 1)
        }
    } else {
        output_permutations.push(permutation_buffer.clone());
    }
}

fn is_strictly_increasing(sequence: &[usize]) -> bool {
    for window in sequence.windows(2) {
        if window[0] >= window[1] {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_permutations_no_constraints() {
        let constraints: Vec<Vec<usize>> = vec!();
        let permutations = generate_permutations(&constraints);
        let no_permutations_expected: Vec<Vec<usize>> = vec!();
        assert_eq!(no_permutations_expected, permutations);
    }

    #[test]
    fn generate_permutations_one_constraint_one_value() {
        let constraints = vec!(vec!(42));
        let permutations = generate_permutations(&constraints);
        assert_eq!(vec!(vec!(42)), permutations);
    }

    #[test]
    fn generate_permutations_one_constraint_multiple_values() {
        let constraints = vec!(vec!(42, 84, 0));
        let permutations = generate_permutations(&constraints);
        assert_eq!(vec!(vec!(42), vec!(84), vec!(0)), permutations);
    }

    #[test]
    fn generate_permutations_various_constraints() {
        let constraints = vec!(
            vec!(0),
            vec!(0, 1),
            vec!(0),
            vec!(2, 3, 4)
        );
        let permutations = generate_permutations(&constraints);
        assert_eq!(permutations, vec!(
            vec!(0, 0, 0, 2),
            vec!(0, 0, 0, 3),
            vec!(0, 0, 0, 4),
            vec!(0, 1, 0, 2),
            vec!(0, 1, 0, 3),
            vec!(0, 1, 0, 4)));
    }

    #[test]
    fn is_strictly_increasing_empty_sequence() {
        let sequence: Vec<usize> = vec!();
        assert!(is_strictly_increasing(sequence.as_slice()));
    }

    #[test]
    fn is_strictly_increasing_sequence_with_one_element() {
        let sequence: Vec<usize> = vec!(42);
        assert!(is_strictly_increasing(sequence.as_slice()));
    }

    #[test]
    fn is_strictly_increasing_sequence_with_multiple_elements() {
        let sequence: Vec<usize> = vec!(42, 43, 44, 46, 80, 15000);
        assert!(is_strictly_increasing(sequence.as_slice()));
    }

    #[test]
    fn is_strictly_increasing_sequence_value_stays_the_same() {
        let sequence: Vec<usize> = vec!(42, 43, 44, 44, 80, 15000);
        assert!(!is_strictly_increasing(sequence.as_slice()));
    }

    #[test]
    fn is_strictly_increasing_sequence_value_goes_down() {
        let sequence: Vec<usize> = vec!(42, 43, 44, 1, 80, 15000);
        assert!(!is_strictly_increasing(sequence.as_slice()));
    }
}
