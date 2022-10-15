/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
// If we add new fields, give them default values when deserializing old state
#[serde(default)]
pub struct SiliconApp {
    // Opt-out of serialization
    #[serde(skip)]
    version: String,
}

impl Default for SiliconApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            version: "0.0.1".to_owned(),
        }
    }
}

impl SiliconApp {
    // Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any)
        // Note that the `persistence` feature must be enabled
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for SiliconApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Top bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Menu bar
            egui::menu::bar(ui, |ui| {
                // Theme switcher
                egui::widgets::global_dark_light_mode_switch(ui);

                // Separator
                ui.separator();

                // Title
                ui.label(format!("Silicon v{}", self.version));

                // Separator
                ui.separator();

                // File menu
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        println!("TODO") // TODO
                    }
                    if ui.button("Open...").clicked() {
                        println!("TODO") // TODO
                    }
                    if ui.button("Save...").clicked() {
                        println!("TODO") // TODO
                    }
                    if ui.button("Save as...").clicked() {
                        println!("TODO") // TODO
                    }
                    if ui.button("Reload from disk").clicked() {
                        println!("TODO") // TODO
                    }

                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });

                // Edit menu
                ui.menu_button("Edit", |ui| {
                    if ui.button("Preferences").clicked() {
                        println!("TODO") // TODO
                    }
                });

                // Debug menu
                ui.menu_button("Debug", |ui| {
                    if ui.button("Organize windows").clicked() {
                        ui.ctx().memory().reset_areas();
                        ui.close_menu();
                    }
                    if ui
                        .button("Reset egui")
                        .on_hover_text("Forget scroll, positions, sizes etc")
                        .clicked()
                    {
                        *ui.ctx().memory() = Default::default();
                        ui.close_menu();
                    }
                });

                // Add elements to the right of the menu bar
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    // Warn if we're in a debug build
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        // Default panel when no file is loaded
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Welcome to Silicon.");
                ui.add(egui::github_link_file!(
                    "https://github.com/SpyHoodle/silicon",
                    "The accurate, open-source, rust circuit simulator."
                ));
            });
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
