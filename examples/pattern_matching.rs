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

    let profit_at_matches = forecaster.profit_at.compute_matches(
        matcher!(&bind!(equal, 42))
    );
    println!("profit_at {:?}", profit_at_matches);

    let write_report_for_matches = forecaster.write_report_for.compute_matches(
        matcher!(&bind!(equal, 42), &bind!(equal, false) )
    );
    println!("write_report_for {:?}", write_report_for_matches);
}
