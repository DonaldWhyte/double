# Double

### Full-featured mocking library in Rust, including rich failure messages and argument matchers.

[![Build Status](https://travis-ci.org/DonaldWhyte/double.svg?branch=master)](https://travis-ci.org/DonaldWhyte/double) [![Docs](https://docs.rs/double/badge.svg)](https://docs.rs/double)

Based off [**iredelmeier's**](https://github.com/iredelmeier/) initial mock implementation.

Double lets you mock `Trait` implementations so that you can track function call arguments and set return values or overrides functions at test time.

Here's a quick example:

```rust
extern crate double;

use double::Double;

trait Foo: Clone {
    fn expensive_fn(&self, x: i64, y: i64) -> i64;
}

#[derive(Clone)]
struct DoubleFoo {
    pub expensive_fn: Double<(i64, i64), i64>,
}

impl Foo for DoubleFoo {
    fn expensive_fn(&self, x: i64, y: i64) -> i64 {
        self.expensive_fn.call((x + 10, y))
    }
}

fn double_expensive_fn<T: Foo>(foo: &T, x: i64, y: i64) -> i64 {
    foo.expensive_fn(x, y) * 2
}

#[test]
fn doubles_return_value() {
    let mock = DoubleFoo { expensive_fn: Double::default() };

    mock.expensive_fn.return_value(1000);

    assert_eq!(double_expensive_fn(&mock, 1, 2), 2000);
}

#[test]
fn uses_correct_args() {
    let mock = DoubleFoo { expensive_fn: Double::default() };

    assert!(!mock.expensive_fn.called());

    double_expensive_fn(&mock, 1, 2);

    assert_eq!(mock.expensive_fn.num_calls(), 1);
    assert!(mock.expensive_fn.called_with((11, 2)));
}
```

More examples are available in the [examples directory](./examples).

### Credits

This library is based off [**iredelmeier's**](https://github.com/iredelmeier/) initial mocking implementation. Massive thanks to her implementation for inpsiring me to work on this. The repo for her library [**can be found here**](https://github.com/iredelmeier/pseudo).

If you're interested in testing, check out her other repositories too!
