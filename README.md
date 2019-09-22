# Double

### Full-featured mocking library in Rust with feature-rich argument matchers.

[![Build Status](https://travis-ci.org/DonaldWhyte/double.svg?branch=master)](https://travis-ci.org/DonaldWhyte/double) [![Docs](https://docs.rs/double/badge.svg)](https://docs.rs/double)

Double lets you mock `trait` implementations so that you can track function call arguments and set return values or overrides functions at test time.

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
| `use_fn_for((args), dyn Fn(...) -> retval)` | invoke given function and return the value it returns when specified `(args)` are passed in |
| `use_closure_for((args), &dyn Fn(...) -> retval)` | invoke given closure and return the value it returns when specified `(args)` are passed in |
| `return_value_for((args), val)` | return `val` when specified `(args)` are passed in |
| `use_fn(dyn Fn(...) -> retval)` | invoke given function and return the value it returns by default |
| `use_closure(&dyn Fn(...) -> retval)` | invoke given closure and return the value it returns by default |
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

| Method                                                 | Returns       | What It Does |
| ------------------------------------------------------ | ------------- | ------------ |
| `calls()`                                              | `Vec<(Args)>` | return the arguments of each mock invocation, ordered by invocation time. |
| `called()`                                             | `bool`        | return `true` if method was called at least once. |
| `num_calls()`                                          | `usize`       | number of times method was called. |
| `called_with((args))`                                  | `bool`        | return `true` if method was called at least once with given `args`. |
| `has_calls(vec!((args), ...))`                         | `bool`        | return `true` if method was called at least once for each of the given `args` tuples. |
| `has_calls_in_order(vec!((args), ...))`                | `bool`        | return `true` if method was called at least once for each of the given `args` collections, and called with arguments in the same order as specified in the input `vec`. |
| `has_calls_exactly(vec!((args), ...))`                 | `bool`        | return `true` if method was called exactly once for each of the given `args` collections. |
| `has_calls_exactly_in_order(vec!((args), ...))`        | `bool`        | return `true` if method was called exactly once for each of the given `args` collections, and called with arguments in the same order as specified in the input `vec`. |
| `called_with_pattern(matcher_set)`                      | `bool`        | return `true` if method was called at least once with args that match the given matcher set. |
| `has_patterns(vec!(matcher_set, ...))`                  | `bool`        | return `true` if all of the given matcher sets were matched at least once by the mock's calls. |
| `has_patterns_in_order(vec!(matcher_set, ...))`         | `bool`        | return `true` if mock has calls that match all the specified matcher sets. The matcher sets must be matched in the order they are specified by the input `matcher_set` vector. |
| `has_patterns_exactly(vec!(matcher_set, ...))`          | `bool`        | return `true` if all of the given matcher sets were matched at least once by the mock's calls. The number of calls equal the number of specified matcher sets. |
| `has_patterns_exactly_in_order(vec!(matcher_set, ...))` | `bool`        | return `true` if mock has calls that match all the specified matcher sets. The matcher sets must be matched in the order they are specified by the input `matcher_set` vector. T the number of calls equal the number of specified matcher sets. |

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

> See section **Pattern Matching** for detail on how to use the pattern-based assertions.

#### Reusing Mocks Across Multiple Tests

Invoke `reset_calls()` to clear all recorded calls of a mock method.

To ensure individual tests are as isolated (thus, less likely to have bugs) as possible, it is recommended that different mock objects are constructed for different test cases.

Nevertheless, there might a some case where reusing the same mock and its return values results in easier to read and more maintainable test code. In those cases, `reset_calls()` can be used to clear calls from previous tests.

### Pattern Matching

When a mock function has been used in a test, we typically want to make assertions about what the mock has been called with. For example, suppose we're testing some logic that determines the next action of a robot. We might want to assert what this logic told the robot to do:

```rust
let robot = MockRobot::default();
do_something_with_the_robot(&robot);
assert!(robot.move_forward.called_with(100);
```

The above code checks that `do_something_with_the_robot()` should tell the robot to move 100 units forward. However, sometimes you might not want to be this specific. This can make tests being too rigid. Over specification leads to brittle tests and obscures the intent of tests. Therefore, it is encouraged to specify only what's necessary &mdash; no more, no less.

If you care that `moved_forward()` will be called but aren't interested in its actual argument, you can simply assert on the call count:

```rust
assert!(robot.move_forward.called())
assert!(robot.move_forward.num_calls() == 1u)
```

But what if the behaviour we wanted to check is a little more nuanced? What if we wanted to check that the robot was moved forward at least 100 units, but it didn't matter if the robot moved even further than that? If this case, our assertion is more specific than "was `move_forward()` called?", but the constraint is not as tight as "has to be moved _exactly_ 100 units".

If we know the current implementation will move exactly 100 units, it is tempting to just use check for exact equality. However, as mentioned, this makes the tests very brittle. If the implementation is technically free to change, and start moving more than 100 units, then this test breaks. The developer has to go to the tests and fix the broken test. That change would not be required if the test wasn't overly restrictive. This may sound minor. However, code bases grow. This means the number of tests also grows. If all of the tests are brittle, then it becomes a huge burden to maintain/update these tests whenever production code changes.

`double` allows developers to avoid this by using fuzzy assertions. One can perform looser assertions on mock argument values using **pattern matching**. In the robot example, we can assert that the robot move forward 100 _or more_ units with one line of code:

```rust
use double::matcher::*;

assert!(robot.move_forward.called_with_pattern(p!(ge, 100)));
```

Let's break this down. First, we changed `called_with` to `called_with_pattern`. Then, we pass in the matcher we want to use like so:

```rust
p!(ge, 100)
```

The `p!` macro generates a matcher function that the mock object accepts. `ge` is the name of the matcher (ge = greater than or equal to). `100` is a *matcher argument* that configures the matcher to match on the right values. Passing `100`  to `ge` means "construct a matching function that matches values that are >= `100`".

Pattern matching is also possible with functions that take multiple arguments. We simply wrap individual argument matchers using the `matcher!` macro:

```rust
assert!(robot.move.called_with_pattern(
    matcher!( p!(ge, 100), p!(eq, Direction::Left) )
));
```

The code above is asserting that the robot's `move()` method was called, and that the robot moved at least 100 units in the `Left` direction.

There are other check functions like `called_with_pattern()` that make use of pattern matchers. See section **THEN: Asserting Code Under Test Used Mock in Expected Way** for a list of these functions.

#### Formal Definition

Formally, a pattern matcher is defined a function that receives a single argument's value. It performs some check on the value, returning `true` if the argument's value "matches" the desired pattern and `false` otherwise. For example, the `any` matcher (which makes any value) is defined as:

```rust
pub fn any<T>(_: &T) -> bool {
    true
}
```

where `_` is the argument value. Most matchers are _parametrised_. For example, the `eq` matcher takes one parameter -- the expected value. Likewise, the matcher `ge` takes one parameter, which specifies the number that the expected value should be greater than or equal to.

Matchers can take any number of parameters. Additional parameters are specified as additional arguments to the matcher function. For example, here's the definition of the `eq` matcher function.

```rust
pub fn eq<T: PartialEq>(arg: &T, target_val: T) -> bool {
    *arg == target_val
}
```

For a parametrised matcher to be used, it must be bound to a parameter set. The `p!` macro does this. It takes a matcher function and a set of parameters to bind to it, then returns a new closure function which is bound to those parameters. For example:

```rust
let bound_matcher = p!(eq, 42);
assert!(bound_matcher(42) == true);
assert!(bound_matcher(10) == false);
```

Notice how the bound matcher takes a single argument &mdash; the argument value being matched. The matcher function's other arguments are bound within the returned closure.

When passing matchers to a `Mock`'s assertion calls (e.g. `called_with_pattern` and `has_patterns`), they need to be passed as a _matcher set_. `Mock`'s assertion checks operation on the full set of arguments the mocked function has, not just individual arguments. For example, if a mocked function takes three arguments, then `called_with_pattern` expects a matcher set of size 3. The set contains one matcher for each of the mock's arguments.

Matcher sets are constructed using the `matcher!` macro. This macro takes a bound matcher function for each argument in the mocked function. The order of the matcher functions corresponds to the order of the arguments in the mocked function.

Here's an example of `matcher!` in use:

```rust
let arg1_matcher = p!(eq, 42);
let arg2_matcher = p!(lt, 10);
let arg_set_matcher = matcher!(arg1_matcher, arg2_matcher);

assert!(matcher((42, 5)) == true);
assert!(matcher((42, -5)) == true);
assert!(matcher((42, 10)) == false);
assert!(matcher((100, 5)) == false);
```

In reality, most `matcher!` and `p!` invocations will be made within the assertion call. Combining `matcher!` and `p!` invocations inline allows developers to write concise and expressive assertions like this:

```rust
// This reads:
//     * first arg should be >= 100
//     * second arg should be `Direction::Left`

assert!(robot.move.called_with_pattern(
    matcher!( p!(ge, 100), p!(eq, Direction::Left) )
));
```

#### Nesting Matchers

It is possible to nest matchers. For example, you might want to assert that an argument matches _multiple_ patterns.

Going back to the robot example, perhaps we don't care about the _exact_ amount the robot moved forward, but we care it's between some range. Let's say want to assert that the moved between the 100-200 unit range. No less, no more.

We use use two matchers, `ge` and `le`, for the one argument. We wrap them in the composite matcher `all_of`, like so:

```rust
assert!(robot.move_forward.called_with_pattern(
    matcher!(
        p!(all_of, vec!(
            p!(ge, 100),
            p!(le, 200))))
));
```

Users can nested matchers using the `p!` macro an arbitrary number of times. Try not to go too far with this feature though, as it can lead to tests that are difficult to read.

> NOTE: The above was for illustration. The simpler way to perform a value range check is using the non-composite `between_exc` and `between_inc` macros.

#### Built-in Matchers

This section lists all the standard matchers built-in into the library. See the **Defining your Own Matchers** section if none of these fit your use case.

##### Wildcards

|         |                                               |
| ------- | --------------------------------------------- |
| `any()` | argument can be any value of the correct type |

##### Comparison Matchers

|                    |                                                                 |
| ------------------ | --------------------------------------------------------------- |
| `eq(value)`        | `argument == value`                                             |
| `ne(value)`        | `argument != value`                                             |
| `lt(value)`        | `argument < value`                                              |
| `le(value)`        | `argument <= value`                                             |
| `gt(value)`        | `argument > value`                                              |
| `ge(value)`        | `argument >= value`                                             |
| `is_some(matcher)` | argument is an `Option::Some`, whose contents matches `matcher` |
| `is_ok(matcher)`   | argument is an `Result::Ok`, whose contents matches `matcher`   |
| `is_err(matcher)`  | argument is an `Result::er`, whose contents matches `matcher`   |

##### Floating-Point Matchers

|                               |                                                                                             |
| ----------------------------- | ------------------------------------------------------------------------------------------- |
| `f32_eq(value)`               | argument is a value approximately equal to the `f32` `value`, treating two NaNs as unequal. |
| `f64_eq(value)`               | argument is a value approximately equal to the `f64` `value`, treating two NaNs as unequal. |
| `nan_sensitive_f32_eq(value)` | argument is a value approximately equal to the `f32` `value`, treating two NaNs as equal.   |
| `nan_sensitive_f64_eq(value)` | argument is a value approximately equal to the `f64` `value`, treating two NaNs as equal.   |

##### String Matchers

|                       |                                                   |
| --------------------- | ------------------------------------------------- |
| `contains(string)`    | argument contains `string` as a sub-string.       |
| `starts_with(prefix)` | argument starts with string `prefix`.             |
| `starts_with(suffix)` | argument ends with string `suffix`.               |
| `eq_nocase(string)`   | argument is equal to `string`, ignoring case.     |
| `ne_nocase(value)`    | argument is not equal to `string`, ignoring case. |

##### Container Matchers

There are currently no matchers to inspect the contents of containers. These will be added in future version of `double`. There is a [GitHub issue](https://github.com/DonaldWhyte/double/issues/12) to track this work.

##### Composite Matchers

|                                |                                                    |
| ------------------------------ | -------------------------------------------------- |
| `all_of(vec!(m1, m2, ... mn))` | argument matches all of the matchers `m1` to `mn`. |
| `any_of(vec!(m1, m2, ... mn))` | matches at least one of the matchers `m1` to `mn`. |
| `not(m)`                       | argument doesn't match matcher `m`.                |

#### Defining your Own Matchers

If none of the built-in matchers fit your use case, you can define your own.

Suppose we were testing a restful service. We have some request handling logic. We want to test the handling logic responded to the request correctly. In this context, "correctly" means it responded with a JSON object that contains the "time" key.

Here's the production code to test:

```rust
trait ResponseSender {
    fn send_response(&mut self, response: &str);
}

fn request_handler(response_sender: &mut ResponseSender) {
    // business logic here
    response_sender.send_response(
        "{ \"current_time\": \"2017-06-10 20:30:00\" }");
}
```

Let's mock response sender and assert on the contents of the JSON response:

```rust
mock_trait!(
    MockResponseSender,
    send_response(&str) -> ());
impl ResponseSender for MockResponseSender {
    mock_method!(send_response(&mut self, response: &str));
}

#[test]
fn ensure_current_time_field_is_returned() {
    // GIVEN:
    let mut mock_sender = MockResponseSender::default();

    // WHEN:
    request_handler(&mock_sender);

    // THEN:
    // check the sender received a response that contains a current_time field
}
```

This check is cumbersome. One has to manually extract the text string passed to the mock, parse it as JSON (handling invalid JSON) and manually checking

For one test, perhaps this is not an issue. However, imagine we had multiple test cases for dozens of API endpoints. Duplicating the same JSON assertion logic across all the tests leads to code repetition. This repetition buries the intentions of the tests and makes them harder to change.

Custom matchers to the rescue! We can use matchers to check if the response text string is a valid JSON object that has a certain key/field.

A matcher is defined as a function that takes at least one argument (the `arg` being matched) and zero or more parameters. It returns a `bool` that indicates if `arg` is a match. We have one parameter in this case &mdash; the `key` we're asserting exists in the response.

```rust
extern crate json;
use self::json;

fn is_json_object_with_key(arg: &str, key: &str) -> bool {
    match json::parse(str) {
        Ok(json_value) => match json_value {
            Object(object) => match object.get(key) {
                Some(_) => true  // JSON object that contains key
                None => false    // JSON object that does contain key
            },
            _ => false  // not a object (must be another JSON type)
        },
        Err(_) => false  // not valid JSON
    }
}
```

Using the matcher then requires binding it to a parameter (using `p!`) and passing it to a mock assertion method, like so:

```rust
fn ensure_current_time_field_is_returned() {
    // GIVEN:
    let mut mock_sender = MockResponseSender::default();

    // WHEN:
    request_handler(&mock_sender);

    // THEN:
    // we expect a "time" field to be in the response JSON
    assert(response_sender.send_response.called_with_pattern(
        p!(is_json_object_with_key, "time")
    ));
    // we DO NOT expect a "time" field to be in the response JSON
    assert(!response_sender.send_response.called_with_pattern(
        p!(is_json_object_with_key, "records")
    ));
}
```

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

If a `trait` provides a method that returns a type that _doesn't_ implement `Default`, then one must generate the mock using `mock_trait_no_default!`. This macro generates a mock that doesn't implement `Default`. Clients must construct instances of the generated mock using `MockTrait::new()`, manually specifying the default return values for each method.

For example:

```rust
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
fn generate_sequence(func: &dyn Fn(i32) -> i32, min: i32, max: i32) -> Vec<i32> {
    // exclusive range
    (min..max).map(func).collect()
}
```

This iterates through a range of integers, mapping each integer to another integer using the supplied transformation function, `func`.

Rather than generate your own mock transformation function boilerplate when testing `generate_sequence`, one can use the macro `mock_func!`. This macro generates a `double::Mock` object and a closure that wraps it for you. For example:

```rust
#[macro_use]
extern crate double;

fn generate_sequence(func: &dyn Fn(i32) -> i32, min: i32, max: i32) -> Vec<i32> {
    // exclusive range
    (min..max).map(func).collect()
}

fn test_function_used_correctly() {
    // GIVEN:
    mock_func!(
        mock,     // name of variable that stores mock object
        mock_fn,  // name of variable that stores closure wrapper
        i32,      // return value type
        i32);     // argument1 type
    mock.use_closure(Box::new(|x| x * 2));

    // WHEN:
    let sequence = generate_sequence(&mock_fn, 1, 5);

    // THEN:
    assert_eq!(vec!(2, 4, 6, 8), sequence);
    assert!(mock.has_calls_exactly(vec!(
      1, 2, 3, 4
    )));
}

fn main() {
    test_function_used_correctly();
}
```

You specify the variable names that should store the generated mock object and closure in the first two arguments of the `mock_func!` macro.

If the function's return type does not implement `Default`, then one must use the `mock_func_no_default!` macro, like so:

```rust
// ...

fn test_function_with_custom_defaults() {
    // GIVEN:
    mock_func_no_default!(
        mock,
        mock_fn,
        i32,   // return value type
        42,    // default return value
        i32);  // argument1 type
    mock.use_closure_for((3), Box::new(|x| x * 2));

    // WHEN:
    let sequence = generate_sequence(&mock_fn, 1, 5);

    // THEN:
    assert_eq!(vec!(42, 42, 6, 42), sequence);
    assert!(mock.has_calls_exactly(vec!(
      1, 2, 3, 4
    )));
}

fn main() {
    test_function_with_custom_defaults();
}
```
