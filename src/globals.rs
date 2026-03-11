// Functions used by both the GUI and the Daemon, so it has to be in a separate file. 

use chrono::offset::Local;
use std::{path::PathBuf, io::Write};

mod storage;

#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {
		crate::globals::write_log(format!($($arg)*))
	}
}

pub struct Dirs {
	pub processes_dir: PathBuf,
	pub daemon_config: PathBuf,
	pub hidden_processes: PathBuf,
	pub log_file: PathBuf,
}

impl Dirs {
	pub fn new() -> Self {
		let data_dir = dirs::data_dir().unwrap().join("Time-Spent");

		return Self {
			processes_dir: data_dir.join("processes"),
			daemon_config: data_dir.join("daemon.json"),
			hidden_processes: data_dir.join("hidden.json"),
			log_file: data_dir.join("log.txt"),
		}
	}
}

pub fn write_log(msg: String) {
	let log_file = Dirs::new().log_file;

	let time = Local::now().format("[%X %v]");

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

pub fn get_date() -> String {
	let time = Local::now();
	return time.format("%Y/%m/%d").to_string()
}