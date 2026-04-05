#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "../shared.rs"]
mod shared;
use shared::tracker;

mod platform;

use std::{thread, process, path::Path, time::Duration};
use sysinfo::{SystemExt, PidExt, ProcessExt};

fn get_focused_application_from_pid(system: &sysinfo::System, pid: u32) -> &Path {
	if let Some(process) = system.process(sysinfo::Pid::from_u32(pid)) {
		return process.exe()
	}

	return Path::new("/")
}

fn main() {
	let file_dirs = shared::Dirs::new();
	// let config_file = file_dirs.daemon_config;

	// let config = match write::get_config(&config_file) {
	// 	Ok(json) => json,

	// 	Err(e) => {
	// 		log!("Couldn't write Config ({})", e);
	// 		write::get_default_config()
	// 	}
	// };

	// Make sysinfo only refresh the process list
	let only_processes = sysinfo::ProcessRefreshKind::new();
	let r = sysinfo::RefreshKind::new().with_processes(only_processes);
	
	let mut system = sysinfo::System::new_with_specifics(r);

	// If timespent-daemon is already running, then stop execution
	if system.processes_by_name("timespent-daemon").count() > 1 {
		log!("The Daemon is already running");
		process::exit(1);
	}

	let flush_delta = 10000;

	let tracker_error_handling = |e: tracker::TrackerError| {
		log!("Couldn't initialize Tracker. ({})", e);
		process::exit(1);
	};

	let mut tracker = tracker::Tracker::new(flush_delta)
						.unwrap_or_else(tracker_error_handling);

	let poll_interval = Duration::from_secs(1);

	let mut last_pid = 0;
	let mut exe = Path::new("/");

	loop {
		if file_dirs.unlock_file.exists() {
			tracker.flush();
			drop(tracker);

			while file_dirs.unlock_file.exists() {
				thread::sleep(poll_interval);
			}

			tracker = tracker::Tracker::new(flush_delta)
						.unwrap_or_else(tracker_error_handling);
		}

		let pid = platform::get_pid();
		if last_pid != pid {
			system.refresh_processes_specifics(only_processes);
			exe = get_focused_application_from_pid(&system, pid);
			last_pid = pid;
		}

		tracker.add_time(&exe.to_path_buf(), poll_interval.as_secs() as i64);

		thread::sleep(poll_interval);
	}
}