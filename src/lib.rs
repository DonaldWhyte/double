use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::rc::Rc;

mod matchers;

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
    calls: Ref<Vec<C>>
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
        } else if  let Some(ref function) = self.fns.borrow().get(&args) {
            return function(args)
        } else if let Some(return_value) = self.return_values.borrow().get(&args) {
            return return_value.clone()
        } else if let Some(ref default_fn) = *self.default_fn.borrow() {
            return default_fn(args);
        } else if let Some(ref default_closure) = *self.default_closure.borrow() {
            return default_closure(args);
        } else {
            // TODO: error if a retval sequence was specified but there's no
            // more values left?
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
    where C: Clone + Debug + Eq + Hash + PartialEq,
          R: Clone
{
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
        self.calls.borrow().contains(&args.into())
    }

    /// Returns true if `Mock::call` has been called with all of the specified
    /// `calls`. The `calls` can be made in any order.
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
    pub fn has_calls<T: Into<C>>(&self, expected_calls: Vec<T>) -> bool {
        let num_expected = expected_calls.len();
        let matches = self.match_calls(expected_calls);
        matches.len() == num_expected
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
    /// let mock = Mock::<&str, ()>::new(());
    /// mock.call("foo");
    /// mock.call("bar");
    ///
    /// let expected_calls1 = vec!("foo", "bar");
    /// assert!(mock.has_calls_in_order(expected_calls1));
    /// let expected_calls2 = vec!("bar", "foo");
    /// assert!(!mock.has_calls_in_order(expected_calls2));
    /// let expected_calls3 = vec!("foo");
    /// assert!(mock.has_calls(expected_calls3));
    /// let expected_calls4 = vec!("bar");
    /// assert!(mock.has_calls(expected_calls4));
    /// ```
    pub fn has_calls_in_order<T: Into<C>>(&self, expected_calls: Vec<T>) -> bool {
        let num_expected = expected_calls.len();
        let matches = self.match_calls(expected_calls);
        if matches.len() != num_expected {
            false
        } else {
            let match_indices: Vec<usize> = matches
                .iter()
                .map(|r| r.1.clone())
                .collect();
            for window in match_indices.as_slice().windows(2) {
                if window[0] >= window[1] {
                    return false
                }
            }
            true
        }
    }

    /// Returns true if `Mock::call` has been called with all of the specified
    /// `calls` and it has not been called any other times. The `calls` can be
    /// made in any order.
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
    /// assert!(mock.has_calls_exactly(expected_calls1));
    /// let expected_calls2 = vec!("bar", "foo");
    /// assert!(mock.has_calls_exactly(expected_calls2));
    /// let expected_calls3 = vec!("foo");
    /// assert!(!mock.has_calls_exactly(expected_calls3));
    /// let expected_calls4 = vec!("bar");
    /// assert!(!mock.has_calls_exactly(expected_calls4));
    pub fn has_calls_exactly<T: Into<C>>(&self, expected_calls: Vec<T>) -> bool {
        let num_expected = expected_calls.len();
        let has_calls = self.has_calls(expected_calls);

        let actual_num_calls = self.calls.borrow().len();
        if actual_num_calls > num_expected {
            println!(
                "Mock was called {:?} times, not {:?}",
                actual_num_calls,
                num_expected);
            return false
        }

        has_calls
    }

    /// Returns true if `Mock::call` has been called with all of the specified
    /// `calls` and it has not been called any other times. The `calls` must be
    /// made in the order they are specified in the vector.
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
    pub fn has_calls_exactly_in_order<T: Into<C>>(&self, expected_calls: Vec<T>) -> bool {
        let num_expected = expected_calls.len();
        let has_calls = self.has_calls_in_order(expected_calls);

        let actual_num_calls = self.calls.borrow().len();
        if actual_num_calls > num_expected {
            println!(
                "Mock was called {:?} times, not {:?}",
                actual_num_calls,
                num_expected);
            return false
        }

        has_calls
    }

    fn match_calls<T: Into<C>>(&self, expected_calls: Vec<T>) -> Vec<(C, usize)> {
        let expected_calls_c: Vec<C> = expected_calls
            .into_iter()
            .map(|r| r.into())
            .collect();

        let mut matches: Vec<(C, usize)> = vec!();
        for call in self.calls.borrow().iter() {
            match expected_calls_c.iter().position(|r| call == r)
            {
                Some(index) => {
                    matches.push((call.clone(), index));
                },
                None => {
                }
            }
        }

        {
            let matches_c: Vec<C> = matches.iter().map(|r| r.0.clone()).collect();
            let missing = expected_calls_c.iter().filter(
                |call| !matches_c.contains(call));
            for missing_call in missing {
                println!("Expected call missing: {:?}", missing_call);
            }
        }

        matches
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
