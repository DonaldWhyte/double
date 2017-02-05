/*

MockFunction class --> handles call counts, args, expectations, rerturn values, etc.\

MockTrait -> wraps each MockFunction, just calling the underlying mock function
  // struct contains instance of wach MockFunction
  // public function to access raw MockFunction to set expectaitons/retvals
  // args are: trait name, mock struct name, type names of the MockFunctions

*/

macro_rules! mock_method_type {
    ( $fname:ident, $retval:ty $( , $arg_name:ident: $arg_type:ty )* ) => (
        #[allow(non_camel_case_types)]
        pub struct $fname {
            // TODO
        }

        impl $fname {
            pub fn new() -> $fname {
                $fname {
                    // TODO
                }
            }

            #[allow(unused_variables)]
            pub fn call(&mut self $(, $arg_name: $arg_type)*) -> $retval {
                println!("{} called", stringify!($fname));
                $(
                    println!("\t{:?}", $arg_name);
                )*
                Default::default()
            }
        }

        impl Drop for $fname {
            fn drop(&mut self) {
                println!("{} dropped", stringify!($fname));
            }
        }
    )
}

macro_rules! mock_trait {
    (
        $trait_name:ident,
        $mock_name:ident
        $(, fn $fname:ident(
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
                        $fname, $retval $(, $arg_name: $arg_type)*);
                )*
            }

            struct Methods {
                $(pub $fname: method_types::$fname),*
            }

            impl Methods {
                pub fn new() -> Methods {
                    Methods {
                        $($fname: method_types::$fname::new()),*
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
                    fn $fname($($self_prefix_token)+ self $(, $arg_name: $arg_type)*) -> $retval {
                        self.m.borrow_mut().$fname.call($($arg_name,)*)
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
