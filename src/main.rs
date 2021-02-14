#![deny(clippy::all)]
// We use the console subsystem in debug builds, but use the Windows subsystem in release
// builds so we don't have to allocate a console and pop up a command line window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]

#[cfg_attr(target_os = "windows", path = "windows.rs")]
mod os;

mod chrome_local_state;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    os::main()
}
