#[macro_use]
extern crate double;

use double::matcher::*;

pub trait ProfitForecaster {
    fn profit_at(&self, timestamp: i32) -> f64;
    fn write_report_for(&self, timestamp: i32, dry_run: bool);
    fn store_forecast_result(&self, result: Result<u32, String>);
}
mock_trait!(
    MockForecaster,
    profit_at(i32) -> f64,
    write_report_for(i32, bool) -> (),
    store_forecast_result(Result<u32, String>) -> ());
impl ProfitForecaster for MockForecaster {
    mock_method!(profit_at(&self, timestamp: i32) -> f64);
    mock_method!(write_report_for(&self, timestamp: i32, dry_run: bool));
    mock_method!(store_forecast_result(&self, result: Result<u32, String>));
}

fn main() {
    let forecaster = MockForecaster::default();
    forecaster.profit_at(42);
    forecaster.profit_at(84);
    forecaster.write_report_for(42, true);
    forecaster.write_report_for(84, true);
    forecaster.write_report_for(42, false);

    assert!(forecaster.profit_at.called_with_pattern(
        matcher!( p!(eq, 42) )
    ));
    assert!(!forecaster.profit_at.called_with_pattern(
        matcher!( p!(gt, 84) )
    ));
    assert!(forecaster.profit_at.called_with_pattern(
        matcher!( p!(between_inc, 42, 84) )
    ));
    assert!(!forecaster.profit_at.called_with_pattern(
        matcher!( p!(between_exc, 42, 84) )
    ));

    assert!(forecaster.profit_at.called_with_pattern(
        matcher!( p!(not, p!(gt, 84)) )
    ));
    assert!(!forecaster.profit_at.called_with_pattern(
        matcher!( p!(not, p!(gt, 0)) )
    ));

    assert!(forecaster.profit_at.called_with_pattern(
        matcher!( p!(all_of, vec!(p!(gt, 40), p!(lt, 90))) )
    ));
    assert!(!forecaster.profit_at.called_with_pattern(
        matcher!( p!(all_of, vec!(p!(gt, 40), p!(lt, 42))) )
    ));

    assert!(forecaster.profit_at.called_with_pattern(
        matcher!( p!(any_of, vec!(p!(lt, 100), p!(gt, 200))) )
    ));
    assert!(!forecaster.profit_at.called_with_pattern(
        matcher!( p!(any_of, vec!(p!(lt, 5), p!(gt, 200))) )
    ));

    assert!(forecaster.write_report_for.called_with_pattern(
        matcher!( p!(eq, 42), p!(eq, false) )
    ));

    assert!(forecaster.write_report_for.has_patterns(vec!(
        matcher!( p!(eq, 42), p!(eq, true) ),
        matcher!( p!(eq, 42), p!(eq, false) )
    )));

    forecaster.store_forecast_result(Ok(51));
    forecaster.store_forecast_result(Err("sad_face :(".to_owned()));
    assert!(forecaster.store_forecast_result.called_with_pattern(
        matcher!( p!(is_ok, p!(ge, 50)) )
    ));
    assert!(forecaster.store_forecast_result.called_with_pattern(
        matcher!( p!(is_err, p!(contains, "sad")) )
    ));
    assert!(!forecaster.store_forecast_result.called_with_pattern(
        matcher!( p!(is_err, p!(contains, "happy")) )
    ));
    assert!(forecaster.store_forecast_result.called_with_pattern(
        matcher!(
            p!(is_ok,
                p!(all_of, vec!(
                    p!(ge, 50),
                    p!(le, 60)))))
    ));
}

