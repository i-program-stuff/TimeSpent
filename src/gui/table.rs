use eframe::egui;
use egui_extras::{Column, TableBuilder, TableRow};

use nucleo_matcher;

use crate::{shared::{self, tracker::FormattedProcessEntry}, utils::format_time};
use crate::TimeSpent;

pub enum SortMethod {
	Title,
	TotalTime,
	PerDayTime,
	Search(String),
}

pub fn sort_data_by(method: SortMethod, data: &mut Vec<FormattedProcessEntry>) {
    match method {
        SortMethod::Title => {
            data.sort_by(|a, b| a.name.cmp(&b.name));
        },

        SortMethod::TotalTime => {
            data.sort_by(|a, b| b.total_time.cmp(&a.total_time));
        },

        SortMethod::PerDayTime => {
            data.sort_by(|a, b| {
                let val_a = a.per_day_time.last_key_value().map(|(_, &t)| t).unwrap_or(0);
                let val_b = b.per_day_time.last_key_value().map(|(_, &t)| t).unwrap_or(0);
                val_b.cmp(&val_a)
            });
        },

		SortMethod::Search(query) => {
			if query.is_empty() {
                return;
            }

            let mut matcher = nucleo_matcher::Matcher::new(nucleo_matcher::Config::DEFAULT);
            let pattern = nucleo_matcher::Utf32String::from(query);

            data.sort_by_cached_key(|entry| {
                let name_utf32 = nucleo_matcher::Utf32String::from(entry.name.as_str());
                
                let score = 
					matcher.fuzzy_match(name_utf32.slice(..), pattern.slice(..)).unwrap_or(0);
                
                std::cmp::Reverse(score)
            });
		}
    }
}

impl TimeSpent {
	fn draw_columns(&mut self, row: &mut TableRow, data: &FormattedProcessEntry, is_hidden: bool) {
		// Name Column
		row.col(|ui| {
			if is_hidden {
				ui.colored_label(egui::Color32::DARK_GRAY, "⊗");

			} else if self.show_hidden {
				ui.colored_label(egui::Color32::DARK_GRAY, "○");
			}
			
			let response = ui.add(egui::Button::new(data.name.clone())
							 .fill(egui::Color32::TRANSPARENT));

			if response.clicked() {
				crate::open_window!(
					self.win.status_window, self.win.status_data, data
				);
			}

			response.context_menu(|ui| {
				self.draw_context_menu(data.name.clone(), data, ui);
			});
		});

		// Today Column
		row.col(|ui| {
			let today = shared::get_todays_date();			
			let time = *data.per_day_time.get(&today).unwrap_or(&0) as f64;
			ui.strong(format_time(time));
		});

		// Total Column
		row.col(|ui| {
			ui.strong(format_time(data.total_time as f64));
		});
	}

	pub fn draw_table(&mut self, ui: &mut egui::Ui) {
		TableBuilder::new(ui)
		.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
		.column(Column::initial(200.).clip(true))
		.column(Column::initial(140.).clip(true))
		.column(Column::remainder().clip(true))
		.resizable(true)
		.striped(true)

		.header(26., |mut header| {
			for title in ["Name", "Today", "Total"] {
				header.col(|ui| {
					if ui.add(
						egui::Button::new(
							egui::RichText::new(title).size(20.)
						)
						.fill(egui::Color32::TRANSPARENT)
					).clicked() {
						let selected = match title {
							"Name" => SortMethod::Title,
							"Today" => SortMethod::PerDayTime,
							"Total" => SortMethod::TotalTime,
							_ => unreachable!(),
						};

						sort_data_by(selected, &mut self.data);
					}
				});	
			}
		})

		.body(|mut body| { 
			for data in self.data.clone() {

				let is_hidden = self.hidden_processes.contains(&data.name);

				if !self.show_hidden && is_hidden {
					continue
				}
				
				body.row(20., |mut row| {
					self.draw_columns(&mut row, &data, is_hidden)
				});
			}
		});
	}
}