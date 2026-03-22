use eframe::egui;
use egui_plot as plot;

use crate::TimeSpent;

impl TimeSpent {
	pub fn draw_status_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Status").open(&mut self.win.status_window)
		.resizable(true).show(ctx, |ui| {
			let status_data = &self.win.status_data.per_day_time;

			ui.horizontal(|ui| {
				ui.set_min_width(220.);

				ui.label("Executable path: ");

				ui.add(
					egui::Label::new(
						egui::RichText::new(
							self.win.status_data.path.clone()
						).monospace()
					).wrap()
				);

			});

			ui.add_space(5.);

			let mut bar_data: Vec<plot::Bar> = Vec::new();
			for (i, d) in status_data.iter().enumerate() {
				let time = *d.1 as f64;

				bar_data.push(
					plot::Bar::new(
						i as f64 + 0.5,
						time / 60., // Convert to minutes
					)	
					.name(format!("{} on {}", // d.0 is the date
						crate::utils::format_time(time), d.0
					))
					.width(0.95)
				);
			}

			let bar_chart = plot::BarChart::new("Time", bar_data.clone())
							.color(egui::Color32::LIGHT_BLUE);

			ui.collapsing("Time Graph", |ui| {
				ui.monospace("X: Days");
				ui.monospace("Y: Time in Minutes");

				plot::Plot::new(self.win.status_data.name.clone())
				.show_x(false)
				.show_axes(false)
				.allow_boxed_zoom(false)
				.y_axis_formatter(|i, _| {
					if i.value > 0. {
						format!("{} minutes", i.value)
					} else {
						String::new()
					}
				})
				.coordinates_formatter(
					plot::Corner::LeftBottom,
					plot::CoordinatesFormatter::new(move |point, _| {
						let msg = "Use Ctrl+Scroll to Zoom in/out".to_string();

						// if X or Y is negative, then show a default message
						if point.x < 0. || point.y < 0. {
							return msg
						}

						let index = point.x.floor() as usize;
						if let Some(data) = bar_data.get(index) {
							format!("{}", data.name)
						} else {
							msg
						}
					})
				)
				.show(ui, |ui| {
					ui.bar_chart(bar_chart);
				});
			});
			
		});
	}
}
