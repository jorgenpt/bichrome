use crate::config::Configuration;
use crate::os::get_config_path;
use eframe::{egui, epi};

#[derive(Default)]
pub struct RuntimeState {}

#[derive(Default)]
pub struct ConfigGui {
    configuration: Configuration,
    runtime_state: RuntimeState,
}

impl epi::App for ConfigGui {
    fn name(&self) -> &str {
        "bichrome configuration"
    }

    fn load(&mut self, _: &dyn epi::Storage) {
        *self = Self {
            configuration: Configuration::read_from_file(get_config_path().unwrap_or_default())
                .unwrap_or_default(),
            runtime_state: RuntimeState::default(),
        }
    }

    fn save(&mut self, _: &mut dyn epi::Storage) {
        // TODO: Write out the modified configuration
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, _ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {}
}

// ----------------------------------------------------------------------------

pub fn run() {
    let app = ConfigGui::default();
    eframe::run_native(Box::new(app));
}
