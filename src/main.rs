#![deny(clippy::all)]
// We use the console subsystem in debug builds, but use the Windows subsystem in release
// builds so we don't have to allocate a console and pop up a command line window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]

mod chrome_local_state;
mod config;
#[cfg_attr(windows, path = "windows.rs")]
mod os;

use config::{generate_config, Configuration};
use log::{debug, error, info, trace, warn};
use simplelog::*;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{error::Error, fmt};
use structopt::StructOpt;

// This is the definition of our command line options
#[derive(Debug, StructOpt)]
#[structopt(
    name = "bichrome",
    about = "A program to pick Chrome profile based on the URL opened"
)]
struct CommandOptions {
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

    /// Choose the mode of operation
    #[structopt(subcommand)]
    mode: Option<ExecutionMode>,

    /// List of URLs to open
    urls: Vec<String>,
}

#[derive(Debug, Clone, Copy, StructOpt)]
enum ExecutionMode {
    /// Open the given URLs in the correct browser
    Open,
    /// Register bichrome as a valid browser
    Register,
    /// Remove previous registration of bichrome, if any
    Unregister,
    /// Show application icons (changes a registry key and nothing else, as we don't have icons)
    ShowIcons,
    /// Hide application icons (changes a registry key and nothing else, as we don't have icons)
    HideIcons,
}

fn get_exe_relative_path(filename: &str) -> io::Result<PathBuf> {
    let mut path = std::env::current_exe()?;
    path.set_file_name(filename);
    Ok(path)
}

#[derive(Debug, Clone)]
struct ChromeNotFoundError;

impl fmt::Display for ChromeNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to retrieve path to chrome.exe")
    }
}

impl Error for ChromeNotFoundError {}

fn init() -> Result<CommandOptions, Box<dyn Error>> {
    // First parse our command line options, so we can use it to configure the logging.
    let options = CommandOptions::from_args();
    let log_level = if options.debug {
        LevelFilter::Trace
    } else if options.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let log_path = get_exe_relative_path("bichrome.log")?;
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
    trace!("command line options: {:?}", options);

    Ok(options)
}

fn read_config(options: &CommandOptions) -> Result<Configuration, Box<dyn Error>> {
    let config_path = get_exe_relative_path("bichrome_config.json")?;
    if !config_path.exists() || options.force_config_generation {
        info!("attempting to generate config at {}", config_path.display());
        let config_template_path = get_exe_relative_path("bichrome_template.json")?;
        if !config_template_path.exists() {
            warn!(
                "could not find template configuration at {}, will not generate config",
                config_template_path.display()
            );
        } else if let Some(local_state_path) = os::get_chrome_local_state_path() {
            let chrome_profiles_data =
                chrome_local_state::read_profiles_from_file(local_state_path)?;
            trace!("chrome profiles data: {:?}", chrome_profiles_data);

            if !options.dry_run || options.force_config_generation {
                generate_config(&config_template_path, &config_path, &chrome_profiles_data)?;
            }
        } else {
            error!("unable to determine google chrome local state path, will not attempt to generate config from template");
        }
    }

    // We try to read the config, and otherwise just use an empty one instead.
    debug!("attempting to load config from {}", config_path.display());
    let config = Configuration::read_from_file(&config_path);
    Ok(match config {
        Ok(config) => {
            trace!("config: {:#?}", config);
            config
        }
        Err(e) => {
            error!("failed to parse config: {:?}", e);
            warn!("opening URLs without profile");
            Configuration::empty()
        }
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = init()?;

    let mode = options.mode.unwrap_or(if options.urls.is_empty() {
        ExecutionMode::Register
    } else {
        ExecutionMode::Open
    });

    if !matches!(mode, ExecutionMode::Open) && !options.urls.is_empty() {
        return Err(Box::new(structopt::clap::Error::with_description(
            &format!("specified a list of urls with mode {:?}", mode),
            structopt::clap::ErrorKind::WrongNumberOfValues,
        )));
    }

    match mode {
        ExecutionMode::Register => {
            if options.dry_run {
                info!("(dry-run) would register URL handler")
            } else {
                info!("registering URL handler");
                let extra_args = if options.debug {
                    Some("--debug")
                } else if options.verbose {
                    Some("--verbose")
                } else {
                    None
                };

                if let Err(e) = os::register_urlhandler(extra_args) {
                    error!("failed to register URL handler: {:?}", e);
                }
            }
        }
        ExecutionMode::Unregister => {
            if options.dry_run {
                info!("(dry-run) would unregister URL handler")
            } else {
                info!("unregistering URL handler");
                os::unregister_urlhandler();
            }
        }
        ExecutionMode::ShowIcons => {
            if options.dry_run {
                info!("(dry-run) would mark icons as visible")
            } else {
                info!("marking icons as visible");
                if let Err(e) = os::show_icons() {
                    error!("failed to show icons: {:?}", e);
                }
            }
        }
        ExecutionMode::HideIcons => {
            if options.dry_run {
                info!("(dry-run) would mark icons as hidden")
            } else {
                info!("marking icons as hidden");
                if let Err(e) = os::hide_icons() {
                    error!("failed to hide icons: {:?}", e);
                }
            }
        }
        ExecutionMode::Open => {
            let config = read_config(&options)?;
            let chrome_path = os::get_chrome_exe_path().ok_or(ChromeNotFoundError)?;
            for url in options.urls {
                let mut args = Vec::new();
                if let Some(profile_name) = config.choose_profile(&url) {
                    args.push(format!("--profile-directory={}", profile_name));
                }
                args.push(url);

                if options.dry_run {
                    info!(
                        "(dry-run) \"{}\" \"{}\"",
                        chrome_path.display(),
                        args.join("\" \"")
                    );
                } else {
                    debug!(
                        "launching \"{}\" \"{}\"",
                        chrome_path.display(),
                        args.join("\" \"")
                    );
                    Command::new(&chrome_path)
                        .stdout(Stdio::null())
                        .stdin(Stdio::null())
                        .stderr(Stdio::null())
                        .args(args)
                        .spawn()?;
                }
            }
        }
    }

    Ok(())
}
