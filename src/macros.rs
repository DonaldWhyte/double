// Private macros. They need to be exported and made public so they can be used
// in the actual public facing macros. Ideally these would be inaccessible to
// clients, but since that's not possible, we at least make it explicit that
// these are intended to be private by prepending the macro names with
// "__private".
#[macro_export]
macro_rules! __private_mock_trait_default_impl {
    ($mock_name:ident $(, $method:ident)*) => (
         impl Default for $mock_name {
            fn default() -> Self {
                Self {
                    $( $method: double::Mock::default() ),*
                }
            }
        }
    );
}

#[macro_export]
macro_rules! __private_mock_trait_new_impl {
    ($mock_name:ident $(, $method:ident: $retval: ty)*) => (
        impl $mock_name {
            #[allow(dead_code)]
            pub fn new( $($method: $retval),* ) -> Self {
                Self {
                    $( $method: double::Mock::new($method) ),*
                }
            }
        }
    );
}

/// Macro that generates a `struct` implementation of a trait.
///
/// Use this instead of `mock_trait_no_default!` if all mocked method return
/// types implement `Default`. If one or more of the return types do not
/// implement `Default`, then `mock_trait_no_default!` must be used to generate
/// the mock.
///
/// This macro generates a `struct` that implements the traits `Clone`, `Debug`
/// and `Default`. Create instances of the mock object by calling the
/// `struct`'s `default()` method, or specify custom default return values for
/// each mocked method using `new()`.
///
/// The `struct` has a field for each method of the `trait`, which manages
/// their respective method's behaviour and call expectations. For example, if
/// one defines a mock like so:
///
/// ```
/// # #[macro_use] extern crate double;
///
/// mock_trait!(
///     MockTaskManager,
///     max_threads(()) -> u32,
///     set_max_threads(u32) -> ()
/// );
///
/// # fn main() {
///     // only here to make `cargo test` happy
/// }
/// ```
///
/// Then the following code is generated:
///
/// ```
/// #[derive(Debug, Clone)]
/// struct MockTaskManager {
///     max_threads: double::Mock<(), u32>,
///     set_max_threads: double::Mock<(u32), ()>,
/// }
///
/// impl Default for MockTaskManager {
///     fn default() -> Self {
///         MockTaskManager {
///             max_threads: double::Mock::default(),
///             set_max_threads: double::Mock::default(),
///         }
///     }
/// }
/// ```
///
/// Note that just defining this macro is not enough. This macro is used to
/// generate the necessary boilerplate, but the generated struct *does not*
/// implement the desired `trait`. To do that, use `double`'s `mock_method`
/// macro.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate double;
///
/// trait TaskManager {
///    fn max_threads(&self) -> u32;
///    fn set_max_threads(&mut self, max_threads: u32);
/// }
///
/// mock_trait!(
///     MockTaskManager,
///     max_threads(()) -> u32,
///     set_max_threads(u32) -> ()
/// );
///
/// # fn main() {
/// let mock = MockTaskManager::default();
/// mock.max_threads.return_value(42u32);
/// assert_eq!(42, mock.max_threads.call(()));
/// mock.set_max_threads.call(9001u32);
/// assert!(mock.set_max_threads.called_with(9001u32));
/// # }
/// ```
#[macro_export]
macro_rules! mock_trait {
    ($mock_name:ident $(, $method:ident($($arg_type:ty),* ) -> $retval:ty )* ) => (
        #[derive(Debug, Clone)]
        struct $mock_name {
            $(
                $method: double::Mock<(($($arg_type),*)), $retval>
            ),*
        }

        __private_mock_trait_new_impl!($mock_name $(, $method: $retval)*);
        __private_mock_trait_default_impl!($mock_name $(, $method)*);
    );

    (pub $mock_name:ident $(, $method:ident($($arg_type:ty),* ) -> $retval:ty )* ) => (
        #[derive(Debug, Clone)]
        pub struct $mock_name {
            $(
                $method: double::Mock<(($($arg_type),*)), $retval>
            ),*
        }

        __private_mock_trait_new_impl!($mock_name $(, $method: $retval)*);
        __private_mock_trait_default_impl!($mock_name $(, $method)*);
    );
}

