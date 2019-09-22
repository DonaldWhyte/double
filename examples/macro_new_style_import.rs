// NOTE: This example is identical to macros.rs. The only difference is that
// this example imports the double macros using the newer, rust 2018 approach.
// macros.rs uses the legacy #[macro_use] approach. This one imports the macros
// being used directly.

extern crate double;

use double::mock_method;
use double::mock_trait;
use double::mock_trait_no_default;

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

}
