use serde_json::json;

use crate::{TimeSpent, shared};
use crate::shared::tracker::FormattedProcessEntry;

#[macro_export]
// Open Window if the process is clicked for the first time
// Close Window if the same process is clicked
// Change Window data if another process is clicked
macro_rules! open_window {
	( $window:expr, $window_data:expr, $data:expr ) => {
		$window = $data != &$window_data;

		if &$window_data == $data {
			$window_data = FormattedProcessEntry::default();
		} else {
			$window_data = $data.clone();
		}
	};
}


impl TimeSpent {
	pub fn draw_context_menu(&mut self, name: String, 
		data: &FormattedProcessEntry, ui: &mut eframe::egui::Ui) {
		
		let hide_button_text = 
			if self.hidden_processes.contains(&data.name) {
				"Unhide" 
			} else {
				 "Hide" 
			};

		eframe::egui::ScrollArea::vertical().show(ui, |ui| {
			ui.vertical_centered(|ui| {
				ui.monospace(name);
			});
	
			ui.add_space(3.);
	
			if ui.button("More Info").clicked() {
				open_window!(self.win.status_window, self.win.status_data, data);
				ui.close();
			}
	
			if ui.button("Rename").clicked() {
				open_window!(self.win.rename_window, self.win.rename_data, data);
				ui.close();
			}
			
			if ui.button(hide_button_text).clicked() {
				
				if hide_button_text == "Hide" { 
					self.hidden_processes.push( data.name.clone() );
				} else {
					// Remove that process from the list of hidden processes
					self.hidden_processes.retain( |x| x != &data.name );
				}
				
				match std::fs::write(shared::Dirs::new().hidden_processes,
					  json!(self.hidden_processes).to_string().as_bytes()) {
	
					Ok(_) => {},
					Err(_) => {
						crate::log!("hidden.json Couldn't be written to disk")
					},
				}
	
				ui.close();
			}
	
			if ui.button("Delete").clicked() {
				open_window!(self.win.delete_window, self.win.delete_data, data);
				ui.close();
			}
	
			if ui.button("Show Raw Data").clicked() {
				open_window!(self.win.raw_data_window, self.win.raw_data, data);
				ui.close();
			}
	
			if ui.button("Close the Menu").clicked() {
				ui.close();
			}
		});

	}
}