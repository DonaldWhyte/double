include!(concat!(env!("OUT_DIR"), "/matcher_generated.rs"));

// TODO: better name and document purpose
#[macro_export]
macro_rules! arg {
    ( $func:ident ) => (
        |&potential_match| -> bool { $func(&potential_match) }
    );

    ( $func:ident, $args:tt ) => (
        |&potential_match| -> bool { $func(&potential_match, $args) }
    );
}

// COMMON MATCHERS (TODO: move somewhere else)
pub fn equal<T: PartialEq>(arg: &T, target_val: T) -> bool {
    *arg == target_val
}

pub fn any<T>(_: &T) -> bool {
    true
}
