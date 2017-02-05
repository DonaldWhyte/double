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
                Default::default()
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

pub trait TestTrait {
    fn no_arg(&self);
    fn no_arg_mut(&mut self);
    fn one_arg(&self, a: i32);
    fn one_arg_mut(&mut self, a: i32);
    fn two_args(&self, a: i32, b: &str);
    fn two_args_mut(&mut self, a: i32, b: &str);
    fn three_args(&self, a: i32, b: &str, c: &mut Vec<i32>);
    fn three_args_mut(&mut self, a: i32, b: &str, c: &mut Vec<i32>);

    fn no_arg_retval(&self) -> Vec<String>;
    fn no_arg_mut_retval(&mut self) -> Vec<String>;
    fn one_arg_retval(&self, a: i32) -> Vec<String>;
    fn one_arg_mut_retval(&mut self, a: i32) -> Vec<String>;
    fn two_args_retval(&self, a: i32, b: &str) -> Vec<String>;
    fn two_args_mut_retval(&mut self, a: i32, b: &str) -> Vec<String>;
    fn three_args_retval(&self, a: i32, b: &str, c: &mut Vec<i32>) -> Vec<String>;
    fn three_args_mut_retval(&mut self, a: i32, b: &str, c: &mut Vec<i32>) -> Vec<String>;
}

mock_trait!(
    TestTrait,
    MockOfTrait,
    fn no_arg((&)self) -> (),
    fn no_arg_mut((&mut)self) -> (),
    fn one_arg((&)self, a: i32) -> (),
    fn one_arg_mut((&mut)self, a: i32) -> (),
    fn two_args((&)self, a: i32, b: &str) -> (),
    fn two_args_mut((&mut)self, a: i32, b: &str) -> (),
    fn three_args((&)self, a: i32, b: &str, c: &mut Vec<i32>) -> (),
    fn three_args_mut((&mut)self, a: i32, b: &str, c: &mut Vec<i32>) -> (),
    fn no_arg_retval((&)self) -> Vec<String>,
    fn no_arg_mut_retval((&mut)self) -> Vec<String>,
    fn one_arg_retval((&)self, a: i32) -> Vec<String>,
    fn one_arg_mut_retval((&mut)self, a: i32) -> Vec<String>,
    fn two_args_retval((&)self, a: i32, b: &str) -> Vec<String>,
    fn two_args_mut_retval((&mut)self, a: i32, b: &str) -> Vec<String>,
    fn three_args_retval((&)self, a: i32, b: &str, c: &mut Vec<i32>) -> Vec<String>,
    fn three_args_mut_retval((&mut)self, a: i32, b: &str, c: &mut Vec<i32>) -> Vec<String>
);


pub trait Empty {}
mock_trait!(Empty, EmptyMock);

fn test() {
    let empty_mocks = EmptyMock::new();
    let mock = MockOfTrait::new();
}

#[cfg(test)]
mod tests {
    use self::super::*;

    #[test]
    fn it_works() {
        super::test();
    }

}
