use crate::config::{Browser, ChromeProfile, Configuration};
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
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let ConfigGui {
            configuration,
            runtime_state: _runtime_state,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Browsers");
            egui::Grid::new("browsers").show(ui, |ui| {
                ui.heading("Default");
                ui.heading("Name");
                ui.heading("Browser");
                ui.heading("Profile");
                ui.end_row();

                for (name, browser) in &mut configuration.profiles {
                    let is_default = match &configuration.default_profile {
                        Some(profile_name) => name == profile_name,
                        None => false,
                    };
                    if ui.radio(is_default, "").clicked() {
                        configuration.default_profile = Some(name.to_owned());
                    }

                    ui.label(name);

                    //match browser {

                    let button_id = ui.make_persistent_id(name).with("browser");

                    ui.horizontal(|ui| {
                        egui::combo_box(ui, button_id, format!("{}", browser), |ui| {
                            let is_chrome_hosted = matches!(
                                browser,
                                Browser::Chrome(ChromeProfile::ByHostedDomain { hosted_domain: _ })
                            );
                            if ui
                                .selectable_label(
                                    is_chrome_hosted,
                                    format!(
                                        "{}",
                                        Browser::Chrome(ChromeProfile::ByHostedDomain {
                                            hosted_domain: String::default(),
                                        })
                                    ),
                                )
                                .clicked()
                            {
                                if !is_chrome_hosted {
                                    *browser = Browser::Chrome(ChromeProfile::ByHostedDomain {
                                        hosted_domain: String::default(),
                                    });
                                }
                            }

                            let is_chrome_profile = matches!(
                                browser,
                                Browser::Chrome(ChromeProfile::ByName { name: _ })
                            );
                            if ui
                                .selectable_label(
                                    matches!(
                                        browser,
                                        Browser::Chrome(ChromeProfile::ByName { name: _ })
                                    ),
                                    format!(
                                        "{}",
                                        Browser::Chrome(ChromeProfile::ByName {
                                            name: String::default()
                                        })
                                    ),
                                )
                                .clicked()
                            {
                                if !is_chrome_profile {
                                    *browser = Browser::Chrome(ChromeProfile::ByName {
                                        name: String::default(),
                                    });
                                }
                            }
                            ui.selectable_value(
                                browser,
                                Browser::Chrome(ChromeProfile::None {}),
                                format!("{}", Browser::Chrome(ChromeProfile::None {})),
                            );
                            ui.selectable_value(
                                browser,
                                Browser::Firefox,
                                format!("{}", Browser::Firefox),
                            );
                            ui.selectable_value(
                                browser,
                                Browser::OsDefault,
                                format!("{}", Browser::OsDefault),
                            );
                        });
                    });

                    if let Browser::Chrome(profile) = browser {
                        match profile {
                            ChromeProfile::ByHostedDomain { hosted_domain } => {
                                ui.text_edit_singleline(hosted_domain)
                            }
                            ChromeProfile::ByName { name } => ui.text_edit_singleline(name),
                            ChromeProfile::None {} => ui.label(""),
                        };
                    } else {
                        ui.label("");
                    }

                    ui.end_row();
                }
            });

            ui.separator();
            ui.heading("Patterns");
        });
    }
}

pub fn run() {
    let app = ConfigGui::default();
    eframe::run_native(Box::new(app));
}
