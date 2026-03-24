#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "../shared.rs"]
mod shared;
mod utils;
mod table;
mod windows;
mod context_menu;

use crate::shared::tracker;

use eframe::egui;

struct TimeSpent {
	data: Vec<tracker::FormattedProcessEntry>,
	win: windows::Window,

	hidden_processes: Vec<String>,
	show_hidden: bool,
}

impl TimeSpent {
	fn new() -> Self {
		let hidden_processes = utils::get_hidden_processes();

		let mut data = tracker::get_formatted_data();
		table::sort_data_by(table::SortMethod::PerDayTime, &mut data);

		let win = windows::Window::default();

		return Self { 
			data, win,
			hidden_processes, show_hidden: false
		}
	}

	fn refresh(&mut self) {
		self.hidden_processes = 
			utils::get_hidden_processes();

		self.data = tracker::get_formatted_data();
	}

	fn draw_footerbar(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::bottom("footer").default_height(30.)
		.show(ctx, |ui| {
			ui.set_min_width(380.);
			ui.horizontal_centered(|ui| {
				if ui.button("Search").clicked() {
					self.win.search_window = !self.win.search_window;
				}

				if ui.button("Refresh").clicked() {
					self.refresh();
				}

				let hide_button_text = format!("{} Hidden Items", 
									   if self.show_hidden {"Hide"} else {"Show"});

				if ui.button(hide_button_text).clicked() {
					self.show_hidden = !self.show_hidden;
				}

				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					egui::widgets::global_theme_preference_switch(ui);

					ui.separator();

					let mut current_scale = ctx.pixels_per_point();
					
					egui::ComboBox::from_id_salt("Scaling")
						.selected_text(format!("Scale: {:.1}x", current_scale))
						.show_ui(ui, |ui| {
							let scales = [0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
							
							for scale in scales {
								if ui.selectable_value(
									&mut current_scale, scale, format!("{:.1}x", scale)
								).clicked() {
									ctx.set_pixels_per_point(current_scale);
								}
							}
						});
				});
			});
		});
	}
}

impl eframe::App for TimeSpent {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::Frame::NONE
				.outer_margin(egui::Margin {
					left: 0, right: 0,
					top: 0, bottom: 25,
				})
				.show(ui, |ui| {
					self.draw_table(ui);

					ui.separator();
				});

			self.draw_raw_data_window(ctx);
			self.draw_status_window(ctx);
			self.draw_search_window(ctx);
			self.draw_delete_window(ctx);
			self.draw_rename_window(ctx);
		});

		self.draw_footerbar(ctx);
	}
}

fn main() -> eframe::Result {
	let icon_data = include_bytes!("../../imgs/hummingbird_new.ico");

	let icon = image::load_from_memory_with_format(
		icon_data, image::ImageFormat::Ico
	).expect("Could not load icon").blur(3.5).to_rgba8();


	let viewport = egui::viewport::ViewportBuilder::default()
		.with_title("Time Spent")
		.with_inner_size(egui::Vec2::new(550., 560.))
		.with_resizable(true)
		.with_icon(egui::viewport::IconData {
			width: icon.width(),
			height: icon.height(),
			
			rgba: icon.into_raw(),
		});

	let win_opts = eframe::NativeOptions {
		viewport,
		..Default::default()
	};
	
	eframe::run_native("Time Spent", win_opts,
		Box::new(|_cc| Ok(Box::new(TimeSpent::new()))))
}