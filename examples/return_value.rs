extern crate double;

use double::Mock;

trait Dependency: Clone {
    fn profit(&self, revenue: u32, costs: u32) -> i32;
}

#[derive(Debug, Clone)]
struct MockDependency {
    pub profit: Mock<(u32, u32), (i32)>,
}

impl Dependency for MockDependency {
    fn profit(&self, revenue: u32, costs: u32) -> i32 {
        self.profit.call((revenue, costs))
    }
}

impl Default for MockDependency {
    fn default() -> Self {
        MockDependency { profit: Mock::default() }
    }
}

fn main() {
    let mock = MockDependency::default();
    mock.profit.return_value(42);
    mock.profit.return_value_for((0, 0), 9001);

    let value = mock.profit(10, 20);
    assert_eq!(42, value);
    mock.profit.has_calls_exactly_in_order(vec!((10, 20)));

    let value = mock.profit(0, 0);
    assert_eq!(9001, value);
    mock.profit.has_calls_exactly_in_order(vec!((10, 20), (0, 0)));
}