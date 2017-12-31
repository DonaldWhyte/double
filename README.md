# Double

### Full-featured mocking library in Rust, including rich failure messages and argument matchers.

[![Build Status](https://travis-ci.org/DonaldWhyte/double.svg?branch=master)](https://travis-ci.org/DonaldWhyte/double) [![Docs](https://docs.rs/double/badge.svg)](https://docs.rs/double)

Double lets you mock `trait` implementations so that you can track function call arguments and set return values or overrides functions at test time. foo

Here's a quick example:

```rust
#[macro_use]
extern crate double;

// Code under test
trait BalanceSheet {
    fn profit(&self, revenue: u32, costs: u32) -> i32;
}

fn double_profit(revenue: u32, costs: u32, balance_sheet: &BalanceSheet) -> i32 {
    balance_sheet.profit(revenue, costs) * 2
}

// Test which uses a mock BalanceSheet
mock_trait!(
    MockBalanceSheet,
    profit(u32, u32) -> i32);
impl BalanceSheet for MockBalanceSheet {
    mock_method!(profit(&self, revenue: u32, costs: u32) -> i32);
}

fn test_doubling_a_sheets_profit() {
    // GIVEN:
    let sheet = MockBalanceSheet::default();
    sheet.profit.return_value(250);
    // WHEN:
    let profit = double_profit(500, 250, &sheet);
    // THEN:
    // mock returned 250, which was doubled
    assert_eq!(500, profit);
    // assert that the revenue and costs were correctly passed to the mock
    sheet.profit.has_calls_exactly_in_order(vec!((500, 250)));
}

// Executing test
fn main() {
    test_doubling_a_sheets_profit();
}
```

More examples are available in the [examples directory](./examples).

### Defining a Mock

Mocking a `trait` requires two steps. One to generate the mock `struct` that will implement the mock and another to generate the bodies of the mocked `trait` methods.

For step one, we use the `mock_trait` macro. This takes the name of the mock `struct` to generate and a list specifying all of the `trait`'s methods, their arguments (omitting `self`) and their return values (specifying `-> ()` if the method does not return a value).

Consider the example below:

```rust
trait BalanceSheet {
    fn profit(&self, revenue: u32, costs: u32) -> i32;
    fn clear(&mut self);
}

mock_trait!(
    MockBalanceSheet,
    profit(u32, u32) -> i32,
    clear() -> ());
```

Here, we generate a `struct` called `MockBalanceSheet`. This struct contains all the necessary data to store the number of types each method is called, what arguments they are invoked with and what values each method should return when invoked. This data is stored per-method, with the `struct` having a `double::Mock` field for each method. This is why all of the `trait`'s methods must be declared when the `struct` is generated.

For step 2, we generate the bodies of the mocked methods. The generated bodies contain boilerplate code for passing the method's arguments to the underlying `double::Mock` objects using `mock_method`. For example:

```rust
impl BalanceSheet for MockBalanceSheet {
    mock_method!(profit(&self, revenue: u32, costs: u32) -> i32);
    mock_method!(clear(&mut self));
}
```

> Notice how both immutable and mutable methods can be specified. One just passes `&self` or `&mut self` to `mock_method`, depending on whether the `trait` being mocked specifies the method as immutable or mutable.

After both of these steps, the mock object is ready to use.

### Using a Mock

Tests with mocks are typically structured like so:

1. **GIVEN**: create mock objects and specify what values they return
2. **WHEN**: run code under test, passing mock objects to it
3. **THEN**: assert mocks were called the expected number of times, with the expected arguments

For example, suppose we wish to test some code that uses a `BalanceSheet` generate a HTML page showing the current profit of something:

```rust
fn generate_profit_page<T: BalanceSheet>(revenue: u32, costs: u32, sheet: &T) {
    let profit_str = sheet.profit(revenue, costs).to_string();
    return "<html><body><p>Profit is: $" + profit_str + "</p></body></html>";
}
```

We can use our generated `MockBalanceSheet` to test this function:

```rust
fn test_balance {
    // GIVEN:
    // create instance of mock and configure its behaviour (will return 42)
    let mock = MockBalanceSheet::default();
    mock.profit.return_value(42);

    // WHEN:
    // run code under test
    let page = generate_profit_page(30, 20);

    // THEN:
    // ensure mock affected output in the right away
    assert_eq!("<html><body><p>Profit is: $42</p></body></html>")
    // also assert that the mock's profit() method was called _exactly_ once,
    // with the arguments 30 (for revenue) and 20 (for costs).
    assert_true!(mock.profit.has_calls_exactly(
        vec!((30, 20))
    ));
}
```

#### GIVEN: Setting Mock Behaviour

Mocks can be configured to return a single value, a sequence of values (one value for each call) or invoke a function/closure. Additionally, it is possible to make a mock return special values/invoke special functions when specific arguments are passed in.

These behaviours are configured by invoking methods on the mock objects themselves. These methods are listed in the table below.

| Method | What It Does |
| ------ | ------------ |
| `use_fn_for((args), Fn(...) -> retval)` | invoke given function and return the value it returns when specified `(args)` are passed in |
| `use_closure_for((args), &Fn(...) -> retval)` | invoke given closure and return the value it returns when specified `(args)` are passed in |
| `return_value_for((args), val)` | return `val` when specified `(args)` are passed in |
| `use_fn(Fn(...) -> retval)` | invoke given function and return the value it returns by default |
| `use_closure(&Fn(...) -> retval)` | invoke given closure and return the value it returns by default |
| `return_values(vec<retval>)` | return values in given vector by default, return one value for each invocation of the mock method. If there are no more values in the vector, return the default value specified by `return_value()`  |
| `return_value(val)` | return `val` by default |

If no behaviour is specified, the mock will just return the default value of the return type, as specified by the `Default` trait.

Example usage:

```rust
// Configure mock to return 9001 profit when given args 42 and 10. Any other
// arguments will cause the mock to return a profit of 1.
let sheet = MockBalanceSheet::default();
sheet.profit.return_value_for((42, 10), 9001);
sheet.profit.return_value(1);

// Configure mock to call arbitrary function. The mock will return the
// result of the function back to the caller.
fn subtract(revenue: u32, costs: u32) -> i32 {
    revenue - costs
}
let sheet2 = MockBalanceSheet::default();
sheet.use_fn(subtract);
```

Code examples on how to use these are available in the [**rustdocs**](https://docs.rs/double).

It is possible to use many of these in conjunction. For example, one can tell a mock to return a specific value for args `(42, 10)` using `return_value_for()`, but return the default value of 1 for everything else using `return_value()`.

When a mock method is invoked, it uses a precdence order to determine if it should return a default value, return a specific value, invoke a function and so on.

The precedence order of these methods is the same order they are specified in the above table. For example, if `use_fn` and `return_value` are invoked, then the mock will invoke the function passed to `use_fn` and not return a value.

If a method returns an `Option<T>` or a `Result<T, E>`, then one can use the following convenience functions for specifying default return values:

| Method        | Returns     | What It Does                         |
| ------------- | ----------- | ------------------------------------ |
| `return_some` | `Some(val)` | return `Some(val)` enum of `Option`  |
| `return_none` | `None`      | returs the `None` enum of `Option`   |
| `return_ok`   | `Ok(val)`   | return `Ok(val)` enum of `Result`    |
| `return_err`  | `Err(val)`   | return `Err(val)` enum of `Result`   |

#### THEN: Asserting Code Under Test Used Mock in Expected Way

After the test has run, we can verify the mock was called the right number of times and with the right arguments.

The table below lists the methods that can be used to verify the mock was invoked as expected.

| Method                                          | Returns       | What It Does |
| ----------------------------------------------- | ------------- | ------------ |
| `calls()`                                       | `Vec<(Args)>` | return the arguments of each mock invocation, ordered by invocation time |
| `called()`                                      | `bool`        | return `true` if method was called at least once |
| `num_calls()`                                   | `usize`       | number of times method was called |
| `called_with((args))`                           | `bool`        | return `true` if method was called at least once with given `args` |
| `has_calls(vec!((args), ...))`                  | `bool`        | return `true` if method was called at least once for each of the given `args` tuples |
| `has_calls_in_order(vec!((args), ...))`         | `bool`        | return `true` if method was called at least once for each of the given `args` collections, and called with arguments in the same order as specified in the input `vec` |
| `has_calls_exactly(vec!((args), ...))`          | `bool`        | return `true` if method was called exactly once for each of the given `args` collections|
| `has_calls_exactly_in_order(vec!((args), ...))` | `bool`        | return `true` if method was called exactly once for each of the given `args` collections, and called with arguments in the same order as specified in the input `vec` |

Example usage:

```rust
let sheet = MockBalanceSheet::default();

// invoke mock method
sheet.profit(42, 10);
sheet.profit(5, 0);

// assert the invocation was recorded correctly
assert!(sheet.profit.called());
assert!(sheet.profit.called_with((42, 10)));
assert!(sheet.profit.has_calls((42, 10)));
assert!(sheet.profit.has_calls_in_order((42, 10), (5, 0)));
assert!(sheet.profit.has_calls_exactly((5, 0), (42, 10)));
assert!(sheet.profit.has_calls_exactly_in_order((42, 10), (5, 0)));
```

#### Reusing Mocks Across Multiple Tests

Invoke `reset_calls()` to clear all recorded calls of a mock method.

To ensure individual tests are as isolated (thus, less likely to have bugs) as possible, it is recommended that different mock objects are constructed for different test cases.

Nevertheless, there might a some case where reusing the same mock and its return values results in easier to read and more maintainable test code. In those cases, `reset_calls()` can be used to clear calls from previous tests.

### Other Use Cases

#### Mocking Methods without a Return Value

If a method does not return anything, the return value can be omitted when generating the method using double's macros:

```rust
trait Queue {
    fn enqueue(&mut self, value: i32);
    fn dequeue(&mut self) -> i32;
}

mock_trait!(
    MockQueue,
    enqueue(i32) -> (), // still have to specify return value here...
    dequeue() -> i32);
impl Queue for MockQueue {
    mock_method!(enqueue(&mut self, value: i32));  // ...but not here!
    mock_method!(dequeue(&mut self) -> i32);
}
```

#### Mocking Methods That Return Types Which Do Not Implement `Default`

The `mock_trait!` macro assumes the return types of all the methods in the mocked `trait` implement `Default`. This makes it convenient to construct the mock object. One can invoke `MockTrait::default()` to construct the mock object and auto-configure it return default values for all methods in one go.

If a `trait` provides a method that returns a type that _does not_ implement `Default`, then one must generate the mock using `mock_trait_no_default!`. This macro generates a mock that does not implement `Default`. Clients must construct instances of the generated mock using `MockTrait::new()`, manually specifying the default return values for each method.

For example:

```
// `Result` does not implement the `Default` trait. Trying to mock `UserStore`
// using the `mock_trait!` macro will fail. We use `mock_trait_no_default!`
// instead.
pub trait UserStore {
    fn get_username(&self, id: i32) -> Result<String, String>;
}

mock_trait_no_default!(
    MockUserStore,
    get_username(i32) -> Result<String, String>);

impl UserStore for MockUserStore {
    mock_method!(get_username(&self, id: i32) -> Result<String, String>);
}

fn test_manually_setting_default_retval() {
    // GIVEN:
    // Construct instance of the mock, manually specifying the default
    // return value for `get_username()`.
    let mock = MockUserStore::new(
        Ok("default_user_name".to_owned()));  // get_username() default retval
    // WHEN:
    let result = mock.get_username(10001);
    // THEN:
    assert_eq!(Ok("default_username".to_owned()), result);
}
```

#### Mocking Methods That Take `&str` References

`&str` is a common argument type. However, double does not support mocking methods with `&str` arguments with additional boilerplate.

This is because a mock cannot _store_ received `&str` arguments. The mock needs to the _own_ the given arguments and `&str` is a non-owning reference. Therefore, the mock trait has to be specified like so:

```rust
trait TextStreamWriter {
    fn write(&mut self, text: &str);
}

mock_trait!(
    MockTextStreamWriter,
    // have to use `String`, not `&str` here, since `&str` is a reference
    write(String) -> ()
);

impl TextStreamWriter for MockTextStreamWriter {
    mock_method!(write(&mut self, text: &str), self, {
        // manually convert the reference to an owned `String` before passing
        // it to the underlying mock object
        self.write.call(text.to_owned())
    });
}
```

The `mock_method` variant used above allows you to specify  the body of the generated function manually. The custom body simply converts the `&str` argument to an owned string and passes it into the underlying `write` `Mock` object manually. (normally auto-generated bodies do this for you).

> NOTE: The name of the underlying mock object is always the same as the mocked
method's name. So in the custom `write` body, you should pass the arguments down to `self.write`.

`&str` parameters are common. We understand that it is inconvenient to manually specify the body each time they appear. There are plans to add a macro to generate a body that calls `to_owned()` automatically. This section will be updated when that has been released.

#### Mocking Methods with Generic Type Parameter

Mocking methods with generic type parameters require extra effort. For example, suppose one had a `Comparator` trait that was responsible for comparing any two values in the program. It might look something like this:

```rust
trait Comparator {
   fn is_equal<T: Eq>(&self, a: &T, b: &T) -> bool;
}
```

`T` can be multiple types. Currently, we cannot store call arguments that
have generic types in the underlying `Mock` objects. Therefore, one has to
convert the generic types to a different, common representation. One way
to get around this limitation is converting each generic type to a `String`.
e.g. for the `Comparator` trait:

```rust
# #[macro_use] extern crate double;

use std::string::ToString;

trait Comparator {
   fn is_equal<T: Eq + ToString>(&self, a: &T, b: &T) -> bool;
}

mock_trait!(
    MockComparator,
    // store all passed in call args as strings
    is_equal((String, String)) -> bool
);

impl Comparator for MockComparator {
    mock_method!(is_equal<(T: Eq + ToString)>(&self, a: &T, b: &T) -> bool, self, {
        // Convert both arguments to strings and manually pass to underlying
        // mock object.
        // Notice how the both arguments as passed as a single tuple. The
        // underlying mock object always expects a single tuple.
        self.is_equal.call((a.to_string(), b.to_string()))
    });
}
```

If the `to_string` conversions for all `T` are not lossy, then our mock expectations can be exact. If the `to_string` conversions _are_ lossy, then this mechanism can still be used, providing all the properties of the passed in objects are captured in the resultant `String`s.

This approach requires the writer to ensure the code under test adds the `ToString` trait to the `trait`'s type argument constraints. This limitation forces test writers to modify production code to use `double` for mocking.

Despite this, there is still value in using `double` for mocking generic methods with type arguments. Despite adding boilerplate to production code and manually implementing mock method bodies being cumbersome, the value add is that all argument matching, expectations, calling test functions, etc. are all still handled by `double`.

The authors of double argue that reimplenting the aforementined features is more cumbersome than the small amount of boilerplate required to mock methods with type arguments.

#### Using double Mocks for Free Functions

`double::Mock` objects can also be used for free functions. Consider the following function:

```rust
fn calculate_factor(value: i32, weighting_fn: &Fn(i32) -> i32) -> i32 {
    weighting_fn(value * 2)
}
```

This doubles some input value and applies a weighting to it. Suppose the weighting function can vary. For example, let's say the weighting function to use depends on user provided config. This means we need to pass a generic weighting function as a parameter.

Rather than generate your own mock weighting function boilerplate when testing `calculate_factor`, one can directly use `double::Mock`:

```rust
fn calculate_factor(value: i32, weighting_fn: &Fn(i32) -> i32) -> i32 {
    weighting_fn(value * 2)
}

fn main() {
    let mock_weighting_fn = Mock::<i32, i32>::default();
    mock_weighting_fn.return_value(100);

    // Wrap mock in a closure that is passed to the function under test. Note
    // how the closure is passed as a _reference_ for this
    // (e.g. &|x: i32| ...)
    let result = calculate_factor(42, &|x: i32| mock_weighting_fn.call(x));

    assert_eq!(100, result);
    assert!(mock_weighting_fn.has_calls_exactly(
        vec!(84)  // input arg should be doubled by calculate_factor()
    ));
}
```
