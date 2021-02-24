#![deny(clippy::all)]

mod chrome_local_state;
mod config;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as os;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as os;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(os::main()?)
}
