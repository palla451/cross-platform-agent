mod collector;

use collector::info::collect_system_info;
use collector::network::collect_network_connections;
use collector::processes::get_processes;

fn main() {
    // ✅ Info client
    let sysinfo = collect_system_info();
    println!("🖥️ Client Info:
{:#?}", sysinfo);

    // ✅ Processi attivi
    let processes = get_processes();
    println!("
🧠 Processi attivi:");
    for proc in processes {
        println!(
            "PID: {}, Name: {}, CPU: {:.2}%",
            proc.pid, proc.name, proc.cpu_usage
        );
    }

    // ✅ Connessioni di rete
    let connections = collect_network_connections();
    println!("
🌐 Connessioni di rete attive:");
    for conn in connections {
        println!(
            "[{}] {} → {} (pid: {})",
            conn.protocol,
            conn.local_address,
            conn.remote_address,
            conn.pid.map(|p| p.to_string()).unwrap_or_else(|| "N/A".into())
        );
    }
}
