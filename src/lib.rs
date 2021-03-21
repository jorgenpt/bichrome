pub mod chrome_local_state;
pub mod config;

// Import os-specific code for macOS, if appropriate
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos as os;

// Import os-specific code for Windows, if appropriate
#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows as os;
