use eframe::egui;

use crate::TimeSpent;
use crate::shared::tracker;

impl TimeSpent {
	pub fn draw_delete_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Delete").show(ctx, |ui| {
			ui.heading(
				format!("Are you sure that you want to delete {}?", self.win.delete_data.name)
			);
			
			ui.add_space(1.);
			
			ui.horizontal(|ui| {
				ui.set_min_width(180.);

				ui.label("Executable path: ");

				ui.add(
					egui::Label::new(
						egui::RichText::new(
								self.win.delete_data.path.clone()
						).monospace()
					).wrap()
				);

			});

			ui.add_space(1.);

			ui.colored_label(
				egui::Color32::LIGHT_RED, 
				"This action can not be undone",
			);
			
			ui.add_space(5.);

			ui.horizontal(|ui| {
				if ui.button(egui::RichText::new("Delete").size(16.)).clicked() {
					tracker::remove_entry(&self.win.delete_data.key);

					self.refresh();
					self.win.delete_window = false;
				}

				ui.add_space(5.);

				if ui.button(egui::RichText::new("Cancel").size(16.)).clicked() {
					self.win.delete_window = false;
				}
			});

		});
	}
}