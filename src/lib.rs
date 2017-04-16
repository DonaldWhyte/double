//! Double is a fully-featured mocking library for mocking `trait`
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
//! fn double_profit(revenue: u32, costs: u32, balance_sheet: &BalanceSheet) -> i32 {
//!     balance_sheet.profit(revenue, costs) * 2
//! }
//!
//! // Test which uses a mock BalanceSheet
//! mock_trait!(
//!     MockBalanceSheet,
//!     profit(u32, u32) -> i32);
//! impl BalanceSheet for MockBalanceSheet {
//!     mock_method!(profit(&self, revenue: u32, costs: u32) -> i32);
//! }
//!
//! fn test_doubling_a_sheets_profit() {
//!     // GIVEN:
//!     let sheet = MockBalanceSheet::default();
//!     sheet.profit.return_value(250);
//!     // WHEN:
//!     let profit = double_profit(500, 250, &sheet);
//!     // THEN:
//!     // mock return 250, which was double
//!     assert_eq!(500, profit);
//!     // assert that the revenue and costs were correctly passed to the mock
//!     sheet.profit.has_calls_exactly_in_order(vec!((500, 250)));
//! }
//!
//! // Executing test
//! fn main() {
//!     test_doubling_a_sheets_profit();
//! }
//! ```

pub use mock::Mock;

pub type Double<C, R> = Mock<C, R>;

mod macros;
mod mock;
