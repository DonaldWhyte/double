/*

Requirements for mock methods:

* counting calls
* counting order of calls
* be able to match exact call
*
* group of args + expected operator (<=, =, >=), sexpected count, expected actions
* list of calls, stored in call sequence, each containing:
    - group of args

How to represent matchers? Some ideas:

* function object

*/

enum CallCount {
    Never,
    Once,
    Exactly(u32),
    AtLeast(u32),
    AtMost(u32),
    Between(u32, u32)
}

macro_rules! mock_method_type {
    ( $method_name:ident, $retval:ty $( , $arg_name:ident: $arg_type:ty )* ) => (
        pub mod $method_name {
            use std::vec::Vec;
            use CallCount;

            type ReturnValue = $retval;

            struct Expectation {
                count: CallCount
                $(, $arg_name: fn($arg_type) -> bool)*
            }

            impl Expectation {
                pub fn new(count: CallCount $(, $arg_name: fn($arg_type) -> bool)*)
                    -> Expectation
                {
                    Expectation {
                        count: count
                        $(, $arg_name: $arg_name)*
                    }
                }
            }

            pub struct Method {
                expectations: Vec<Expectation>
            }

            impl Method {
                pub fn new() -> Method {
                    Method {
                        expectations: vec![]
                    }
                }

                #[allow(unused_variables)]
                pub fn call(&mut self $(, $arg_name: $arg_type)*) -> $retval {
                    println!("{} called", stringify!($method_name));
                    $(
                        println!("\t{:?}", $arg_name);
                    )*
                    Default::default()
                }

                #[allow(unused_variables)]
                pub fn expect(&mut self, call_count: CallCount
                              $(, $arg_name: fn($arg_type) -> bool)*)
                {
                    self.expectations.push(
                        Expectation::new(call_count $(, $arg_name)*)
                    );
                }

                pub fn check_and_clear_expectations(&mut self) {
                    // TODO: have unordered check by default
                    // TODO: checked ordered if some flag is set
                }
            }

            impl Drop for Method {
                fn drop(&mut self) {
                    println!("{} dropped", stringify!($method_name));
                    self.check_and_clear_expectations();
                }
            }
        }
    )
}

macro_rules! mock_trait {
    (
        $trait_name:ident,
        $mock_name:ident
        $(, fn $method_name:ident(
            ($($self_prefix_token:tt)+) self $( , $arg_name:ident: $arg_type:ty )*
          ) -> $retval:ty
        )*
    ) => (
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        mod $mock_name {
            use std;
            use $trait_name;

            mod method_types {
                $(
                    mock_method_type!(
                        $method_name, $retval $(, $arg_name: $arg_type)*);
                )*
            }

            struct Methods {
                $(pub $method_name: method_types::$method_name::Method),*
            }

            impl Methods {
                pub fn new() -> Methods {
                    Methods {
                        $($method_name: method_types::$method_name::Method::new()),*
                    }
                }
            }

            pub struct Mock {
                m: std::cell::RefCell<Methods>
            }

            impl Mock {
                pub fn new() -> Mock {
                    Mock {
                        m: std::cell::RefCell::new(Methods::new())
                    }
                }
            }

            impl $trait_name for Mock {
                $(
                    fn $method_name($($self_prefix_token)+ self
                                    $(, $arg_name: $arg_type)*) -> $retval
                    {
                        self.m.borrow_mut().$method_name.call($($arg_name,)*)
                    }
                )*
            }

            pub fn new() -> Mock {
                Mock::new()
            }
        }
    )
}

// Test traits
// TODO: move to test module
trait EmptyTrait {

}

trait SimpleTrait {
    fn a_method(&mut self, a: i32, b: &str);
}

trait ComplexTrait {
    fn no_arg(&self);
    fn no_arg_mut(&mut self);
    fn one_arg(&self, a: i32);
    fn one_arg_mut(&mut self, a: i32);
    fn two_args(&self, a: i32, b: &str);
    fn two_args_mut(&mut self, a: i32, b: &str);
    fn three_args(&self, a: i32, b: &str, c: &mut Vec<i32>);
    fn three_args_mut(&mut self, a: i32, b: &str, c: &mut Vec<i32>);

