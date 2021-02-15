use crate::config::{Browser, Configuration};
use fruitbasket::FruitApp;
use fruitbasket::FruitCallbackKey;
use fruitbasket::RunPeriod;
use log::{debug, error, trace, warn};
use simplelog::*;
use std::{
    error::Error,
    fs::File,
    path::PathBuf,
    process::{Command, Stdio},
};

fn get_chrome_binary_path() -> PathBuf {
    // TODO Could be -- hopefully this would find it in Applications too?
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
pub fn get_chrome_local_state_path() -> Option<PathBuf> {
    get_application_support_path().map(|path| path.join("Google/Chrome/Local State"))
}

fn get_log_path() -> Option<PathBuf> {
    get_application_support_path().map(|path| path.join("com.bitspatter.bichrome/bichrome.log"))
}

fn get_config_path() -> Option<PathBuf> {
    get_application_support_path()
        .map(|path| path.join("com.bitspatter.bichrome/bichrome_config.json"))
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

    let browser = config.choose_browser(&url)?;
    let (exe, args) = match browser {
        Browser::Chrome(profile) => {
            if let Some(argument) = profile.get_argument()? {
                let args = vec![argument, url.to_string()];
                (get_chrome_binary_path().to_str().unwrap().to_string(), args)
            } else {
                // We use `open -b com.google.Chrome <url>` when you don't specify a profile as it
                // responds faster, and it is the more "natural" way to open an URL in Chrome.
                let args = ["-b", "com.google.Chrome", url]
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                ("open".to_string(), args)
            }
        }
        Browser::Firefox => {
            // TODO If we support Firefox profiles, use something like the Chrome path with firefox -P <profile>
            let args = ["-b", "org.mozilla.firefox", url]
                .iter()
                .map(|s| s.to_string())
                .collect();
            ("open".to_string(), args)
        }
        Browser::Safari => {
            let args = ["-b", "com.apple.Safari", url]
                .iter()
                .map(|s| s.to_string())
                .collect();
            ("open".to_string(), args)
        }
    };

    Command::new(&exe)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .args(args)
        .spawn()?;

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let log_level = LevelFilter::Debug;
    let log_path = get_log_path().unwrap();
    let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::new();
    // If we can write to bichrome.log, always use it.
    if let Ok(file) = File::create(log_path) {
        loggers.push(WriteLogger::new(log_level, Config::default(), file));
    }
    if let Some(logger) = TermLogger::new(log_level, Config::default(), TerminalMode::Mixed) {
        loggers.push(logger)
    }
    CombinedLogger::init(loggers)?;

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
