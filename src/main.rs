mod annotations;
mod cli;
mod globals;
mod logger;
mod test_seq_match;
mod vcf_scanner;

use std::{error, process};

use hhmmss::Hhmmss;
use log::info;

use crate::annotations::get_default_annotations;
use crate::cli::{DerivedSettings, Settings, validate_and_fix_settings};
use crate::globals::{PROGRAM_NAME, PROGRAM_VERSION};
use crate::logger::setup_logger;
use crate::vcf_scanner::scan_vcf_file;

fn run(
    settings: &Settings,
    derived_settings: &DerivedSettings,
) -> Result<(), Box<dyn error::Error>> {
    info!("Starting {PROGRAM_NAME} {PROGRAM_VERSION}");
    info!(
        "cmdline: {}",
        std::env::args().collect::<Vec<_>>().join(" ")
    );
    info!("Running on {} threads", derived_settings.thread_count);

    let start = std::time::Instant::now();

    let annotations = get_default_annotations();

    scan_vcf_file(
        &settings.in_vcf,
        derived_settings.thread_count,
        &annotations,
    );

    info!(
        "{PROGRAM_NAME} completed. Total Runtime: {}",
        start.elapsed().hhmmssxxx()
    );
    Ok(())
}

fn main() {
    let settings = cli::parse_settings();
    let derived_settings = validate_and_fix_settings(&settings);

    setup_logger().unwrap();

    if let Err(err) = run(&settings, &derived_settings) {
        eprintln!("{}", err);
        process::exit(2);
    }
}
