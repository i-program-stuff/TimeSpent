use crate::utils::format_time;
use crate::globals;

use serde_json::{json, Value as JsonValue};

use eframe::egui;
use egui_extras::{Column, TableBuilder, TableRow};

use crate::TimeSpent;

impl TimeSpent {
	fn draw_columns(&mut self, row: &mut TableRow, d: JsonValue, is_hidden: bool) {
		// Name Column
		row.col(|ui| {
			let name = d["friendlyName"].as_str().unwrap().to_string();

			if is_hidden {
				ui.colored_label(egui::Color32::DARK_GRAY, "⊗");

			} else if self.show_hidden {
				ui.colored_label(egui::Color32::DARK_GRAY, "○");
			}
			
			let response = ui.add(egui::Button::new(&name)
							 .fill(egui::Color32::TRANSPARENT));

			if response.clicked() {
				crate::open_window!(
					self.win.status_window, self.win.status_data, &d);
			}

			response.context_menu(|ui| {
				self.draw_context_menu(name, &d, ui);
			});
		});

		// Today Column
		row.col(|ui| {
			let today = globals::get_date();

			if let Some(time) = d["perDayTimeRun"][today].as_f64() {
				ui.strong(format_time(time));
			} else {
				ui.strong("0s");
			}
			
		});

		// Total Column
		row.col(|ui| {
			ui.strong(format_time(d["totalTimeRun"].as_f64().unwrap()));
		});
	}

	pub fn draw_table(&mut self, ui: &mut egui::Ui) {
		TableBuilder::new(ui)
		.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
		.column(Column::initial(200.).clip(true))
		.column(Column::initial(140.).clip(true))
		.column(Column::remainder())
		.resizable(true)
		.striped(true)

		.header(20., |mut header| {
			for title in ["Name", "Today", "Total"] {
				header.col(|ui| {
					ui.heading( title );
				});
			}
		})

		.body(|mut body| { 
			for data in self.data.clone() {

				let is_hidden = 
					self.hidden_processes.contains(&json!(data["name"]));

				if !self.show_hidden && is_hidden {
					continue
				}
				
				body.row(20., |mut row| {
					self.draw_columns(&mut row, data, is_hidden)
				});
			}
		});
	}
}