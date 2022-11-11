/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SiliconApp {
    version: String,
}

impl Default for SiliconApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }
}

impl SiliconApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for SiliconApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Top bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Menu bar
            egui::menu::bar(ui, |ui| {
                // Light and dark mode switch
                egui::widgets::global_dark_light_mode_switch(ui);

                // The title
                ui.label(format!("Silicon v{}", self.version));

                // Seperate the title from the menu options
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
                    if ui.button("Reload").clicked() {
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
                        .button("Reset gui")
                        .on_hover_text("Forget scroll, positions, sizes etc")
                        .clicked()
                    {
                        *ui.ctx().memory() = Default::default();
                        ui.close_menu();
                    }
                });

                // Elements to the right of the menu bar
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    // Display a warning if we're in a debug build
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        // The electronics drawer, where components are picked from
        egui::TopBottomPanel::bottom("electronics").show(ctx, |ui| {
            ui.heading("Electronics");
            ui.label("Components will be visible here...");
        });

        // The main grid for drawing circuits
        egui::CentralPanel::default().show(ctx, |ui| {
            // Placeholder text
            ui.heading("Nothing yet..");
        });

        egui::Window::new("Preferences").show(ctx, |ui| {
            // Placeholder text
            ui.label("Windows can be moved by dragging them.");
            ui.label("They are automatically sized based on contents.");
            ui.label("You can turn on resizing and scrolling if you like.");
            ui.label("You would normally chose either panels OR windows.");

            // Credits
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Made with ");
                    ui.hyperlink_to("♡", "https://github.com/SpyHoodle/silicon");
                    ui.label(" for ");
                    ui.hyperlink_to("💍", "https://github.com/RetroEvelyne");
                    ui.label(" and ");
                    ui.hyperlink_to("⚛", "https://en.wikipedia.org/wiki/Physics");
                    ui.label(".");
                });
            });
        });

        egui::Window::new("Reference").show(ctx, |ui| {
            // Placeholder text
            ui.label("Windows can be moved by dragging them.");
            ui.label("They are automatically sized based on contents.");
            ui.label("You can turn on resizing and scrolling if you like.");
            ui.label("You would normally chose either panels OR windows.");
        });
    }
}
