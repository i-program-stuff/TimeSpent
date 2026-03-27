use serde_json::json;

use crate::{log, shared};

use std::{fs, collections::HashSet};

pub fn get_hidden_processes() -> HashSet<String> {
	let hidden_processes_file = shared::Dirs::new().hidden_processes;

	if !hidden_processes_file.exists() {
		match fs::write(&hidden_processes_file, "[]".as_bytes()) {
			Ok(_) => log!("hidden.json Created"),
			Err(_) => log!("hidden.json Couldn't be Created"),
		}
	}

	let raw_content = fs::read_to_string(&hidden_processes_file)
					  .unwrap_or("[]".to_string());

	let content = match serde_json::from_str(&raw_content) {
		Ok(cont) => cont,
		Err(e) => {
			log!("Couldn't process hidden.json ({})", e);
			serde_json::json!([])
		},
	};

    return content.as_array().unwrap().iter()
        .filter_map(|arr| Some(arr.as_str()?.to_string()))
        .collect()
}

pub fn save_hidden_processes(hidden_processes: &HashSet<String>) {
	match std::fs::write(
		shared::Dirs::new().hidden_processes,
		json!(hidden_processes).to_string().as_bytes()
	) {
		Ok(_) => {},
		Err(_) => {
			crate::log!("hidden.json Couldn't be written to disk")
		},
	}
}

pub fn format_time(time_in_secs: f64) -> String {
	let days = (time_in_secs / 86400.).floor();
	let hours = (time_in_secs / 3600.).floor() - days * 24.;
	let mins = ((time_in_secs / 60.) % 60.).floor();
	let secs = time_in_secs % 60.;

	let time = [days, hours, mins, secs];
	let time_symbols = ["d", "h", "m", "s"];

	let mut parts = Vec::new();
	for (t, l) in time.iter().zip(time_symbols) {
		if *t != 0. {
			parts.push(format!("{}{}", t, l));
		}
	}

	if parts.is_empty() {
		return "0s".to_string();
	}

	return parts.join(" ")
}