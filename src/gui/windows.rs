// Deals with the windows of the GUI and the data used by them.

mod status_window;
mod rename_window;
mod delete_window;

use eframe::egui;

use crate::TimeSpent;
use crate::shared::tracker::FormattedProcessEntry;

#[derive(Default)]
pub struct Window {
	pub status_window: bool,
	pub status_data: FormattedProcessEntry,

	pub raw_data_window: bool,
	pub raw_data: FormattedProcessEntry,

	pub delete_window: bool,
	pub delete_data: FormattedProcessEntry,

	pub rename_window: bool,
	pub rename_data: FormattedProcessEntry,
	pub rename_to: String,
	pub rename_error: String,
}

impl TimeSpent {
	pub fn draw_raw_data_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Raw Data").open(&mut self.win.raw_data_window)
		.vscroll(true).show(ctx, |ui| {
			let mut text = format!("{:#?}", self.win.raw_data);

			ui.add( egui::TextEdit::multiline(&mut text)
					.code_editor()
					.interactive(false)
			)
		});
	}
}