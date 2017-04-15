#[macro_use]
extern crate double;

trait Calculator: Clone {
    fn multiply(&self, x: i32, y: i32) -> i32;
}

trait BalanceSheet: Clone {
    fn profit(&self, revenue: u32, costs: u32) -> i32;
    fn loss(&self, revenue: u32, costs: u32) -> i32;
}

trait Greeter: Clone {
    fn greet<S: AsRef<str>>(&mut self, name: S);
}

mock_trait!(EmptyMock);

mock_trait!(
    MockCalculator,
    multiply(i32, i32) -> i32);
impl Calculator for MockCalculator {
    mock_method!(multiply(&self, x: i32, y: i32) -> i32);
}

mock_trait!(
    MockBalanceSheet,
    profit(u32, u32) -> i32,
    loss(u32, u32) -> i32);
impl BalanceSheet for MockBalanceSheet {
    mock_method!(profit(&self, revenue: u32, costs: u32) -> i32);
    mock_method!(loss(&self, revenue: u32, costs: u32) -> i32);
}

mock_trait!(
    MockGreeter,
    greet(String) -> ());
impl Greeter for MockGreeter {
    mock_method!(greet<(S: AsRef<str>)>(&mut self, name: S), self, {
        self.greet.call(name.as_ref().to_string());
    });
}

fn main() {
    // Test individual return values
    let mock = MockBalanceSheet::default();
    mock.profit.return_value(42);
    mock.profit.return_value_for((0, 0), 9001);

    let value = mock.profit(10, 20);
    assert_eq!(42, value);
    mock.profit.has_calls_exactly_in_order(vec!((10, 20)));

    let value = mock.profit(0, 0);
    assert_eq!(9001, value);
    mock.profit.has_calls_exactly_in_order(vec!((10, 20), (0, 0)));

    // Test sequence of return values
    mock.profit.return_values(vec!(1, 2, 3));
    assert_eq!(1, mock.profit.call((1, 2)));
    assert_eq!(2, mock.profit.call((2, 4)));
    assert_eq!(3, mock.profit.call((3, 6)));
    assert_eq!(42, mock.profit.call((4, 8)));
}
