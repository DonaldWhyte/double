use std::collections::HashMap;
use std::cell::Cell;

fn build_func_call_map(func_names: Vec<&str>, initial_value: i32) -> HashMap<&str, i32> {
    let mut call_map = HashMap::new();
    for func in func_names {
        call_map.insert(func, initial_value);
    }
    call_map
}

fn test() {
    let map = build_func_call_map(vec!("foo", "bar"), -1);
    for (func, value) in &map {
        println!("{}: {}", func, value);
    }
}

macro_rules! define_struct {
    ($mock_name:ident, $( $func_name:ident ),*) =>
    (
        struct $mock_name {
            pub call_counts: HashMap<String, i32>,
            pub call_expectations: HashMap<String, i32>
        }

        impl $mock_name {
            fn new() {
                let funcs = vec!($($func_name)*, $last_func);
                let initial_call_counts = build_func_call_map(funcs, 0);
                let initial_call_expectations = build_func_call_map(funcs, -1);
                $mock_name {
                    call_counts: initial_call_counts,
                    call_expectations: initial_call_expectations,
                }
            }
        }
    );
}
/*

macro_rules! mock_trait {
    ($mock_name:ident, $trait_to_implement:ident,
     $($func_name:ident),*) =>
    (
        define_struct!($mock_name, $($func_name)*);

        impl $trait_to_implement for $mock_name {
            $(
                fn $func_name() {
                    // TODO: update call state, exepctations and return values
                }
            )*
        }
    );
}

macro_rules! mock_func {
    ($func_name:ident) => ($func_name, ());
    ($func_name:ident, $return_type:ty) => ($func_name, $return_type);
}
*/
#[cfg(test)]


// WORKS
macro_rules! make_func {
    ($func_name:ident, $return_val:ty, $($arg: ident: $ty: ty),*) => {
        fn $func_name ( $($arg: $ty),* ) -> $return_val {
            println!("{}", "hello");
        }
    }
}

// DOES NOT WORK
macro_rules! make_method {
    ($func_name:ident, $return_val:ty, $($arg: ident: $ty: ty),*) => {
        fn $func_name(&self, $($arg: $ty),* ) -> $return_val {
            self.count += 1;
        }
    }
}

macro_rules! make_method_test {
    ($func_name:ident, $return_val:ty, $($arg: ident: $ty: ty),*) => (
        pub struct Foo {
            count: Cell<i32>
        }

        impl Foo {
            fn new() -> Foo {
                Foo {
                    count: Cell::new(0)
                }
            }

            fn $func_name(&self, $($arg: $ty),* ) -> $return_val {
                self.count.get();
                return 0; // TODO: return some set value or a defdault value
            }
        }
    );
}

make_method_test!(foo, u32, one: i32, two: i32);

mod tests {
    extern crate spectral;
    use self::spectral::prelude::*;
    use self::super::*;

    #[test]
    fn it_works() {
        //define_struct!(Test, foo, bar);
        make_func!(bar, (), one: i32, two: u32);
        bar(3, 5);

        let instance = Foo::new();
        instance.foo(2, 2);

        //let test = Test::new();
        //assert_that(&test.call_counts).contains_key(&"hello").is_equal_to(&"hi");
    }

}
