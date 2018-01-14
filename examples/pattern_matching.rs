#[macro_use]
extern crate double;

use double::matcher::*;

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

fn main() {
    let forecaster = MockForecaster::default();
    forecaster.profit_at(42);
    forecaster.profit_at(84);
    forecaster.write_report_for(42, true);
    forecaster.write_report_for(84, true);
    forecaster.write_report_for(42, false);

    let profit_at_matches = forecaster.profit_at.called_with_pattern(
        matcher!(&arg!(equal, 42))
    );
    println!("profit_at {:?}", profit_at_matches);

    let write_report_for_matches = forecaster.write_report_for.called_with_pattern(
        matcher!(&arg!(equal, 42), &arg!(equal, false) )
    );
    println!("write_report_for {:?}", write_report_for_matches);

    let write_report_for_matches_all = forecaster.write_report_for.has_patterns(vec!(
        matcher!(&arg!(equal, 42), &arg!(equal, true) ),
        matcher!(&arg!(equal, 42), &arg!(equal, false) )
    ));
    println!("write_report_for matches all: {:?}", write_report_for_matches_all);

    // TODO: add tests for all pattern checking variants
}