/// Macro that generates a `struct` implementation of a trait.
///
/// Use this instead of `mock_trait!` if one or more of the return types do not
/// implement `Default`. If all return types implement `Default`, then it's
/// more convenient to use `mock_trait!`, since you instantiate mock objects
/// using `default()`,
///
/// This macro generates a `struct` that implements the traits `Clone` and
/// and `Debug`. Create instances of the mock object by calling `new()`,
/// passing in the return values for each mocked method using `new()`.
///
/// The `struct` has a field for each method of the `trait`, which manages
/// their respective method's behaviour and call expectations. For example, if
/// one defines a mock like so:
//
/// ```
/// # #[macro_use] extern crate double;
///
/// // `Result` does not implement `Default`.
/// mock_trait_no_default!(
///     MockTaskManager,
///     max_threads(()) -> Result<u32, String>,
///     set_max_threads(u32) -> ()
/// );
///
/// # fn main() {
///     // only here to make `cargo test` happy
/// }
/// ```
///
/// Then the following code is generated:
///
/// ```
/// #[derive(Debug, Clone)]
/// struct MockTaskManager {
///     max_threads: double::Mock<(), Result<u32, String>>,
///     set_max_threads: double::Mock<(u32), ()>,
/// }
///
/// impl MockTaskManager {
///     pub fn new(max_threads: Result<u32, String>, set_max_threads: ()) -> Self {
///         MockTaskManager {
///             max_threads: double::Mock::new(max_threads),
///             set_max_threads: double::Mock::new(set_max_threads),
///         }
///     }
/// }
/// ```
///
/// Note that just defining this macro is not enough. This macro is used to
/// generate the necessary boilerplate, but the generated struct *does not*
/// implement the desired `trait`. To do that, use `double`'s `mock_method`
/// macro.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate double;
///
/// trait TaskManager {
///    fn max_threads(&self) -> Result<u32, String>;
///    fn set_max_threads(&mut self, max_threads: u32);
/// }
///
/// mock_trait_no_default!(
///     MockTaskManager,
///     max_threads(()) -> Result<u32, String>,
///     set_max_threads(u32) -> ()
/// );
///
/// # fn main() {
/// let mock = MockTaskManager::new(Ok(42), ());
/// assert_eq!(Ok(42), mock.max_threads.call(()));
/// mock.set_max_threads.call(9001u32);
/// assert!(mock.set_max_threads.called_with(9001u32));
/// # }
/// ```
#[macro_export]
macro_rules! mock_trait_no_default {
    ($mock_name:ident $(, $method:ident($($arg_type:ty),* ) -> $retval:ty )* ) => (
        #[derive(Debug, Clone)]
        struct $mock_name {
            $(
                $method: double::Mock<(($($arg_type),*)), $retval>
            ),*
        }

        __private_mock_trait_new_impl!($mock_name $(, $method: $retval)*);
    );

    (pub $mock_name:ident $(, $method:ident($($arg_type:ty),* ) -> $retval:ty )* ) => (
        #[derive(Debug, Clone)]
        pub struct $mock_name {
            $(
                $method: double::Mock<(($($arg_type),*)), $retval>
            ),*
        }

        __private_mock_trait_new_impl!($mock_name $(, $method: $retval)*);
    );
}

