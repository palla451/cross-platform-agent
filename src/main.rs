mod collector;

use collector::info::collect_system_info;
use collector::network::collect_network_connections;
use collector::process::get_processes;
use collector::payload::Payload;
use collector::snapshot::save_snapshot;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 🧱 Costruzione del payload aggregato
    let payload = Payload {
        system: collect_system_info(),
        processes: get_processes(),
        connections: collect_network_connections(),
    };

    // 📦 Serializzazione e stampa JSON completo
    let json = serde_json::to_string_pretty(&payload)?;
    println!("{}", json);

    // 💾 Salvataggio snapshot
    save_snapshot(&payload)?;

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
            "PID: {}, Name: {}, CPU: {:.2}%",
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
