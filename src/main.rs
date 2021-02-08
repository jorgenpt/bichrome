#![deny(clippy::all)]
#![forbid(unsafe_code)]
// We use the console subsystem in debug builds, but use the Windows subsystem in release
// builds so we don't have to allocate a console and pop up a command line window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]

mod bichrome_config;
mod chrome_local_state;

use log::{error, info, trace, warn};
use simplelog::*;
use std::fs::File;
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

    /// List of URLs to open
    urls: Vec<String>,
}

// Chrome Local State paths:
//  - macOS path: "/Users/jorgenpt/Library/Application Support/Google/Chrome/Local State";
//  - Windows path: r"C:\Users\jorgenpt\AppData\Local\Google\Chrome\User Data\Local State";

const CHROME_EXE_PATH: &str = r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe";

fn main() {
    // First parse our command line options, so we can use it to configure the logging.
    let opt = Opt::from_args();
    let log_level = if opt.debug {
        LevelFilter::Trace
    } else if opt.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::new();
    // If we can write to bichrome.log, always use it.
    if let Ok(file) = File::create("bichrome.log") {
        loggers.push(WriteLogger::new(log_level, Config::default(), file));
    }
    // We only use the terminal logger in the debug build, since we don't allocate a console window otherwise.
    if cfg!(debug_assertions) {
        if let Some(logger) = TermLogger::new(log_level, Config::default(), TerminalMode::Mixed) {
            loggers.push(logger)
        }
    };

    CombinedLogger::init(loggers).unwrap();
    trace!("command line options: {:?}", opt);

    // TODO: Set up the appropriate registry entries to appear on a new install.
    // TODO: Use profiles + a distribution config to generate bichrome_config if it doesn't exist.
    // TODO: Read profile from %localappdata%.

    // We try to read the config, and otherwise just use an empty one instead.
    let config = bichrome_config::read_config_from_file("bichrome_config.json");
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

    for url in opt.urls {
        let mut args = Vec::new();
        if let Some(profile_name) = config.choose_profile(&url) {
            args.push(format!("--profile-directory={}", profile_name));
        }
        args.push(url);

        if opt.dry_run {
            info!(
                "launching \"{}\" \"{}\"",
                CHROME_EXE_PATH,
                args.join("\" \"")
            );
        } else {
            Command::new(CHROME_EXE_PATH)
                .stdout(Stdio::null())
                .stdin(Stdio::null())
                .stderr(Stdio::null())
                .args(args)
                .spawn()
                .unwrap();
        }
    }
}
