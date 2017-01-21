macro_rules! mock_trait {
    ($trait_name:ident, $struct_name:ident, $(fn $fname:ident($($arg:tt)*) -> $retval:ty $body:block),*) => {
        struct $struct_name {
            pub call_counts:
                std::cell::RefCell<std::collections::HashMap<String, i32>>,
            pub call_expectations:
                std::cell::RefCell<std::collections::HashMap<String, i32>>
        }

        impl $struct_name {
            fn build_func_call_map(func_names: Vec<&str>, initial_value: i32) -> std::collections::HashMap<String, i32> {
                let mut call_map = std::collections::HashMap::new();
                for func in func_names {
                    call_map.insert(func.to_string(), initial_value);
                }
                call_map
            }

            fn new() -> $struct_name {
                let funcs = vec!($($fname)* );
                let initial_call_counts = $struct_name::build_func_call_map(funcs, 0);
                let initial_call_expectations = $struct_name::build_func_call_map(funcs, -1);
                $struct_name {
                    call_counts: std::cell::RefCell::new(initial_call_counts),
                    call_expectations: std::cell::RefCell::new(initial_call_expectations),
                }
            }
        }


    }
}

pub trait FileWriter {
    fn write_contents(&mut self, filename: &str, contents: &str);
}

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
        let mut mock = SomeMock::new();
        mock.write_contents("test.txt", "Hello, World!");
    }

}
