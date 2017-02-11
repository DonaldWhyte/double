use std::any::TypeId;

trait Decay {
    type Type;
}

impl<T> Decay for T {
    default type Type = T;
}

impl<'a, T> Decay for &'a T {
    type Type = <T as Decay>::Type;
}

impl<'a, T> Decay for &'a mut T {
    type Type = <T as Decay>::Type;
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use super::Decay;

macro_rules! decay_type {
    (str) => (String);
    (& str) => (String);
    (& mut str) => (String);
    (&& str) => (String);
    (&& mut str) => (String);
    ($typename:tt) => ($typename);
    (& $typename:tt) => ($typename);
    (& mut $typename:tt) => ($typename);
    (&& $typename:tt) => ($typename);
    (&& mut $typename:tt) => ($typename);
}


    struct Test {
        one: <i32 as Decay>::Type,
        two: <&i32 as Decay>::Type,
        three: decay_type!(i32),
        four: decay_type!(&i32)
    }
/*
    #[test]
    fn decaying_value_type() {
        assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay_type!(i32)>());
    }

    #[test]
    fn decaying_reference_type() {
        assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay_type!(&i32)>());
    }

    #[test]
    fn decaying_mutable_reference_type() {
        assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay_type!(&mut i32)>());
    }

    #[test]
    fn decaying_static_type() {
        //assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay_type!(&'static i32)>());
    }

    #[test]
    fn decaying_double_reference_type() {
        assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay_type!(&& i32)>());
    }

    #[test]
    fn decaying_double_mutable_reference_type() {
        assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay_type!(&&mut i32)>());
    }

    #[test]
    fn decaying_value_str() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay_type!(str)>());
    }

    #[test]
    fn decaying_reference_str() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay_type!(&str)>());
    }

    #[test]
    fn decaying_mutable_reference_str() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay_type!(&mut str)>());
    }

    #[test]
    fn decaying_static_str() {
        //assert_eq!(TypeId::of::<i32>(), TypeId::of::<decay_type!(&'static str)>());
    }

    #[test]
    fn decaying_double_reference_str() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay_type!(&& str)>());
    }

    #[test]
    fn decaying_double_mutable_reference_str() {
        assert_eq!(TypeId::of::<String>(), TypeId::of::<decay_type!(&&mut str)>());
    }
*/
}