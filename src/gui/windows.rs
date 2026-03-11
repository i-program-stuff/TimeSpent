mod status_window;
mod rename_window;
mod delete_window;

use eframe::egui;

use crate::TimeSpent;

#[derive(Default)]
pub struct Window {
	pub status_window: bool,
	pub status_data: serde_json::Value,

	pub raw_data_window: bool,
	pub raw_data: serde_json::Value,

	pub delete_window: bool,
	pub delete_data: serde_json::Value,

	pub rename_window: bool,
	pub rename_data: serde_json::Value,
	pub rename_to: String,
	pub rename_error: String,
}

impl TimeSpent {
	pub fn draw_raw_data_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Raw Data").open(&mut self.win.raw_data_window)
		.vscroll(true).show(ctx, |ui| {
			let json = serde_json::to_string_pretty(&self.win.raw_data);

			ui.add( egui::TextEdit::multiline(&mut json.unwrap())
					.code_editor()
					.interactive(false)
			)
		});
	}
}