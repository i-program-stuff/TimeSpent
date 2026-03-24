use eframe::egui;

use crate::TimeSpent;
use crate::shared::tracker;

impl TimeSpent {
	pub fn draw_rename_window(&mut self, ctx: &egui::Context) {
		let mut rename_window = self.win.rename_window;
		let mut should_open = self.win.rename_window;
		
		egui::Window::new("Rename").open(&mut rename_window).show(ctx, |ui| {
			ui.heading(
				format!("What should \"{}\" be renamed to?", self.win.rename_data.name)
			);

			ui.add_space(3.);		

			ui.add(
				egui::widgets::TextEdit::singleline(&mut self.win.rename_to)
				.hint_text("New Name")
				.desired_width(130.)
			);

			if self.win.rename_to.len() > 25 {
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
					tracker::change_entry_name(
						&self.win.rename_data.key, self.win.rename_to.clone()
					);

					self.refresh();
					self.win.rename_to = String::new();

					should_open = false;
				}

				if ui.button("Cancel").clicked() {
					self.win.rename_error = String::new();
					self.win.rename_to = String::new();
					should_open = false;
				}
			});
		});

		self.win.rename_window = rename_window && should_open;
	}
}