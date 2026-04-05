#![allow(dead_code)]
// Functions used by both the GUI and the Daemon, so it has to be in a separate file.

use std::{path::PathBuf, io::Write};

pub mod config;
pub mod tracker;

#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {
		crate::shared::write_log(format!($($arg)*))
	}
}

pub fn get_file_name(path: &PathBuf) -> &str {
	return path.file_name().and_then(|n| n.to_str()).unwrap_or("");
}

pub struct Dirs {
	pub processes_dir: PathBuf,
	pub unlock_file: PathBuf,
	pub daemon_config: PathBuf,
	pub hidden_processes: PathBuf,
	pub log_file: PathBuf,
}

impl Dirs {
	pub fn new() -> Self {
		let data_dir = dirs::data_dir().unwrap().join("Time-Spent");

		return Self {
			processes_dir: data_dir.join("processes"),
			unlock_file: data_dir.join("donotwrite.txt"),
			daemon_config: data_dir.join("daemon.json"),
			hidden_processes: data_dir.join("hidden.json"),
			log_file: data_dir.join("log.txt"),
		}
	}
}

pub fn write_log(msg: String) {
	let log_file = Dirs::new().log_file;

	let time = chrono::Local::now().format("[%X %v]");

	let file_result = std::fs::OpenOptions::new()
						.append(true)
						.create(true)
						.open(log_file);
	
	println!("{}", msg);

	match file_result {
		Ok(mut file) => {
			if let Err(e) = writeln!(file, "{} {}", time, msg) {
				eprintln!("Couldn't write to Log ({})", e)
			};
		},

		Err(e) => {
			eprintln!("Couldn't make Log File ({})", e)
		},
	}
}

pub fn get_current_timestamp() -> i64 {
	return chrono::Utc::now().timestamp();
}

pub fn get_date_from_timestamp(timestamp: i64) -> String {
	format_timestamp_with(timestamp, "%Y/%m/%d")
}

pub fn format_timestamp_with(timestamp: i64, format: &str) -> String {
	chrono::DateTime::from_timestamp(timestamp, 0)
		.unwrap().with_timezone(&chrono::Local)
		.format(format).to_string()
}

pub fn get_todays_date() -> String {
	let time = chrono::Local::now();
	return time.format("%Y/%m/%d").to_string()
}