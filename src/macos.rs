use crate::config::Configuration;
use fruitbasket::FruitApp;
use fruitbasket::FruitCallbackKey;
use fruitbasket::RunPeriod;
use log::{debug, error, trace, warn};
use simplelog::*;
use std::{
    error::Error,
    path::PathBuf,
    process::{Command, Stdio},
};

fn get_chrome_binary_path() -> PathBuf {
    // TODO Could be:
    // `mdfind 'kMDItemCFBundleIdentifier = "com.google.Chrome"'`
    PathBuf::from("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
}

fn get_application_support_path() -> Option<PathBuf> {
    let home_dir = std::env::var_os("HOME")
        .and_then(|h| if h.is_empty() { None } else { Some(h) })
        .map(PathBuf::from);
    home_dir.map(|path| path.join("Library/Application Support"))
}

#[allow(dead_code)]
fn get_chrome_local_state_path() -> Option<PathBuf> {
    get_application_support_path().map(|path| path.join("Google/Chrome/Local State"))
}

fn get_config_path() -> Option<PathBuf> {
    get_application_support_path().map(|path| path.join("com.bitspatter.bichrome/config.json"))
}

fn init() -> Configuration {
    let config_path = get_config_path();
    // We try to read the config, and otherwise just use an empty one instead.
    match config_path {
        Some(config_path) => {
            debug!("attempting to load config from {}", config_path.display());
            let config = Configuration::read_from_file(&config_path);
            match config {
                Ok(config) => {
                    trace!("config: {:#?}", config);
                    config
                }
                Err(e) => {
                    error!("failed to parse config: {:?}", e);
                    warn!("opening URLs without profile");
                    Configuration::empty()
                }
            }
        }
        None => {
            error!("failed to determine config path");
            warn!("opening URLs without profile");
            Configuration::empty()
        }
    }
}

fn handle_url(url: &str) -> Result<(), Box<dyn Error>> {
    let config = init();

    let mut args = Vec::new();
    if let Some(profile_name) = config.choose_profile(&url) {
        args.push(format!("--profile-directory={}", profile_name));
    }
    args.push(url.to_string());

    let chrome_path = get_chrome_binary_path();
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

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap()])?;

    let mut app = FruitApp::new();

    let stopper = app.stopper();
    app.register_callback(
        FruitCallbackKey::Method("applicationWillFinishLaunching:"),
        Box::new(move |_event| {
            stopper.stop();
        }),
    );

    // Register a callback to get receive custom URL schemes from any Mac program
    app.register_apple_event(fruitbasket::kInternetEventClass, fruitbasket::kAEGetURL);
    let stopper = app.stopper();
    app.register_callback(
        FruitCallbackKey::Method("handleEvent:withReplyEvent:"),
        Box::new(move |event| {
            let url: String = fruitbasket::parse_url_event(event);
            if let Err(error) = handle_url(&url) {
                panic!("error handling url: {}", error);
            }
            stopper.stop();
        }),
    );

    // Run 'forever', until the URL callback fires
    let _ = app.run(RunPeriod::Forever);

    fruitbasket::FruitApp::terminate(0);

    // This will never execute.
    Ok(())
}
