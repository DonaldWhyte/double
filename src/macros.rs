#[macro_export]
macro_rules! mock_trait {
    ($mock_name:ident $(, $method:ident($($arg_type:ty),* ) -> $retval:ty )* ) => (
        #[derive(Debug, Clone)]
        struct $mock_name {
            $(
                $method: double::Mock<(($($arg_type),*)), $retval>
            ),*
        }

        impl Default for $mock_name {
            fn default() -> Self {
                $mock_name {
                    $(
                        $method: double::Mock::default()
                    ),*
                }
            }
        }
    );
}

#[macro_export]
macro_rules! mock_method {
    // immutable, no return value, no type parameter, no body
    ( $method:ident(&self, $($arg_name:ident: $arg_type:ty),*)) => (
        fn $method(&self, $($arg_name: $arg_type),*) {
            self.$method.call(($($arg_name),*))
        }
    );
    // immutable, no return value, no type parameter, body
    ( $method:ident(&self, $($arg_name:ident: $arg_type:ty),*), $sel:ident, $body:tt ) => (
        fn $method(&$sel, $($arg_name: $arg_type),*) $body
    );
    // immutable, no return value, type parameter, no body
    ( $method:ident<($($type_params: tt)*)>(&self, $($arg_name:ident: $arg_type:ty),*) ) => (
            fn $method<$($type_params)*>(&$sel, $($arg_name: $arg_type),*) {
                self.$method.call(($($arg_name),*))
            }
    );
    // immutable, no return value, type parameter, body
    ( $method:ident<($($type_params: tt)*)>(&self, $($arg_name:ident: $arg_type:ty),*),
        $sel:ident, $body:tt) => (
            fn $method<$($type_params)*>(&$sel, $($arg_name: $arg_type),*) $body
    );
    // immutable, return value, no type parameter, no body
    ( $method:ident(&self, $($arg_name:ident: $arg_type:ty),*) -> $retval:ty ) => (
        fn $method(&self, $($arg_name: $arg_type),*) -> $retval {
            self.$method.call(($($arg_name),*))
        }
    );
    // immutable, return value, no type parameter, body
    ( $method:ident(&self, $($arg_name:ident: $arg_type:ty),*) -> $retval:ty, $sel:ident, $body:tt ) => (
        fn $method(&$sel, $($arg_name: $arg_type),*) -> $retval $body
    );
    // immutable, return value, type parameter, no body
    ( $method:ident<($($type_params: tt)*)>(&self, $($arg_name:ident: $arg_type:ty),*)
        -> $retval:ty ) => (
            fn $method<$($type_params)*>(&$sel, $($arg_name: $arg_type),*) -> $retval {
                self.$method.call(($($arg_name),*))
            }
    );
    // immutable, return value, type parameter, body
    ( $method:ident<($($type_params: tt)*)>(&self, $($arg_name:ident: $arg_type:ty),*)
        -> $retval:ty, $sel:ident, $body:tt ) => (
            fn $method<$($type_params)*>(&$sel, $($arg_name: $arg_type),*) -> $retval $body
    );
    // mutable, no return value, no type parameter, no body
    ( $method:ident(&mut self, $($arg_name:ident: $arg_type:ty),*)) => (
        fn $method(&mut self, $($arg_name: $arg_type),*) {
            self.$method.call(($($arg_name),*))
        }
    );
    // mutable, no return value, no type parameter, body
    ( $method:ident(&mut self, $($arg_name:ident: $arg_type:ty),*), $sel:ident, $body:tt ) => (
        fn $method(&mut $sel, $($arg_name: $arg_type),*) $body
    );
    // mutable, no return value, type parameter, no body
    ( $method:ident<($($type_params: tt)*)>(&mut self, $($arg_name:ident: $arg_type:ty),*) ) => (
            fn $method<$($type_params)*>(&mut $sel, $($arg_name: $arg_type),*) {
                self.$method.call(($($arg_name),*))
            }
    );
    // mutable, no return value, type parameter, body
    ( $method:ident<($($type_params: tt)*)>(&mut self, $($arg_name:ident: $arg_type:ty),*),
        $sel:ident, $body:tt) => (
            fn $method<$($type_params)*>(&mut $sel, $($arg_name: $arg_type),*) $body
    );
    // mutable, return value, no type parameter, no body
    ( $method:ident(&mut self, $($arg_name:ident: $arg_type:ty),*) -> $retval:ty ) => (
        fn $method(&mut self, $($arg_name: $arg_type),*) -> $retval {
            self.$method.call(($($arg_name),*))
        }
    );
    // mutable, return value, no type parameter, body
    ( $method:ident(&mut self, $($arg_name:ident: $arg_type:ty),*) -> $retval:ty, $sel:ident, $body:tt ) => (
        fn $method(&mut $sel, $($arg_name: $arg_type),*) -> $retval $body
    );
    // mutable, return value, type parameter, no body
    ( $method:ident<($($type_params: tt)*)>(&mut self, $($arg_name:ident: $arg_type:ty),*)
        -> $retval:ty ) => (
            fn $method<$($type_params)*>(&mut $sel, $($arg_name: $arg_type),*) -> $retval {
                self.$method.call(($($arg_name),*))
            }
    );
    // mutable, return value, type parameter, body
    ( $method:ident<($($type_params: tt)*)>(&mut self, $($arg_name:ident: $arg_type:ty),*)
        -> $retval:ty, $sel:ident, $body:tt ) => (
            fn $method<$($type_params)*>(&mut $sel, $($arg_name: $arg_type),*) -> $retval $body
    );
}
