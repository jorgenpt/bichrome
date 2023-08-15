#![deny(clippy::all)]
// We use the console subsystem in debug builds, but use the Windows subsystem in release
// builds so we don't have to allocate a console and pop up a command line window.
// This needs to live in main.rs rather than windows.rs because it needs to be a crate-level
// attribute, and it doesn't affect the mac build at all, so it's innocuous to leave for
// both target_os.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]

mod chrome_local_state;
mod config;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use crate::macos as os;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use crate::windows as os;

use anyhow::Result;
use log::error;

fn main() -> Result<()> {
    let result = os::main();
    if let Err(error) = &result {
        error!("Encountered error: {error:?}");
    }
    result
}
