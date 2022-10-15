// #[cfg(target_arch = "wasm32")]
// use core::any::Any;

// Grid app
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct GridApp {
    grid: crate::tabs::Grid,
}

impl eframe::App for GridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.grid.ui(ui, ctx);
            });
    }
}

// Store the state of the program
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct State {
    grid: GridApp,
    selected_tab: String,
}

pub struct SiliconApp {
    state: State,    // Program state
    version: String, // Version number
}

impl SiliconApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        #[allow(unused_mut)]
        let mut silicon_self = Self {
            state: State::default(),
            version: "0.0.1".to_owned(),
        };

        #[cfg(feature = "persistence")]
        if let Some(storage) = _cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                silicon_self.state = state;
            }
        }

        silicon_self
    }

    fn iter_mut_tabs(&mut self) -> impl Iterator<Item = (&str, &str, &mut dyn eframe::App)> {
        let vec = vec![(
            "⚒ Grid",
            "grid",
            &mut self.state.grid as &mut dyn eframe::App,
        )];

        vec.into_iter()
    }
}

impl eframe::App for SiliconApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        if let Some(anchor) = frame.info().web_info.location.hash.strip_prefix('#') {
            self.state.selected_tab = anchor.to_owned();
        }

        if self.state.selected_tab.is_empty() {
            let selected_tab = self.iter_mut_tabs().next().unwrap().0.to_owned();
            self.state.selected_tab = selected_tab;
        }

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
                        .button("Reset gui")
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

            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.tab_bar_contents(ui, frame);
            });
        });

        // Default panel when no file is loaded
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Welcome to Silicon.");
                ui.hyperlink_to(
                    "The accurate, open-source, rust circuit simulator.",
                    "https://github.com/SpyHoodle/silicon/",
                );
            });
        });

        self.show_selected_tab(ctx, frame);
    }

    // Unsure what this is supposed to do, commented out for now.
    // #[cfg(target_arch = "wasm32")]
    // fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
    //     Some(&mut *self)
    // }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }
}

impl SiliconApp {
    fn show_selected_tab(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut found_anchor = false;
        let selected_anchor = self.state.selected_tab.clone();
        for (_name, anchor, tab) in self.iter_mut_tabs() {
            if anchor == selected_anchor || ctx.memory().everything_is_visible() {
                tab.update(ctx, frame);
                found_anchor = true;
            }
        }
        if !found_anchor {
            self.state.selected_tab = "default".into();
        }
    }

    fn tab_bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let mut selected_anchor = self.state.selected_tab.clone();
        for (name, anchor, _tab) in self.iter_mut_tabs() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor.to_owned();
                if frame.is_web() {
                    ui.output().open_url(format!("#{}", anchor));
                }
            }
        }
        self.state.selected_tab = selected_anchor;
    }
}