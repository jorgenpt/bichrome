#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(clippy::all)]

use anyhow::Result;
use bichrome::gui;

pub fn main() -> Result<()> {
    gui::run();
    Ok(())
}
