#[macro_use]
extern crate double;

// Traits which only return types that implement `Default`.
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

// Traits which return types that do not implement `Default`.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    name: String
}

pub trait UserStore {
    fn get_user(&self, id: i32) -> Result<User, String>;
    fn delete_user(&self, id: i32) -> Result<(), String>;
}

mock_trait_no_default!(
    MockUserStore,
    get_user(i32) -> Result<User, String>,
    delete_user(i32) -> Result<(), String>);

impl UserStore for MockUserStore {
    mock_method!(get_user(&self, id: i32) -> Result<User, String>);
    mock_method!(delete_user(&self, id: i32) -> Result<(), String>);
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

    // Test using mocks that do not implement the `Default` trait.
    // One must manually specify the default values for all methods in the
    // mocked trait.
    let store = MockUserStore::new(
        Err("cannot get, no user with given ID".to_owned()),
        Err("cannot delete, no user with given ID".to_owned()));

    store.get_user.return_value_for(
        42,
        Ok(User{ name: "Donald".to_owned() }));
    assert_eq!(
        Err("cannot get, no user with given ID".to_owned()),
        store.get_user(10));
    assert_eq!(
        Ok(User{ name: "Donald".to_owned() }),
        store.get_user(42));

    store.delete_user.return_value_for(42, Ok(()));
    assert_eq!(
        Err("cannot delete, no user with given ID".to_owned()),
        store.delete_user(10));
    assert_eq!(Ok(()), store.delete_user(42));
}
