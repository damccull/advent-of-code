use std::path::PathBuf;

use super::get_diagnostic_report;

pub fn get_diagnostics(filename: PathBuf) -> u32 {
    let report = get_diagnostic_report(filename);
    dbg!(
        report.get_gamma_rate_binary(),
        report.get_epsilon_rate_binary(),
        report.get_gamma_rate_decimal(),
        report.get_epsilon_rate_decimal(),
        true as u8,
        false as u8,
    );
    report.get_gamma_rate_decimal() as u32 * report.get_epsilon_rate_decimal() as u32
}
