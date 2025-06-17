use camino::{Utf8Path, Utf8PathBuf};
use chrono::Datelike;
use clap::{Parser, ValueEnum};
use log::error;
use simple_error::{SimpleResult, bail};

use crate::globals::PROGRAM_VERSION;

/// Annotation modes
///
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, strum::Display, ValueEnum)]
#[allow(non_camel_case_types)]
pub enum AnnotationMode {
    /// Provide pbsv-compatible mobile element annotations
    #[default]
    pbsv,
}

#[derive(Parser)]
#[command(
    author,
    version = PROGRAM_VERSION,
    about,
    after_help = format!("Copyright (C) 2004-{}     Pacific Biosciences of California, Inc.
This program comes with ABSOLUTELY NO WARRANTY; it is intended for
Research Use Only and not for use in diagnostic procedures.", chrono::Utc::now().year()),
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage} >| annotated_vcf_output

{all-args}{after-help}"
)]
#[clap(rename_all = "kebab_case")]
pub struct Settings {
    /// Input SV VCF/BCF file, does not need to be sorted or indexed. Use '-' for stdin.
    #[arg(long = "vcf", value_name = "FILE")]
    pub in_vcf: Utf8PathBuf,

    /// Annotation mode
    #[arg(long, value_enum, value_name = "MODE", default_value_t = Default::default())]
    pub mode: AnnotationMode,

    /// Number of threads to use. Defaults to all logical cpus detected.
    #[arg(long = "threads", value_name = "THREAD_COUNT")]
    thread_count_option: Option<usize>,
}

/// Values immediately computed from the user settings, but not part of direct user inputs
///
pub struct DerivedSettings {
    /// Global thread count for pb-CpG-tools to use
    pub thread_count: usize,
}

/// Validate settings and use these to produce derived settings
///
fn validate_and_fix_settings_impl(settings: &Settings) -> SimpleResult<DerivedSettings> {
    fn check_required_filename(filename: &Utf8Path, label: &str) -> SimpleResult<()> {
        if filename.as_str().is_empty() {
            bail!("Must specify {label} file");
        }
        if !filename.exists() {
            bail!("Can't find specified {label} file: '{filename}'");
        }
        Ok(())
    }

    if settings.in_vcf.as_str() != "-" {
        check_required_filename(&settings.in_vcf, "input vcf")?;
    }

    let thread_count = match settings.thread_count_option {
        Some(count) => {
            if count == 0 {
                bail!("--threads argument must be greater than 0");
            }
            count
        }
        None => num_cpus::get(),
    };

    Ok(DerivedSettings { thread_count })
}

pub fn validate_and_fix_settings(settings: &Settings) -> DerivedSettings {
    match validate_and_fix_settings_impl(settings) {
        Ok(x) => x,
        Err(msg) => {
            error!("Invalid command-line setting: {}", msg);
            std::process::exit(exitcode::USAGE);
        }
    }
}

pub fn parse_settings() -> Settings {
    Settings::parse()
}
