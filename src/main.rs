#![deny(clippy::all)]

#[cfg_attr(target_os = "windows", path = "windows.rs")]
mod os;

mod chrome_local_state;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    os::main()
}
