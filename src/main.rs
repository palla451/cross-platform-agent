mod collector;

use collector::info::collect_system_info;
use collector::network::collect_network_connections;
use collector::process::get_processes;
use collector::payload::Payload;
use collector::snapshot::{save_snapshot, has_payload_changed};
use collector::http::send_payload;
use collector::buffer::save_to_buffer;

use serde_json;
use dotenv::from_path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 📥 Carica variabili da /app/.env (necessario nel container)
    from_path("/app/.env").ok();

    println!(" URL_BACKEND={}", std::env::var("URL_BACKEND").unwrap_or("N/A".into()));
    println!(" BEARER_TOKEN={}", std::env::var("BEARER_TOKEN").unwrap_or("N/A".into()));

    // 🧱 Costruzione del payload aggregato
    let payload = Payload {
        system: collect_system_info(),
        processes: get_processes(),
        connections: collect_network_connections(),
    };

    // 📦 Serializzazione e stampa JSON completo
    let json = serde_json::to_string_pretty(&payload)?;
    println!("{}", json);

    // 🔄 Controlla se il payload è cambiato rispetto allo snapshot
    if has_payload_changed(&payload) {
        println!("📤 Inizio invio del payload...");

        match send_payload(&payload) {
            Ok(()) => {
                println!("✅ Payload inviato correttamente.");
                save_snapshot(&payload)?; // Salva snapshot solo se invio riuscito
            },
            Err(e) => {
                eprintln!("⚠️  Errore durante l'invio del payload: {}", e);
                eprintln!("🔍 Debug errore: {:?}", e);
                save_to_buffer(&payload)?;
                eprintln!("💾 Payload salvato in /output/buffer.");
            }
        }
    } else {
        println!("🟡 Nessun cambiamento rilevato: non invio nulla.");
    }

    // ✅ Info client leggibili
    println!("\n🖥️ Client Info:");
    println!("UUID: {}", payload.system.uuid);
    println!("Hostname: {}", payload.system.hostname.as_deref().unwrap_or("N/A"));
    println!("OS: {}", payload.system.os_version.as_deref().unwrap_or("N/A"));
    println!("CPU: {} × {}", payload.system.cpu_brand, payload.system.cpu_cores);
    println!("Architettura: {}", payload.system.architecture);
    println!("IP address: {}", payload.system.ip_address.as_deref().unwrap_or("N/A"));
    println!("RAM: {} MB / {} MB", payload.system.used_memory / 1024, payload.system.total_memory / 1024);

    // 🧠 Processi attivi
    println!("\n🧠 Processi attivi:");
    for proc in &payload.processes {
        println!(
            "PID: {}, Name: {}, CPU: {:.6}%",
            proc.pid, proc.name, proc.cpu_usage
        );
    }

    // 🌐 Connessioni di rete
    println!("\n🌐 Connessioni di rete attive:");
    for conn in &payload.connections {
        println!(
            "[{}] {} → {} (pid: {})",
            conn.protocol,
            conn.local_address,
            conn.remote_address,
            conn.pid_program.as_deref().unwrap_or("N/A")
        );
    }

    Ok(())
}
