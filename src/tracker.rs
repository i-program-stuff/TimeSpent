#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::{
    shared::{self, get_current_timestamp, get_date_from_timestamp}, 
};

use std::{path::PathBuf, collections::BTreeMap};

pub type Timestamp = u64;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessEntry {
    pub name: String,
    pub key: String,
    pub path: PathBuf,
}

impl ProcessEntry {
    pub const NULL: Self = Self {
        name: String::new(),
        key: String::new(),
        path: PathBuf::new(),
    };

    pub fn new(path: PathBuf) -> Self {
        let name = crate::shared::get_file_name(&path);

        return Self {
            name: name.to_string(),
            key: name.to_string(),
            path,
        }
    }

    pub fn serialize(&self) -> Option<Vec<u8>> {
        bincode::serialize(self).ok()
    }

    pub fn deserialize(data: &[u8]) -> Option<Self> {
        bincode::deserialize(data).ok()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub start_time: Timestamp,
    pub duration: u32,
}

impl Session {
    pub fn new(start_time: Timestamp) -> Self {
        return Self {
            start_time,
            duration: 0,
        }
    }

    pub fn add_duration(&mut self, time: u32) {
        self.duration += time;
    }
}

pub struct Tracker {
    db: sled::Db,
    entries: sled::Tree,
    sessions: sled::Tree,
    current_process: ProcessEntry,
    current_session: Session,
}

impl Tracker {
    pub fn new(flush_delta: u64) -> Self {
        let processes_dir = shared::Dirs::new().processes_dir;

        let sled_config = sled::Config::new()
                            .path(processes_dir)
                            .flush_every_ms(Some(flush_delta))
                            .mode(sled::Mode::HighThroughput)
                            .use_compression(true);

        let db = sled_config.open().unwrap();

        return Self {
            entries: db.open_tree("entries").unwrap(),
            sessions: db.open_tree("sessions").unwrap(),

            db,

            current_process: ProcessEntry::NULL,
            current_session: Session::new(get_current_timestamp()),
        }
    }

    pub fn add_time(&mut self, path: &PathBuf, time_spent: u64) {
        let key = shared::get_file_name(path);

        // If the process is already being tracked, add the time to the current session
        if self.current_process.key == key {
            // do nothing

        } else if self.entries.contains_key(key.as_bytes()).unwrap() {
            self.write_current_session();

            if let Ok(Some(data)) = self.entries.get(key.as_bytes()) {
                if let Some(process_entry) = ProcessEntry::deserialize(&data) {
                    self.current_process = process_entry;
                }
            }
            
            self.current_session = Session::new(get_current_timestamp());

        } else {
            self.write_current_session();

            // Create a new entry for the process and add the time to it
            self.current_process = ProcessEntry::new(path.clone());

            self.current_process.serialize().map(|data| {
                let _ = self.entries.insert(key.as_bytes(), data);
            });

            self.current_session = Session::new(get_current_timestamp());
        }
        
        self.current_session.add_duration(time_spent as u32);

        if self.current_session.duration % 256 == 0 {
            self.write_current_session();
        }
    }

    fn write_current_session(&mut self) {
        // let key = vec![
        //     self.current_process.key.as_bytes(), 
        //     ":".as_bytes(), 
        //     &self.current_session.start_time.to_be_bytes(),
        // ].concat();

        let key = format!("{}:{}", self.current_process.key, self.current_session.start_time);

        let _ = self.sessions.insert(
            &key, &self.current_session.duration.to_le_bytes()
        );
    }

