macro_rules! decay {
    (str) => (String);
    (& str) => (String);
    (& mut str) => (String);
    (&& str) => (String);
    (&& mut str) => (String);
    (&'static str) => (String);
    (&'static mut str) => (String);
    (&&'static str) => (String);
    (&&'static mut str) => (String);
    ($typename:tt) => ($typename);
    (& $typename:tt) => ($typename);
    (& mut $typename:tt) => ($typename);
    (&& $typename:tt) => ($typename);
    (&& mut $typename:tt) => ($typename);
    (&'static $typename:tt) => ($typename);
    (&'static mut $typename:tt) => ($typename);
    (&&'static $typename:tt) => ($typename);
    (&&'static mut $typename:tt) => ($typename);
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    // Compile-time tests. These ensure that the decay macro works correctly
    // when using it to *declare* varialbes with decayed types.
    //
    // The tests ensure both primitive, string and complex types are properly
    // decayed. If they are not properly decayed, buidling the test will produce
    // a compiler error because the reference types have to explicit lifetime
    // (required when declaring struct fields as references).

    #[test]
    fn test_decaying_integral_types_in_struct_definition() {
        #[allow(dead_code)]
        struct IntegralTypeDecay {
            value: decay!(i32),
            reference: decay!(&i32),
            mut_ref: decay!(&mut i32),
            double_ref: decay!(&&i32),
            double_mutable_ref: decay!(&&mut i32),
            static_ref: decay!(&'static i32),
            static_mutable_ref: decay!(&'static mut i32),
            double_static_ref: decay!(&&'static i32),
            double_static_mutable_ref: decay!(&&'static mut i32)
        }
    }

    #[test]
    fn test_decaying_str_types_in_struct_definition() {
        #[allow(dead_code)]
        struct StrTypeDecay {
            value: decay!(str),
            reference: decay!(&str),
            mut_ref: decay!(&mut str),
            double_ref: decay!(&&str),
            double_mutable_ref: decay!(&&mut str),
            static_ref: decay!(&'static str),
            static_mutable_ref: decay!(&'static mut str),
            double_static_ref: decay!(&&'static str),
            double_static_mutable_ref: decay!(&&'static mut str)
        }
    }

    #[test]
    fn test_decaying_string_types_in_struct_definition() {
        #[allow(dead_code)]
        struct StringTypeDecay {
            value: decay!(String),
            reference: decay!(&String),
            mut_ref: decay!(&mut String),
            double_ref: decay!(&&String),
            double_mutable_ref: decay!(&&mut String),
            static_ref: decay!(&'static String),
            static_mutable_ref: decay!(&'static mut String),
            double_static_ref: decay!(&&'static String),
            double_static_mutable_ref: decay!(&&'static mut String)
        }
    }

    #[test]
    fn test_decaying_struct_types_in_struct_definition() {
        #[allow(dead_code)]
        struct Person {
            name: String,
            age: u8
        }

        #[allow(dead_code)]
        struct StructTypeDecay {
            value: decay!(Person),
            reference: decay!(&Person),
            mut_ref: decay!(&mut Person),
            double_ref: decay!(&&Person),
            double_mutable_ref: decay!(&&mut Person),
            static_ref: decay!(&'static Person),
            static_mutable_ref: decay!(&'static mut Person),
            double_static_ref: decay!(&&'static Person),
            double_static_mutable_ref: decay!(&&'static mut Person)
        }
    }

    #[test]
    fn decaying_value_type() {
        assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay!(i32)>());
    }

    #[test]
    fn decaying_ref_type() {
        assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay!(&i32)>());
    }

    #[test]
    fn decaying_mutable_ref_type() {
        assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay!(&mut i32)>());
    }

    #[test]
    fn decaying_double_ref_type() {
        assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay!(&& i32)>());
    }

    #[test]
    fn decaying_double_mutable_ref_type() {
        assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay!(&&mut i32)>());
    }

    #[test]
    fn decaying_static_reference_type() {
        assert_eq!(
            TypeId::of::<i32>(),
            TypeId::of::<decay!(&'static i32)>());
    }

    #[test]
    fn decaying_static_mutable_reference_type() {
        assert_eq!(
            TypeId::of::<i32>(),
            TypeId::of::<decay!(&'static mut i32)>());
    }

    #[test]
    fn decaying_double_static_reference_type() {
        assert_eq!(
            TypeId::of::<i32>(),
            TypeId::of::<decay!(&&'static i32)>());
    }

    #[test]
    fn decaying_value_str() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay!(str)>());
    }

    #[test]
    fn decaying_ref_str() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay!(&str)>());
    }

    #[test]
    fn decaying_mutable_ref_str() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay!(&mut str)>());
    }

    #[test]
    fn decaying_double_ref_str() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay!(&& str)>());
    }

    #[test]
    fn decaying_double_mutable_ref_str() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay!(&&mut str)>());
    }

    #[test]
    fn decaying_static_reference_str() {
        assert_eq!(
            TypeId::of::<String>(),
            TypeId::of::<decay!(&'static str)>());
    }

    #[test]
    fn decaying_static_mutable_reference_str() {
        assert_eq!(
            TypeId::of::<String>(),
            TypeId::of::<decay!(&'static mut str)>());
    }

    #[test]
    fn decaying_double_static_reference_str() {
        assert_eq!(
            TypeId::of::<String>(),
            TypeId::of::<decay!(&&'static str)>());
    }

    #[test]
    fn decaying_value_string() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay!(String)>());
    }

    #[test]
    fn decaying_ref_string() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay!(&String)>());
    }

    #[test]
    fn decaying_mutable_ref_string() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay!(&mut String)>());
    }

    #[test]
    fn decaying_double_ref_string() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay!(&& String)>());
    }

    #[test]
    fn decaying_double_mutable_ref_string() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay!(&&mut String)>());
    }

    #[test]
    fn decaying_static_reference_string() {
        assert_eq!(
            TypeId::of::<String>(),
            TypeId::of::<decay!(&'static String)>());
    }

    #[test]
    fn decaying_static_mutable_reference_string() {
        assert_eq!(
            TypeId::of::<String>(),
            TypeId::of::<decay!(&'static mut String)>());
    }

    #[test]
    fn decaying_double_static_reference_string() {
        assert_eq!(
            TypeId::of::<String>(),
            TypeId::of::<decay!(&&'static String)>());
    }

    #[allow(dead_code)]
    struct Point {
        x: f64,
        y: f64
    }

    #[test]
    fn decaying_value_struct() {
        assert_eq!(TypeId::of::<Point>(), TypeId::of::<decay!(Point)>());
    }

    #[test]
    fn decaying_ref_struct() {
        assert_eq!(TypeId::of::<Point>(), TypeId::of::<decay!(&Point)>());
    }

    #[test]
    fn decaying_mutable_ref_struct() {
        assert_eq!(TypeId::of::<Point>(), TypeId::of::<decay!(&mut Point)>());
    }

    #[test]
    fn decaying_double_ref_struct() {
        assert_eq!(TypeId::of::<Point>(), TypeId::of::<decay!(&& Point)>());
    }

    #[test]
    fn decaying_double_mutable_ref_struct() {
        assert_eq!(TypeId::of::<Point>(), TypeId::of::<decay!(&&mut Point)>());
    }

    #[test]
    fn decaying_static_reference_struct() {
        assert_eq!(
            TypeId::of::<Point>(),
            TypeId::of::<decay!(&'static Point)>());
    }

    #[test]
    fn decaying_static_mutable_reference_struct() {
        assert_eq!(
            TypeId::of::<Point>(),
            TypeId::of::<decay!(&'static mut Point)>());
    }

    #[test]
    fn decaying_double_static_reference_struct() {
        assert_eq!(
            TypeId::of::<Point>(),
            TypeId::of::<decay!(&&'static Point)>());
    }

}