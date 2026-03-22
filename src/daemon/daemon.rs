#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "../shared.rs"]
mod shared;
mod write;

use shared::tracker;

use std::{thread, process, path::Path, time::Duration};
use sysinfo::{SystemExt, PidExt, ProcessExt};

#[cfg(target_os = "windows")]
fn get_pid() -> u32 {
	use winapi::um::winuser;

	let mut pid: u32 = 0;

	unsafe {
		let hwnd = winuser::GetForegroundWindow();
		winuser::GetWindowThreadProcessId(hwnd, &mut pid);
	}

	return pid
}

#[cfg(target_os = "linux")]
fn get_pid() -> u32 {
	let output_opt = process::Command::new("xdotool")
					 .args(["getwindowfocus", "getwindowpid"])
					 .output();
	
	if output_opt.is_err() {
		log!("Failed to execute process");
		log!("Is xdotool installed?");

		process::exit(1);
	}

	let mut output = output_opt.unwrap().stdout;
	output.pop(); // To remove \n from the end

	let pid_string = String::from_utf8_lossy(&output);
	let pid: u32 = pid_string.parse().unwrap_or(0);

	return pid
}

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

	// If TimeSpentDaemon is already running, then stop execution
	if system.processes_by_name("TimeSpentDaemon").count() > 1 {
		log!("The Daemon is already running");
		process::exit(1);
	}

	let flush_delta = 10000;

	let mut tracker = tracker::Tracker::new(flush_delta);

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

			tracker = tracker::Tracker::new(flush_delta);
		}

		let pid = get_pid();
		if last_pid != pid {
			system.refresh_processes_specifics(only_processes);
			exe = get_focused_application_from_pid(&system, pid);
			last_pid = pid;
		}

		tracker.add_time(&exe.to_path_buf(), poll_interval.as_secs());

		thread::sleep(poll_interval);
	}
}