    pub fn flush(&mut self) {
        self.write_current_session();
        let _ = self.db.flush();
    }
}

// Used for temporarily opening the db when it's locked by the tracker.
fn remove_lock_und_open_db() -> sled::Db {
    let dirs = shared::Dirs::new();

    std::fs::File::create(&dirs.unlock_file).unwrap();

    let sled_config = sled::Config::new()
                    .path(dirs.processes_dir)
                    .flush_every_ms(None)
                    .use_compression(true);

    loop {
        match sled_config.open() {
            Ok(opened_db) => break opened_db,
            Err(_) => {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }
}

fn lock_and_close_db(db: sled::Db) {
    drop(db);
    std::fs::remove_file(&shared::Dirs::new().unlock_file).unwrap();
}

// fn apply_batch_to_entries(batch: sled::Batch) {
//     let db = remove_lock_und_open_db();
//     let entries = db.open_tree("entries").unwrap();

//     entries.apply_batch(batch).unwrap();
//     db.flush().unwrap();

//     lock_and_close_db(db);
// }

pub fn remove_entry(key: &str) {
    let db = remove_lock_und_open_db();
    let entries = db.open_tree("entries").unwrap();
    let sessions = db.open_tree("sessions").unwrap();

    entries.remove(key.as_bytes()).unwrap();

    let prefix = format!("{}:", key);
    let keys_to_remove: Vec<_> = sessions.scan_prefix(prefix.as_bytes())
        .filter_map(|entry| entry.ok())
        .map(|(key, _)| key)
        .collect();

    for key in keys_to_remove {
        sessions.remove(key).unwrap();
    }

    db.flush().unwrap();
    lock_and_close_db(db);
}

pub fn change_entry_name(key: &str, new_name: String) {
    let db = remove_lock_und_open_db();
    let entries = db.open_tree("entries").unwrap();

    if let Ok(Some(data)) = entries.get(key.as_bytes()) {
        if let Some(mut process_entry) = ProcessEntry::deserialize(&data) {
            process_entry.name = new_name;

            process_entry.serialize().map(|data| {
                let _ = entries.insert(key.as_bytes(), data);
            });
        }
    }

    db.flush().unwrap();
    lock_and_close_db(db);
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormattedProcessEntry {
    pub name: String,
    pub key: String,
    pub path: String,
    pub total_time: u32,

    // Will be sorted by date and formatted as (date, time).
    pub per_day_time: BTreeMap<String, u32>, 
    
}

// tuple.0 is timestamp and tuple.1 is time spent in seconds
fn categorize_into_days(sessions: Vec<(u64, u32)>) -> BTreeMap<String, u32> {
    let mut categorized: BTreeMap<String, u32> = BTreeMap::new();

    for (date, time) in sessions {
        *categorized.entry(get_date_from_timestamp(date)).or_insert(0) += time;
    }

    categorized
}

pub fn get_formatted_data() -> Vec<FormattedProcessEntry> {
    let db: sled::Db = remove_lock_und_open_db();

    let entries = db.open_tree("entries").unwrap().iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|(_key, value)| ProcessEntry::deserialize(&value));

    let sessions = db.open_tree("sessions").unwrap();

    let formatted_data = entries.filter_map(|entry| {
        let total_time = sessions.scan_prefix(entry.key.as_bytes())
            .filter_map(|session| session.ok())
            .filter_map(|(_, value)| {
                Some(u32::from_le_bytes(value.as_ref().try_into().ok()?))
            })
            .sum::<u32>();
        
        let per_day_time_as_timestamps = sessions.scan_prefix(entry.key.as_bytes())
            .filter_map(|session| session.ok())
            .filter_map(|(key, value)| {
                let key_str = String::from_utf8(key.to_vec()).ok()?;
                let timestamp_str = key_str.split(':').nth(1)?;

                let time = u32::from_le_bytes(value.as_ref().try_into().ok()?);

                Some((timestamp_str.parse::<u64>().ok()?, time))
            })
            .collect::<Vec<_>>();

        let per_day_time = categorize_into_days(per_day_time_as_timestamps);
        
        if entry.key.is_empty() {
            return None
        }

        Some(FormattedProcessEntry {
            name: entry.name.clone(),
            key: entry.key.clone(),
            path: entry.path.to_str().unwrap_or("").to_string(),
            total_time,
            per_day_time,
        })
    }).collect();

    lock_and_close_db(db);

    return formatted_data;
}