    fn no_arg_retval(&self) -> u8;
    fn no_arg_mut_retval(&mut self) -> i8;
    fn one_arg_retval(&self, a: i32) -> u16;
    fn one_arg_mut_retval(&mut self, a: i32) -> i16;
    fn two_args_retval(&self, a: i32, b: &str) -> String;
    fn two_args_mut_retval(&mut self, a: i32, b: &str) -> Vec<i32>;
    fn three_args_retval(&self, a: i32, b: &str, c: &mut Vec<i32>) -> Vec<String>;
    fn three_args_mut_retval(&mut self, a: i32, b: &str, c: &mut Vec<i32>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use SimpleTrait;
    use ComplexTrait;

    mock_trait!(EmptyTrait, EmptyMock);

    mock_trait!(
        SimpleTrait,
        MockSimple,
        fn a_method((&mut)self, a: i32, b: &str) -> ()
    );

    mock_trait!(
        ComplexTrait,
        MockComplex,
        fn no_arg((&)self) -> (),
        fn no_arg_mut((&mut)self) -> (),
        fn one_arg((&)self, a: i32) -> (),
        fn one_arg_mut((&mut)self, a: i32) -> (),
        fn two_args((&)self, a: i32, b: &str) -> (),
        fn two_args_mut((&mut)self, a: i32, b: &str) -> (),
        fn three_args((&)self, a: i32, b: &str, c: &mut Vec<i32>) -> (),
        fn three_args_mut((&mut)self, a: i32, b: &str, c: &mut Vec<i32>) -> (),

        fn no_arg_retval((&)self) -> u8,
        fn no_arg_mut_retval((&mut)self) -> i8,
        fn one_arg_retval((&)self, a: i32) -> u16,
        fn one_arg_mut_retval((&mut)self, a: i32) -> i16,
        fn two_args_retval((&)self, a: i32, b: &str) -> String,
        fn two_args_mut_retval((&mut)self, a: i32, b: &str) -> Vec<i32>,
        fn three_args_retval((&)self, a: i32, b: &str, c: &mut Vec<i32>) -> Vec<String>,
        fn three_args_mut_retval((&mut)self, a: i32, b: &str, c: &mut Vec<i32>) -> Vec<String>
    );

    #[test]
    fn mocking_empty_trait() {
        EmptyMock::new();
    }

    #[test]
    fn mocking_simple_trait() {
        let mut mock = MockSimple::new();
        mock.a_method(42, "hello");
    }

    #[test]
    fn mocking_trait_with_all_method_variances() {
        let mut empty_vec = vec![];
        let mut vec_with_one_elem = vec![1];
        let mut vec_with_multiple_elems = vec![2, 3, 4];
        let mut another_vec = vec![5, 6, 7];

        let mut mock = MockComplex::new();
        mock.no_arg();
        mock.no_arg_mut();
        mock.one_arg(1);
        mock.one_arg_mut(2);
        mock.two_args(3, "donald");
        mock.two_args_mut(4, "whyte");
        mock.three_args(5, "is", &mut empty_vec);
        mock.three_args_mut(6, "alive", &mut vec_with_one_elem);

        println!("{:?}", mock.no_arg_retval());
        println!("{:?}", mock.no_arg_mut_retval());
        println!("{:?}", mock.one_arg_retval(7));
        println!("{:?}", mock.one_arg_mut_retval(8));
        println!("{:?}", mock.two_args_retval(9, "my"));
        println!("{:?}", mock.two_args_mut_retval(10, "name"));
        println!("{:?}", mock.three_args_retval(11, "is", &mut vec_with_multiple_elems));
        println!("{:?}", mock.three_args_mut_retval(12, "donald", &mut another_vec));
    }

}
