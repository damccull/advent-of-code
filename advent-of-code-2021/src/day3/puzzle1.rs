use std::path::PathBuf;

use super::get_diagnostic_report;

pub fn get_diagnostics(filename: PathBuf) -> u32 {
    let report = get_diagnostic_report(filename);
    report.get_gamma_rate_decimal() as u32 * report.get_epsilon_rate_decimal() as u32
}
