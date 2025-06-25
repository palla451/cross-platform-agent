use sysinfo::{CpuExt, System, SystemExt};
use uuid::Uuid;
use std::fs;
use std::path::PathBuf;
use get_if_addrs::get_if_addrs;


#[derive(Debug)]
pub struct SystemInfo {
    pub uuid: String,
    pub hostname: Option<String>,
    pub os_version: Option<String>,
    pub total_memory: u64,
    pub used_memory: u64,
    pub cpu_cores: usize,
    pub cpu_brand: String,
    pub architecture: String,
    pub ip_address: Option<String>, // 🆕 campo aggiunto
}

pub fn collect_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_brand = sys
        .cpus()
        .get(0)
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".into());

    let architecture = std::env::consts::ARCH.to_string();

    let uuid = load_or_generate_uuid();

    let ip_address = get_if_addrs()
    .ok()
    .and_then(|interfaces| {
        interfaces
            .into_iter()
            .find(|iface| {
                iface.ip().is_ipv4() && !iface.is_loopback() // evita 127.0.0.1
            })
            .map(|iface| iface.ip().to_string())
    });
// 🆕 logica IP

    SystemInfo {
        uuid,
        hostname: sys.host_name(),
        os_version: sys.long_os_version(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        cpu_cores: sys.cpus().len(),
        cpu_brand,
        architecture,
        ip_address, // 🆕 incluso nella struct
    }
}

fn load_or_generate_uuid() -> String {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    path.push(".agent_id");

    if let Ok(contents) = fs::read_to_string(&path) {
        let uuid = contents.trim().to_string();
        if !uuid.is_empty() {
            return uuid;
        }
    }

    let new_uuid = Uuid::new_v4().to_string();
    let _ = fs::write(&path, &new_uuid);
    new_uuid
}
