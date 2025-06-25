use std::fs;
use std::io::{Write, Read};
use std::path::PathBuf;

use crate::collector::payload::Payload;

/// Percorso snapshot
const SNAPSHOT_PATH: &str = "/output/.agent_snapshot.json";

pub fn save_snapshot(payload: &Payload) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(payload)?;

    let mut path = PathBuf::from(SNAPSHOT_PATH);
    fs::create_dir_all(path.parent().unwrap())?;

    println!("Scrivo snapshot in {:?}", path);

    let mut file = fs::File::create(path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn has_payload_changed(payload: &Payload) -> bool {
    let path = PathBuf::from(SNAPSHOT_PATH);

    if !path.exists() {
        println!("📂 Nessuno snapshot precedente: invio forzato.");
        return true;
    }

    let mut contents = String::new();
    match fs::File::open(&path).and_then(|mut f| f.read_to_string(&mut contents)) {
        Ok(_) => match serde_json::from_str::<Payload>(&contents) {
            Ok(previous) => has_diff(&previous, payload),
            Err(_) => {
                println!("⚠️  Errore parsing snapshot. Reinvio forzato.");
                true
            }
        },
        Err(_) => {
            println!("⚠️  Errore lettura snapshot. Reinvio forzato.");
            true
        }
    }
}

/// Confronta due payload e verifica se ci sono nuovi PID o nuove connessioni
fn has_diff(old: &Payload, new: &Payload) -> bool {
    let old_pids: std::collections::HashSet<_> = old.processes.iter().map(|p| p.pid).collect();
    let new_pids: std::collections::HashSet<_> = new.processes.iter().map(|p| p.pid).collect();

    if !new_pids.is_subset(&old_pids) {
        println!("🧠 Nuovi PID trovati.");
        return true;
    }

    let old_conns: std::collections::HashSet<_> = old.connections.iter()
        .map(|c| (c.local_address.clone(), c.remote_address.clone()))
        .collect();

    let new_conns: std::collections::HashSet<_> = new.connections.iter()
        .map(|c| (c.local_address.clone(), c.remote_address.clone()))
        .collect();

    if !new_conns.is_subset(&old_conns) {
        println!("🌐 Nuove connessioni trovate.");
        return true;
    }

    false
}
