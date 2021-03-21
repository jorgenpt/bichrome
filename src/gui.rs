use crate::config::Configuration;
use eframe::{egui, epi};

// App state is automatically loaded when the app runs
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ConfigGui {
    configuration: Configuration,
}

impl Default for ConfigGui {
    fn default() -> Self {
        Self {
            configuration: Configuration::empty(),
        }
    }
}

impl epi::App for ConfigGui {
    fn name(&self) -> &str {
        "bichrome configuration"
    }

    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
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
