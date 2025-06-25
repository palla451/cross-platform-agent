use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use chrono::{Utc, Duration};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::collector::payload::Payload;

const MAX_BUFFER_SIZE_BYTES: u64 = 1_048_576; // 1MB

pub fn save_to_buffer(payload: &Payload) -> std::io::Result<()> {
    let dir = PathBuf::from("/output/buffer");
    fs::create_dir_all(&dir)?;

    // 🔁 Esegui pulizia prima di salvare
    cleanup_old_files(&dir)?;

    let json = serde_json::to_string_pretty(payload)?;
    let timestamp = Utc::now().format("%Y%m%dT%H%M%S%.fZ");
    let mut file_path = dir.clone();
    file_path.push(format!("payload-{}.json", timestamp));

    println!(" Scrivo buffer in {:?}", file_path);
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

/// Elimina file buffer più vecchi di 24h o se superano 1MB totali
fn cleanup_old_files(dir: &PathBuf) -> std::io::Result<()> {
    let entries = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .collect::<Vec<_>>();

    let now = SystemTime::now();
    let day_ago = now - Duration::hours(24).to_std().unwrap();

    let mut total_size: u64 = 0;
    let mut files = Vec::new();

    for entry in entries {
        let metadata = entry.metadata()?;
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let size = metadata.len();

        if modified < day_ago {
            fs::remove_file(entry.path())?;
            println!(" Rimosso file vecchio: {:?}", entry.path());
        } else {
            total_size += size;
            files.push((entry.path(), modified, size));
        }
    }

    if total_size > MAX_BUFFER_SIZE_BYTES {
        // Ordina da più vecchio a più recente
        files.sort_by_key(|(_, modified, _)| *modified);
        for (path, _, size) in files {
            fs::remove_file(&path)?;
            println!(" Rimosso per spazio: {:?}", path);
            total_size -= size;
            if total_size <= MAX_BUFFER_SIZE_BYTES {
                break;
            }
        }
    }

    Ok(())
}
