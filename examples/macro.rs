extern crate double;

trait Dependency: Clone {
    fn profit(&self, revenue: u32, costs: u32) -> i32;
}

trait Greeter: Clone {
    fn greet<S: AsRef<str>>(&mut self, name: S);
}

macro_rules! mock_trait {
    ( $mock_name:ident, $($method:ident($retval:ty)($($arg_type:ty),*))* ) => (
        #[derive(Debug, Clone)]
        struct $mock_name {
            $(
                $method: double::Mock<(($($arg_type),*)), $retval>
            )*
        }

        impl Default for $mock_name {
            fn default() -> Self {
                $mock_name {
                    $(
                        $method: double::Mock::default(),
                    )*
                }
            }
        }
    );
}

macro_rules! mock_method {
    ( $method:ident($retval:ty)(&self, $($arg_name:ident: $arg_type:ty),*) ) => (
        fn $method(&self, $($arg_name: $arg_type),*) -> $retval {
            self.$method.call(($($arg_name),*))
        }
    );
    ( $method:ident($retval:ty)(&mut self, $($arg_name:ident: $arg_type:ty),*) ) => (
        fn $method(&mut self, $($arg_name: $arg_type),*) -> $retval {
            self.$method.call(($($arg_name),*))
        }
    )
}

mock_trait!(MockDependency, profit(i32)(u32, u32));
impl Dependency for MockDependency {
    mock_method!(profit(i32)(&self, revenue: u32, costs: u32));
}

mock_trait(MockGreeter, greet(())(String));

fn main() {
    // Test individual return values
    let mock = MockDependency::default();
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
