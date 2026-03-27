#![allow(dead_code)]

use std::{fs, path::Path};
use serde_json::{Value as JsonValue};

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
