use std::{fs, path::Path};
use serde_json::{json, Value as JsonValue};

// ==CONFIG DOCUMENTATION==
// "tickSpeed" Controls how often the application checks for the focused app
// it is recommended to not put it above 10
//
// "autoDeleteCorrupted" deletes corrupted json files
// (Disable it if you plan on manually editing the Json)
//
// "safeWrite" decreases the probability of json being corrupted
// disabling it might increase performance (NOT RECOMMENDED)

pub fn get_default_config() -> JsonValue {
	return serde_json::json!({
		"tickSpeed": 1,
		"autoDeleteCorrupted": true,
		"safeWrite": true,
	});
}

pub fn get_config(file_path: &Path) -> std::io::Result<JsonValue> {
	if !file_path.is_file() {
		let default_json = get_default_config();

		let pretty_json = serde_json::to_string_pretty(&default_json).unwrap();

		fs::write(&file_path, pretty_json.as_bytes())?;
	}

	let contents = fs::read_to_string(&file_path)?;

	return Ok(serde_json::from_str(&contents)?);
}


pub fn set_json_data(name: String, exe_dir: &Path, json_dir: &Path, config: &JsonValue) 
	-> std::io::Result<()> {
		
	let data_file = json_dir.join(format!("{}.json", name));

	let friendly_name = match name.rsplit_once(".") {
		Some(n) => n.0,
		None => &name,
	};

	if !data_file.is_file() {
		let j = json!({
			"name": name,
			"friendlyName": friendly_name,
			"exeLocation": exe_dir.to_str().unwrap(),
			"totalTimeRun": 0,
			"perDayTimeRun": {}
		});

		fs::write(&data_file, j.to_string().as_bytes())?;
	}

	let contents = fs::read_to_string(&data_file)?;

	let tick_speed = config["tickSpeed"].as_u64().unwrap_or(1);

	// totalTimeRun
	let mut info: JsonValue = match serde_json::from_str(&contents) {
		Ok(data) => data,
		Err(e) => {
			if config["autoDeleteCorrupted"].as_bool().unwrap_or(false) {
				if let Err(e) = fs::remove_file(&data_file) {
					crate::log!("Failed to remove corrupted file ({})", e);
				};
			}

			return Err(e.into())
		}
	};

	info["totalTimeRun"] = json!(info["totalTimeRun"].as_u64().unwrap() + tick_speed);

	// perDayTimeRun
	let today = crate::shared::get_todays_date();

	if info["perDayTimeRun"].get(&today).is_none() {
		info["perDayTimeRun"][&today] = json!(0);
	}

	info["perDayTimeRun"][&today] = 
		json!(info["perDayTimeRun"][&today].as_u64().unwrap() + tick_speed);
	//

	// safeWrite should decrease the chance of corruption when writing
	if config["safeWrite"].as_bool().unwrap_or(false) {
		let temp_file = json_dir.join("temp");
		fs::write(&temp_file, info.to_string().as_bytes())?;

		fs::rename(&temp_file, &data_file)?;
	} else {
		fs::write(&data_file, info.to_string().as_bytes())?;
	}

	Ok(())
}