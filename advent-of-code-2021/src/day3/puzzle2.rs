use std::path::PathBuf;

use crate::day3::get_diagnostic_report;

pub fn get_life_support_status(filename: PathBuf) -> u32 {
    let report = get_diagnostic_report(filename);
    report.get_oxygen_generator_rating_decimal() * report.get_co2_scrubber_rating_decimal()
}
