use eframe::egui;

use crate::TimeSpent;

impl TimeSpent {
	pub fn draw_delete_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Delete").show(ctx, |ui| {
			let data = self.win.delete_data.clone();

			ui.heading(
				format!("Are you sure that you want to delete {}?", data["friendlyName"])
			);
			
			ui.add_space(1.);
			
			ui.horizontal(|ui| {
				ui.set_min_width(180.);

				ui.label("Executable path: ");

				ui.add(
					egui::Label::new(
						egui::RichText::new(

								data["exeLocation"].as_str().unwrap_or("null")
								
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
					let filename = format!("{}.json", 
						data["name"].as_str().unwrap());
					
					let fullpath = &self.processes_dir.join(filename);

					if let Err(e) = std::fs::remove_file(fullpath) {
						crate::log!("Couldn't delete {} ({})", data["name"], e);
					}

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