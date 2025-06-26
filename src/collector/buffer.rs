use std::fs;
use std::path::Path;
use crate::collector::payload::Payload;
use chrono::Utc;

pub fn save_to_buffer(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    // ✅ Serializza il payload in JSON leggibile
    let json = serde_json::to_string_pretty(payload)?;

    // ✅ Crea la directory se non esiste
    let buffer_dir = Path::new("output/buffer");
    fs::create_dir_all(buffer_dir)?;

    // ✅ Costruisce il percorso del file con timestamp UTC
    let timestamp = Utc::now().timestamp();
    let filename = buffer_dir.join(format!("{}.json", timestamp));

    // ✅ Scrive il file
    fs::write(&filename, json)?;

    println!("💾 Salvato il payload in {}", filename.display());
    Ok(())
}
