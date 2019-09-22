#[macro_use]
extern crate double;

// Code under test
trait BalanceSheet {
    fn profit(&self, revenue: u32, costs: u32) -> i32;
}

fn double_profit(revenue: u32, costs: u32, balance_sheet: &dyn BalanceSheet) -> i32 {
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
        Ok("default_user_name".to_owned()));
    // WHEN:
    let result = mock.get_username(10001);
    // THEN:
    assert_eq!(Ok("default_username".to_owned()), result);
}

// Executing tests
fn main() {
    test_doubling_a_sheets_profit();
    test_manually_setting_default_retval();
}
