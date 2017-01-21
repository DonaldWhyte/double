macro_rules! mock_method_type_name {
    ($mock_name:ident, $fname:ident) => ($mock_name::method_types::$fname)
}

macro_rules! mock_method_type {
    ($fname:ident, $retval:ty, $($arg:tt)*) => (
        pub struct $fname {
            // TODO
        }

        impl $fname {
            pub fn new() -> $fname {
                $fname {
                    // TODO
                }
            }
        }
    )
}

macro_rules! mock_trait {
    ($trait_name:ident, $mock_name:ident, $(fn $fname:ident($($arg:tt)*) -> $retval:ty $body:block),*) => (
        mod $mock_name {
            use $trait_name;

            pub mod method_types {
                $(mock_method_type!($fname, $retval, $($arg)*))*;
            }

            pub struct Methods {
                $(pub $fname: method_types::$fname)*
            }

            impl Methods {
                pub fn new() -> Methods {
                    // TODO
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
                $(fn $fname($($arg)*) -> $retval $body)*
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

#[cfg(test)]
mod tests {
    use self::super::*;
    use std;

    mock_trait!(
        FileWriter,
        SomeMock,
        fn write_contents(&mut self, filename: &str, contents: &str) -> () {}
    );

    #[test]
    fn it_works() {
        let mut mock = SomeMock::Mock::new();
        mock.write_contents("test.txt", "Hello, World!");
    }

}