/// Macro that generates a mock implementation of a `trait` method.
///
/// This should be used to implement a `trait` on a mock type generated by
/// `double`'s `mock_trait` macro. If one has generated a mock `struct` using
/// `mock_trait`, then the actual *implementation* of the desired trait can be
/// auto-generated using `mock_method`, like so:
///
/// ```
/// # #[macro_use] extern crate double;
///
/// trait TaskManager {
///    fn max_threads(&self) -> u32;
///    fn set_max_threads(&mut self, max_threads: u32);
/// }
///
/// mock_trait!(
///     MockTaskManager,
///     max_threads(()) -> u32,
///     set_max_threads(u32) -> ()
/// );
///
/// // Actually implement the trait that should be mocked
/// impl TaskManager for MockTaskManager {
///     mock_method!(max_threads(&self) -> u32);
///     mock_method!(set_max_threads(&mut self, max_threads: u32));
/// }
///
/// # fn main() {
/// let mut mock = MockTaskManager::default();
/// mock.max_threads.return_value(42u32);
/// assert_eq!(42, mock.max_threads());
/// assert!(mock.max_threads.called_with(()));
/// mock.set_max_threads(9001u32);
/// assert!(mock.set_max_threads.called_with(9001u32));
/// # }
/// ```
///
/// There are many different variants of `mock_method`. In total there are 12
/// variants. 8 variants provides a combination of the following:
///
/// 1. const method (`&self`) **or** mutable method (`&mut self`)
/// 2. return value (`fn foo(&self) -> bool`) **or** no return value (`fn foo(&self)`)
/// 3. automatically generated method body **or** custom method body
///
/// (1) allows both constant and mutable methods tobe mocked, like in the
/// `MockTaskManager` example above.
///
/// (2) is for convenience. It means one doesn't have to specify `-> ()`
/// explicitly for mocked methods that don't return values. This can also be
/// shown in the `MockTaskManager` example. Notice how the return type is not
/// specified when generating the `set_max_threads()` mock.
///
/// (3) is useful when the stored call arguments' types (defined by the
/// `mock_trait()` macro) are different to the mocked method. There are cases
/// where type differences in the stored args and the actual method args are
/// required. For example, suppose you had the following trait:
///
/// ```
/// trait TextStreamWriter {
///     fn write(text: &str);
/// }
/// ```
///
/// A mock can't _store_ received `text` arguments as `&str` because the mock
/// needs to the _own_ the given arguments (and `&str` is a non-owning
/// reference). Therefore, the mock trait has to be specified like so:
///
/// ```
/// # #[macro_use] extern crate double;
///
/// trait TextStreamWriter {
///     fn write(&mut self, text: &str);
/// }
///
/// mock_trait!(
///     MockTextStreamWriter,
///     // have to use `String`, not `&str` here, since `&str` is a reference
///     write(String) -> ()
/// );
///
/// impl TextStreamWriter for MockTextStreamWriter {
///     mock_method!(write(&mut self, text: &str), self, {
///         // manually convert the reference to an owned `String` before
///         // passing it to the underlying mock object
///         self.write.call(text.to_owned())
///     });
/// }
/// # fn main() {
///     // only here to make `cargo test` happy
/// }
/// ```
///
/// Using variant (3) of `mock_method` means we specify the body of the
/// generated function manually. The custom body simply converts the `&str`
/// argument to an owned string and passes it into the underlying `write` `Mock`
/// object manually. (normally auto-generated bodies do this for you).
///
/// The name of the underlying mock object is always the same as the mocked
/// method's name.
///
/// `&str` parameters are common. It can be inconvenient haven't to manually
/// specify the body each time they appear. There are plans to add a macro to
/// generate a body that calls `to_owned()` automatically.
/// (TODO: implement the macro)
///
/// ### Type Parameters
///
/// There are an additional 4 variants to handle method type parameters
/// (e.g. `fn foo<T: Eq>(&self, a: &T)`). These variants allow one to generate
/// mock methods which take some generic type parameters.
///
/// For example, suppose one had a `Comparator` trait that was responsible for
/// comparing any two values in the program. It might look something like this:
///
/// ```
/// trait Comparator {
///    fn is_equal<T: Eq>(&self, a: &T, b: &T) -> bool;
/// }
/// ```
///
/// `T` can be multiple types. Currently, we cannot store call arguments that
/// have generic types in the underlying `Mock` objects. Therefore, one has to
/// convert the generic types to a different, common representation. One way
/// to get around this limitation is converting each generic type to a `String`.
/// e.g. for the `Comparator` trait:
///
/// ```
/// # #[macro_use] extern crate double;
///
/// use std::string::ToString;
///
/// trait Comparator {
///    fn is_equal<T: Eq + ToString>(&self, a: &T, b: &T) -> bool;
/// }
///
/// mock_trait!(
///     MockComparator,
///     // store all passed in call args as strings
///     is_equal((String, String)) -> bool
/// );
///
/// impl Comparator for MockComparator {
///     mock_method!(is_equal<(T: Eq + ToString)>(&self, a: &T, b: &T) -> bool, self, {
///         // Convert both arguments to strings and manually pass to underlying
///         // mock object.
///         // Notice how the both arguments as passed as a single tuple. The
///         // underlying mock object always expects a single tuple.
///         self.is_equal.call((a.to_string(), b.to_string()))
///     });
/// }
/// # fn main() {
///     // only here to make `cargo test` happy
/// }
/// ```
///
/// If the `to_string` conversions for all `T` are not lossy, then our mock
/// expectations can be exact. If the `to_string` conversions _are_ lossy, then
/// this mechanism can still be used, providing all the properties of the passed
/// in objects are captured in the resultant `String`s.
///
/// This approach requires the writer to ensure the code under test adds the
/// `ToString` trait to the `trait`'s type argument constraints. This limitation
/// forces test writers to modify production code to use `double` for mocking.
///
/// Despite this, there is still value in using `double` for mocking generic
/// methods with type arguments. Despite adding boilerplate to production code
/// and manually implementing mock method bodies being cumbersome, the value add
/// is that all argument matching, expectations, calling test functions, etc.
/// are all still handled by `double`. Arguably, reimplenting those features is
/// more cumbersome than the small amount of boilerplate required to mock
/// methods with type arguments.
#[macro_export]
macro_rules! mock_method {

    // immutable, no return value, no type parameter, no body
    ( $method:ident(&self $(,$arg_name:ident: $arg_type:ty)*)) => (
        fn $method(&self $(,$arg_name: $arg_type)*) {
            self.$method.call(($($arg_name.clone()),*))
        }
    );

    // immutable, no return value, no type parameter, body
    ( $method:ident(&self $(,$arg_name:ident: $arg_type:ty)*), $sel:ident, $body:tt ) => (
        fn $method(&$sel $(,$arg_name: $arg_type)*) $body
    );

    // immutable, no return value, type parameter, no body
    // not provided, since type parameters need a custom body 99% of the time

    // immutable, no return value, type parameter, body
    ( $method:ident<($($type_params: tt)*)>(&self $(,$arg_name:ident: $arg_type:ty)*),
        $sel:ident, $body:tt) => (
            fn $method<$($type_params)*>(&$sel $(,$arg_name: $arg_type)*) $body
    );

    // immutable, return value, no type parameter, no body
    ( $method:ident(&self $(,$arg_name:ident: $arg_type:ty)*) -> $retval:ty ) => (
        fn $method(&self $(,$arg_name: $arg_type)*) -> $retval {
            self.$method.call(($($arg_name.clone()),*))
        }
    );

    // immutable, return value, no type parameter, body
    ( $method:ident(&self $(,$arg_name:ident: $arg_type:ty)*) -> $retval:ty, $sel:ident, $body:tt ) => (
        fn $method(&$sel $(,$arg_name: $arg_type)*) -> $retval $body
    );

    // immutable, return value, type parameter, no body
    // not provided, since type parameters need a custom body 99% of the time

    // immutable, return value, type parameter, body
    ( $method:ident<($($type_params: tt)*)>(&self $(,$arg_name:ident: $arg_type:ty)*)
        -> $retval:ty, $sel:ident, $body:tt ) => (
            fn $method<$($type_params)*>(&$sel $(,$arg_name: $arg_type)*) -> $retval $body
    );

    // mutable, no return value, no type parameter, no body
    ( $method:ident(&mut self $(,$arg_name:ident: $arg_type:ty)*)) => (
        fn $method(&mut self $(,$arg_name: $arg_type)*) {
            self.$method.call(($($arg_name.clone()),*))
        }
    );

    // mutable, no return value, no type parameter, body
    ( $method:ident(&mut self $(,$arg_name:ident: $arg_type:ty)*), $sel:ident, $body:tt ) => (
        fn $method(&mut $sel $(,$arg_name: $arg_type)*) $body
    );

    // mutable, no return value, type parameter, no body
    // not provided, since type parameters need a custom body 99% of the time

    // mutable, no return value, type parameter, body
    ( $method:ident<($($type_params: tt)*)>(&mut self $(,$arg_name:ident: $arg_type:ty)*),
        $sel:ident, $body:tt) => (
            fn $method<$($type_params)*>(&mut $sel $(,$arg_name: $arg_type)*) $body
    );

    // mutable, return value, no type parameter, no body
    ( $method:ident(&mut self $(,$arg_name:ident: $arg_type:ty)*) -> $retval:ty ) => (
        fn $method(&mut self $(,$arg_name: $arg_type)*) -> $retval {
            self.$method.call(($($arg_name.clone()),*))
        }
    );

    // mutable, return value, no type parameter, body
    ( $method:ident(&mut self $(,$arg_name:ident: $arg_type:ty)*) -> $retval:ty, $sel:ident, $body:tt ) => (
        fn $method(&mut $sel $(,$arg_name: $arg_type)*) -> $retval $body
    );

    // mutable, return value, type parameter, no body
    // not provided, since type parameters need a custom body 99% of the time

    // mutable, return value, type parameter, body
    ( $method:ident<($($type_params: tt)*)>(&mut self $(,$arg_name:ident: $arg_type:ty)*)
        -> $retval:ty, $sel:ident, $body:tt ) => (
            fn $method<$($type_params)*>(&mut $sel $(,$arg_name: $arg_type)*) -> $retval $body
    );

}
