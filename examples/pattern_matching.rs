#[macro_use]
extern crate double;

pub trait ProfitForecaster {
    fn profit_at(&self, timestamp: i32) -> f64;
    fn write_report_for(&self, timestamp: i32, dry_run: bool);
}

mock_trait!(
    MockForecaster,
    profit_at(i32) -> f64,
    write_report_for(i32, bool) -> ());
impl ProfitForecaster for MockForecaster {
    mock_method!(profit_at(&self, timestamp: i32) -> f64);
    mock_method!(write_report_for(&self, timestamp: i32, dry_run: bool));
}

macro_rules! matcher {
    // TODO: no arg

    // TODO: split into separate matchers and add & manually if possible

    ( $arg_matchers:tt ) => (
        |args| -> bool { match_impl(args, ($arg_matchers)); }
    )
}

fn match_impl<A>(arg: &A, arg_matcher: &Fn(&A) -> bool) -> bool {
    arg_matcher(arg)
}

fn match_impl2<A, B>(
    args: &(
        A,
        B,
    ),
    arg_matchers: (
        &Fn(&A) -> bool,
        &Fn(&B) -> bool,
    )) -> bool
{
    let matches = vec!(
        arg_matchers.0(&args.0),
        arg_matchers.1(&args.1),
    );
    !matches.iter().any(|is_match| !is_match)
}

/*)
macro_rules! matcher {
    ( $($matcher:tt)* ) => (
        |args| -> bool {
            // TODO: short-circuit the matcher checks
            let results = vec!($(
                $matcher(args.$i)
            )*);
            !result.iter.any(|is_match| !is_match)
        }
    );

    // base case
    () => (
        |args| -> bool { true }
    );
}*/

macro_rules! bind {
    ( $func:ident ) => (
        |&potential_match| -> bool { $func(&potential_match) }
    );

    ( $func:ident, $args:tt ) => (
        |&potential_match| -> bool { $func(&potential_match, $args) }
    );
}

fn equal<T: PartialEq>(arg: &T, target_val: T) -> bool {
    *arg == target_val
}

fn any<T>(_: &T) -> bool {
    true
}

fn assert_called_with<T>(args: &Vec<T>, matcher: &Fn(&T) -> bool) -> bool {
    for arg in args {
        if matcher(&arg) == true {
            return true;
        }
    }
    return false;
}

fn main() {
    assert!(assert_called_with(
        &vec!(0, 1, 2),  // all arguments
        &bind!(equal, 1)));

    //let m = matcher!(bind!(equal, 42));
    //let m = |args| -> bool { match_impl(args, ( bind!(equal, 42)) ) };

    let forecaster = MockForecaster::default();
    forecaster.profit_at(42);
    forecaster.profit_at(84);
    forecaster.write_report_for(42, true);
    forecaster.write_report_for(84, true);
    forecaster.write_report_for(42, false);

    let profit_at_matches = forecaster.profit_at.compute_matches(
        // matcher!(bind!(equal, 42));
        &|args| -> bool {
            match_impl(args, &bind!(equal, 42))
        }
    );
    println!("profit_at {:?}", profit_at_matches);

    let write_report_for_matches = forecaster.write_report_for.compute_matches(
        // matcher!(bind!(equal, 42));
        &|args| -> bool {
            match_impl2(args, ( &bind!(equal, 42), &bind!(equal, false) ))
        }
    );
    println!("write_report_for {:?}", write_report_for_matches);

}
