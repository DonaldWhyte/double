# Double

### Full-featured mocking library in Rust, including rich failure messages and argument matchers.

[![Build Status](https://travis-ci.org/DonaldWhyte/double.svg?branch=master)](https://travis-ci.org/DonaldWhyte/double) [![Docs](https://docs.rs/double/badge.svg)](https://docs.rs/double)

Based off [**iredelmeier's**](https://github.com/iredelmeier/) initial mock implementation.

Double lets you mock `Trait` implementations so that you can track function call arguments and set return values or overrides functions at test time.

Here's a quick example:

```rust
#[macro_use]
extern crate double;

// Code under test
trait BalanceSheet {
    fn profit(&self, revenue: u32, costs: u32) -> i32;
}

fn get_profit(revenue: u32, costs: u32, balance_sheet: &BalanceSheet) -> i32 {
    balance_sheet.profit(revenue, costs)
}

// Tests which uses a mock BalanceSheet
mock_trait!(
    MockBalanceSheet,
    profit(u32, u32) -> i32);
impl BalanceSheet for MockBalanceSheet {
    mock_method!(profit(&self, revenue: u32, costs: u32) -> i32);
}

fn test_weighting_is_applied() {
    // GIVEN:
    let sheet = MockBalanceSheet::default();
    sheet.profit.return_value(250);
    // WHEN:
    let profit = get_profit(500, 250, &sheet);
    // THEN:
    assert_eq!(250, profit);
}

fn test_correct_args_passed_to_balance_sheet() {
    // GIVEN:
    let sheet = MockBalanceSheet::default();
    // WHEN:
    let _ = get_profit(500, 250, &sheet);
    // THEN:
    sheet.profit.has_calls_exactly_in_order(vec!((500, 250)));
}

// Executing tests
fn main() {
    test_weighting_is_applied();
    test_correct_args_passed_to_balance_sheet();
}
```

More examples are available in the [examples directory](./examples).
