use egui::Ui;

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Grid {

}

impl Default for Grid {
    fn default() -> Self {
        Self {

        }
    }
}

impl Grid {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            // Heading for the component picker
            ui.heading("Components");

            // Credits
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("made with ");
                    ui.hyperlink_to("♡", "https://github.com/SpyHoodle/silicon");
                    ui.label(" by ");
                    ui.hyperlink_to("Maddie", "https://github.com/SpyHoodle");
                    ui.label(" for ");
                    ui.hyperlink_to("my one and only :3", "https://github.com/RetroEvelyne");
                    ui.label(".");
                });
            });
        });
    }
}
