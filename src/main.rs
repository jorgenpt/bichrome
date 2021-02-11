#![deny(clippy::all)]
#![forbid(unsafe_code)]
// We use the console subsystem in debug builds, but use the Windows subsystem in release
// builds so we don't have to allocate a console and pop up a command line window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]

mod bichrome_config;
mod chrome_local_state;
#[cfg_attr(windows, path = "windows.rs")]
mod os;

use log::{debug, error, info, trace, warn};
use simplelog::*;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use structopt::StructOpt;

// This is the definition of our command line options
#[derive(Debug, StructOpt)]
#[structopt(
    name = "bichrome",
    about = "A program to pick Chrome profile based on the URL opened"
)]
struct Opt {
    /// Use verbose logging
    #[structopt(short, long)]
    verbose: bool,
    /// Use debug logging, even more verbose than --verbose
    #[structopt(long)]
    debug: bool,

    /// Do not launch Chrome, just log what would've been launched
    #[structopt(long)]
    dry_run: bool,
    /// Always generate a config, even if it exists or if we're using --dry-run
    #[structopt(long)]
    force_config_generation: bool,

    /// List of URLs to open
    urls: Vec<String>,
}

// Chrome Local State paths:
//  - macOS path: "/Users/jorgenpt/Library/Application Support/Google/Chrome/Local State";
//  - Windows path: r"C:\Users\jorgenpt\AppData\Local\Google\Chrome\User Data\Local State";

const CHROME_EXE_PATH: &str = r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe";

fn get_relative_path(filename: &str) -> Result<PathBuf, std::io::Error> {
    let mut path = std::env::current_exe()?;
    path.set_file_name(filename);
    Ok(path)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First parse our command line options, so we can use it to configure the logging.
    let opt = Opt::from_args();
    let log_level = if opt.debug {
        LevelFilter::Trace
    } else if opt.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let log_path = get_relative_path("bichrome.log")?;
    let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::new();
    // If we can write to bichrome.log, always use it.
    if let Ok(file) = File::create(log_path) {
        loggers.push(WriteLogger::new(log_level, Config::default(), file));
    }
    // We only use the terminal logger in the debug build, since we don't allocate a console window otherwise.
    if cfg!(debug_assertions) {
        if let Some(logger) = TermLogger::new(log_level, Config::default(), TerminalMode::Mixed) {
            loggers.push(logger)
        }
    };

    CombinedLogger::init(loggers)?;
    trace!("command line options: {:?}", opt);

    let config_path = get_relative_path("bichrome_config.json")?;
    if !config_path.exists() || opt.force_config_generation {
        // TODO: Error handling when this doesn't exist?
        let config_template_path = get_relative_path("bichrome_template.json")?;

        // TODO: Correctly detect this path
        let local_state_path =
            r"C:\Users\jorgenpt\AppData\Local\Google\Chrome\User Data\Local State";

        // TODO: Handle if this fails to parse?
        let profile_to_hosted_domain =
            chrome_local_state::read_profiles_from_file(local_state_path).unwrap();

        // TODO: Handle if this fails to generate?
        bichrome_config::generate_config(
            &config_template_path,
            &config_path,
            &profile_to_hosted_domain,
        )
        .unwrap();
    }

    // We try to read the config, and otherwise just use an empty one instead.
    debug!("attempting to load config from {}", config_path.display());
    let config = bichrome_config::read_config_from_file(&config_path);
    let config = match config {
        Ok(config) => {
            trace!("config: {:#?}", config);
            config
        }
        Err(e) => {
            error!("failed to parse config: {:?}", e);
            warn!("opening URLs without profile");
            bichrome_config::Configuration::empty()
        }
    };

    // TODO: Figure out what --reinstall / --hideicons / --showicons invocations are supposed to do.
    if opt.urls.is_empty() {
        if opt.dry_run {
            info!("(dry-run) direct launch -- would register URL handler")
        } else {
            info!("direct launch -- registering URL handler");
            let extra_args = if opt.debug {
                Some("--debug")
            } else if opt.verbose {
                Some("--verbose")
            } else {
                None
            };

            if let Err(e) = os::register_urlhandler(extra_args) {
                error!("failed to register URL handler: {:?}", e);
            }
        }
    } else {
        for url in opt.urls {
            let mut args = Vec::new();
            if let Some(profile_name) = config.choose_profile(&url) {
                args.push(format!("--profile-directory={}", profile_name));
            }
            args.push(url);

            if opt.dry_run {
                info!(
                    "(dry-run) \"{}\" \"{}\"",
                    CHROME_EXE_PATH,
                    args.join("\" \"")
                );
            } else {
                debug!(
                    "launching \"{}\" \"{}\"",
                    CHROME_EXE_PATH,
                    args.join("\" \"")
                );
                Command::new(CHROME_EXE_PATH)
                    .stdout(Stdio::null())
                    .stdin(Stdio::null())
                    .stderr(Stdio::null())
                    .args(args)
                    .spawn()?;
            }
        }
    }

    Ok(())
}
