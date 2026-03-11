use eframe::egui;
use serde_json::json;

use crate::TimeSpent;

impl TimeSpent {
	pub fn draw_rename_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Rename").show(ctx, |ui| {
			let mut rename_data = self.win.rename_data.clone();

			ui.heading(
				format!("What should {} be renamed to?", 
						rename_data["friendlyName"])
			);

			ui.add_space(3.);		

			ui.add(
				egui::widgets::TextEdit::singleline(&mut self.win.rename_to)
				.hint_text("New Name")
				.desired_width(120.)
			);

			if self.win.rename_to.is_empty() {
				self.win.rename_error = "Please Enter a New Name".to_string();
			
			} else if self.win.rename_to.len() > 25 {
				self.win.rename_error = 
					"Please Enter a Name Shorter than 25 Letters".to_string();
			
			} else {
				self.win.rename_error = String::new();
			}

			if !self.win.rename_error.is_empty() {
				ui.label(&self.win.rename_error);
			}
			
			ui.add_space(5.);

			ui.horizontal(|ui| {
				if ui.button("Rename").clicked() && self.win.rename_error.is_empty() {
					let filename = format!("{}.json", 
						rename_data["name"].as_str().unwrap());
	
					let fullpath = &self.processes_dir.join(filename);
	
					rename_data["friendlyName"] = json!(self.win.rename_to);

					if let Err(e) = std::fs::write(&fullpath, 
									rename_data.to_string().as_bytes()) {

						crate::log!("{:?} Could not be written ({})", fullpath, e);
					}

					
					self.refresh();
					self.win.rename_to = String::new();

					self.win.rename_window = false;
				}

				if ui.button("Cancel").clicked() {
					self.win.rename_error = String::new();
					self.win.rename_to = String::new();
					self.win.rename_window = false;
				}
			});
		});
	}
}