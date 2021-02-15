#![deny(clippy::all)]

#[cfg_attr(target_os = "macos", path = "macos.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
mod os;

mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    os::main()
}
