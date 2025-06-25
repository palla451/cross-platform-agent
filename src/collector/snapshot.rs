use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::collector::payload::Payload;

pub fn save_snapshot(payload: &Payload) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(payload)?;

    let mut path = PathBuf::from("/output");
    path.push(".agent_snapshot.json");

    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
