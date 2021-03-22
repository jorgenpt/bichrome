use crate::config::{Browser, ChromeProfile, Configuration};
use crate::os::get_config_path;
use eframe::{
    egui::{self, ScrollArea},
    epi,
};

#[derive(Default)]
pub struct RuntimeState {
    selected_profile: Option<String>,
}

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
        if let Some(config_path) = get_config_path() {
            *self = Self {
                configuration: Configuration::read_from_file(config_path).unwrap_or_default(),
                runtime_state: RuntimeState::default(),
            }
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
            runtime_state,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.heading("Browsers");
                ui.columns(2, |columns| {
                    ScrollArea::auto_sized()
                        .id_source("browsers")
                        .show(&mut columns[0], |ui| {
                            for (name, browser) in &configuration.profiles {
                                let is_default = match &configuration.default_profile {
                                    Some(profile_name) => name == profile_name,
                                    None => false,
                                };
                                let default_suffix = if is_default { " (default)" } else { "" };

                                let is_selected =
                                    if let Some(profile) = &runtime_state.selected_profile {
                                        profile == name
                                    } else {
                                        false
                                    };

                                let mut label = match browser {
                                    Browser::Chrome(_) => "",
                                    Browser::Firefox => "",
                                    Browser::OsDefault => {
                                        if cfg!(target_os = "windows") {
                                            ""
                                        } else {
                                            ""
                                        }
                                    }
                                }
                                .to_owned();
                                label.push(' ');
                                label.push_str(name);
                                label.push_str(default_suffix);

                                if ui.selectable_label(is_selected, label).clicked() {
                                    runtime_state.selected_profile = if is_selected {
                                        None
                                    } else {
                                        Some(name.to_owned())
                                    };
                                }
                            }
                        });

                    let selected = if let Some(profile) = &runtime_state.selected_profile {
                        configuration
                            .profiles
                            .get_mut(profile)
                            .map(|b| (profile, b))
                    } else {
                        None
                    };

                    if let Some((name, browser)) = selected {
                        ScrollArea::auto_sized().id_source("browser_detail").show(
                            &mut columns[1],
                            |ui| {
                                ui.label("Browser: ");
                                egui::combo_box(
                                    ui,
                                    ui.make_persistent_id("browser_combo"),
                                    format!("{}", browser),
                                    |ui| {
                                        let is_chrome_byhosteddomain = matches!(
                                            browser,
                                            Browser::Chrome(ChromeProfile::ByHostedDomain {
                                                hosted_domain: _
                                            })
                                        );
                                        if ui
                                            .selectable_label(
                                                is_chrome_byhosteddomain,
                                                format!(
                                                    "{}",
                                                    Browser::Chrome(
                                                        ChromeProfile::ByHostedDomain {
                                                            hosted_domain: String::default(),
                                                        }
                                                    )
                                                ),
                                            )
                                            .clicked()
                                        {
                                            if !is_chrome_byhosteddomain {
                                                *browser = Browser::Chrome(
                                                    ChromeProfile::ByHostedDomain {
                                                        hosted_domain: String::default(),
                                                    },
                                                );
                                            }
                                        }

                                        let is_chrome_byname = matches!(
                                            browser,
                                            Browser::Chrome(ChromeProfile::ByName { name: _ })
                                        );
                                        if ui
                                            .selectable_label(
                                                matches!(
                                                    browser,
                                                    Browser::Chrome(ChromeProfile::ByName {
                                                        name: _
                                                    })
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
                                            if !is_chrome_byname {
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
                                    },
                                );
                            },
                        );
                    }
                });
            });
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

                    let button_id = ui.make_persistent_id(name).with("browser");

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
