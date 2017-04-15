//! Double is a fully-featured mocking library for mocking `Trait`
//! implementations.
//!
//! The `Double` struct tracks function call arguments and specifies return
//! values or function overrides.
//!
//! This library is based off iredelmeier's initial mocking implementation.
//! Massive thanks to her implementation for inpsiring me to work on this. Her
//! repo can be found here:
//!
//! https://github.com/iredelmeier/pseudo
//!
//! If you're interested in testing, check out her other repositories too!
//!
//! # Examples
//!
//! ```
//! use double::Double;
//!
//! trait Foo: Clone {
//!     fn expensive_fn(&self, x: i64, y: i64) -> i64;
//! }
//!
//! #[derive(Clone)]
//! struct DoubleFoo {
//!     pub expensive_fn: Double<(i64, i64), i64>,
//! }
//!
//! impl Foo for DoubleFoo {
//!     fn expensive_fn(&self, x: i64, y: i64) -> i64 {
//!         self.expensive_fn.call((x + 10, y))
//!     }
//! }
//!
//! fn double_expensive_fn<T: Foo>(foo: &T, x: i64, y: i64) -> i64 {
//!     foo.expensive_fn(x, y) * 2
//! }
//!
//! fn test_doubles_return_value() {
//!     let mock = DoubleFoo { expensive_fn: Double::default() };
//!
//!     mock.expensive_fn.return_value(1000);
//!
//!     assert_eq!(double_expensive_fn(&mock, 1, 2), 2000);
//! }
//!
//! fn test_uses_correct_args() {
//!     let mock = DoubleFoo { expensive_fn: Double::default() };
//!
//!     assert!(!mock.expensive_fn.called());
//!
//!     double_expensive_fn(&mock, 1, 2);
//!
//!     assert_eq!(mock.expensive_fn.num_calls(), 1);
//!     assert!(mock.expensive_fn.called_with((11, 2)));
//! }
//!
//! test_doubles_return_value();
//! test_uses_correct_args();
//! ```

pub use mock::Mock;

pub type Double<C, R> = Mock<C, R>;

mod macros;
mod mock;
