# Double

### Full-featured mocking library in Rust, including rich failure messages and argument matchers.

[![Build Status](https://travis-ci.org/DonaldWhyte/double.svg?branch=master)](https://travis-ci.org/DonaldWhyte/double) [![Docs](https://docs.rs/double/badge.svg)](https://docs.rs/double)

Based off [**iredelmeier's**](https://github.com/iredelmeier/) initial mock implementation.

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
    // mock return 250, which was double
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
