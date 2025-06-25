mod collector;

use collector::info::collect_system_info;
use collector::network::collect_network_connections;
use collector::process::get_processes;

fn main() {
    // ✅ Info client
    let sysinfo = collect_system_info();
    println!("🖥️ Client Info:");
    println!("UUID: {}", sysinfo.uuid);
    println!("Hostname: {}", sysinfo.hostname.as_deref().unwrap_or("N/A"));
    println!("OS: {}", sysinfo.os_version.as_deref().unwrap_or("N/A"));
    println!("CPU: {} × {}", sysinfo.cpu_brand, sysinfo.cpu_cores);
    println!("Architettura: {}", sysinfo.architecture);
    println!("IP address: {}", sysinfo.ip_address.as_deref().unwrap_or("N/A"));
    println!("RAM: {} MB / {} MB", sysinfo.used_memory / 1024, sysinfo.total_memory / 1024);


    // ✅ Processi attivi
    let processes = get_processes();
    println!("\n🧠 Processi attivi:");
    for proc in processes {
        println!(
            "PID: {}, Name: {}, CPU: {:.2}%",
            proc.pid, proc.name, proc.cpu_usage
        );
    }

    // ✅ Connessioni di rete
    let connections = collect_network_connections();
    println!("\n🌐 Connessioni di rete attive:");
    for conn in connections {
        println!(
            "[{}] {} → {} (pid: {})",
            conn.protocol,
            conn.local_address,
            conn.remote_address,
            conn.pid_program.as_deref().unwrap_or("N/A")
        );
    }
}
