// Deals with the windows of the GUI and the data used by them.

mod status_window;
mod rename_window;
mod delete_window;

use eframe::egui;

use crate::{TimeSpent, table};
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

	pub search_window: bool,
	pub search_query: String,
}

impl TimeSpent {
	pub fn draw_raw_data_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Raw Data").open(&mut self.win.raw_data_window)
		.vscroll(true).max_height(450.).show(ctx, |ui| {
			let mut text = format!("{:#?}", self.win.raw_data);

			ui.add( 
				egui::TextEdit::multiline(&mut text)
					.code_editor()
					.interactive(false)
					.desired_width(f32::INFINITY)
					.desired_rows(25)
			);

		});
	}

	pub fn draw_search_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("🔍 Search").open(&mut self.win.search_window)
		    .default_pos(egui::pos2(0.0, ctx.content_rect().max.y))
			.pivot(egui::Align2::LEFT_BOTTOM).collapsible(false).auto_sized()
			.show(ctx, |ui| {
			if ui.add(
				egui::TextEdit::singleline(&mut self.win.search_query).desired_width(220.0)
			).changed() {
				table::sort_data_by(
					table::SortMethod::Search(self.win.search_query.clone()),
					&mut self.data
				);
			}
		});
	}
}