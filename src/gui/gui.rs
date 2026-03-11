#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "../globals.rs"]
mod globals;
mod utils;
mod table;
mod windows;
mod context_menu;

use std::path::PathBuf;

use eframe::egui;

struct TimeSpent {
	data: Vec<serde_json::Value>,
	win: windows::Window,
	processes_dir: PathBuf,

	hidden_processes_file: PathBuf,
	hidden_processes: Vec<serde_json::Value>,
	show_hidden: bool,
}

impl TimeSpent {
	fn new() -> Self {
		let file_dirs = globals::Dirs::new();
		let processes_dir = file_dirs.processes_dir;
		let hidden_processes_file = file_dirs.hidden_processes;

		let hidden_processes = 
			utils::get_hidden_processes(&hidden_processes_file);

		let data = utils::get_info(&processes_dir);

		let win = windows::Window::default();

		return Self { 
			data, win, processes_dir, hidden_processes_file,
			hidden_processes, show_hidden: false
		}
	}

	fn refresh(&mut self) {
		self.hidden_processes = 
			utils::get_hidden_processes(&self.hidden_processes_file);

		self.data = utils::get_info(&self.processes_dir);
	}

	fn draw_footerbar(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::bottom("footer").default_height(30.)
		.show(ctx, |ui| {
			ui.horizontal_centered(|ui| {
				if ui.button("Refresh").clicked() {
					self.refresh();
				}

				let hide_button_text = format!("{} Hidden Items", 
									   if self.show_hidden {"Hide"} else {"Show"});

				if ui.button(hide_button_text).clicked() {
					self.show_hidden = !self.show_hidden;
				}

				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					egui::widgets::global_theme_preference_buttons(ui);
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

			if self.win.delete_window {
				self.draw_delete_window(ctx);
			}

			if self.win.rename_window {
				self.draw_rename_window(ctx);
			}
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
	// win_opts.initial_window_size = Some(egui::Vec2::new(550., 560.));
	// win_opts.resizable = true;
	// win_opts.default_theme = win_opts.system_theme()
	// 						 .unwrap_or(eframe::Theme::Dark);
	// win_opts.icon_data = Some(eframe::IconData {
	// 	width: icon.width(),
	// 	height: icon.height(),
		
	// 	rgba: icon.into_raw(),
	// });

	eframe::run_native("Time Spent", win_opts,
		Box::new(|_cc| Ok(Box::new(TimeSpent::new()))))
}