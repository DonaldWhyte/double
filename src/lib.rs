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
//! #[macro_use]
//! extern crate double;
//!
//! // Code under test
//! trait BalanceSheet {
//!     fn profit(&self, revenue: u32, costs: u32) -> i32;
//! }
//!
//! fn get_profit(revenue: u32, costs: u32, balance_sheet: &BalanceSheet) -> i32 {
//!     balance_sheet.profit(revenue, costs)
//! }
//!
//! // Tests which uses a mock BalanceSheet
//! mock_trait!(
//!     MockBalanceSheet,
//!     profit(u32, u32) -> i32);
//! impl BalanceSheet for MockBalanceSheet {
//!     mock_method!(profit(&self, revenue: u32, costs: u32) -> i32);
//! }
//!
//! fn test_weighting_is_applied() {
//!     // GIVEN:
//!     let sheet = MockBalanceSheet::default();
//!     sheet.profit.return_value(250);
//!     // WHEN:
//!     let profit = get_profit(500, 250, &sheet);
//!     // THEN:
//!     assert_eq!(250, profit);
//! }
//!
//! fn test_correct_args_passed_to_balance_sheet() {
//!     // GIVEN:
//!     let sheet = MockBalanceSheet::default();
//!     // WHEN:
//!     let _ = get_profit(500, 250, &sheet);
//!     // THEN:
//!     sheet.profit.has_calls_exactly_in_order(vec!((500, 250)));
//! }
//!
//! // Executing tests
//! fn main() {
//!     test_weighting_is_applied();
//!     test_correct_args_passed_to_balance_sheet();
//! }
//! ```

pub use mock::Mock;

pub type Double<C, R> = Mock<C, R>;

mod macros;
mod mock;
