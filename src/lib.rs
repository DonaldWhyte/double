macro_rules! mock_method_type {
    ($fname:ident, $retval:ty, $($self_prefix_token:tt)+, $($arg_name:ident: $arg_type:ty)*) => (
        pub struct $fname {
            // TODO
        }

        impl $fname {
            pub fn new() -> $fname {
                $fname {
                    // TODO
                }
            }

            pub fn call() -> $retval {

            }
        }
    )
}

macro_rules! mock_trait {
    ($trait_name:ident, $mock_name:ident,
     $(fn $fname:ident(($($self_prefix_token:tt)+) self,
                       $($arg_name:ident: $arg_type:ty,)*)
                       -> $retval:ty),*
    ) =>
    (

        mod $mock_name {
            use $trait_name;

            pub mod method_types {
                $(mock_method_type!(
                    $fname, $retval,
                    $($self_prefix_token)+,
                    $($arg_name: $arg_type)*))*;
            }

            pub struct Methods {
                $(pub $fname: method_types::$fname)*
            }

            impl Methods {
                pub fn new() -> Methods {
                    Methods {
                        $($fname: method_types::$fname::new())*
                    }
                }
            }

            pub struct Mock {
                pub m: Methods
            }

            impl Mock {
                pub fn new() -> Mock {
                    Mock {
                        m: Methods::new()
                    }
                }
            }

            impl $trait_name for Mock {
                $(
                    fn $fname($($self_prefix_token)+ self, $($arg_name: $arg_type)*) -> $retval {
                        //self.m.$fname.call($($arg_name,)*);
                        self.m.$fname.call();
                    }
                )*
            }
        }
    )
}

pub trait FileWriter {
    fn write_contents(&mut self, filename: &str, contents: &str);
}

/*

MockFunction class --> handles call counts, args, expectations, rerturn values, etc.\

MockTrait -> wraps each MockFunction, just calling the underlying mock function
  // struct contains instance of wach MockFunction
  // public function to access raw MockFunction to set expectaitons/retvals
  // args are: trait name, mock struct name, type names of the MockFunctions

*/

macro_rules! match_args_name_and_type {
    ($($arg_name:ident: $arg_type:ty),*) => ()
}

match_args_name_and_type!(foo: i32);
match_args_name_and_type!(foo: i32, bar: &str);

#[cfg(test)]
mod tests {
    use self::super::*;
    use std;

    mock_trait!(
        FileWriter,
        SomeMock,
        fn write_contents((&mut) self, filename: &str, contents: &str) -> ()
    );

    #[test]
    fn it_works() {
        let mut mock = SomeMock::Mock::new();
        mock.write_contents("test.txt", "Hello, World!");
    }

}
