extern crate double;

use double::mock_method;
use double::mock_trait;

fn main() {
    pub trait A {
        fn foo(&self);
    }

    mock_trait!(
        MockA,
        foo() -> ()
    );
    impl A for MockA {
        mock_method!(foo(&self));
    }
    let mock = MockA::default();
    let _ = Box::new(mock) as Box<A + Send>;
}
