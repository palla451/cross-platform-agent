use serde::Serialize;
use sysinfo::{System, SystemExt, UserExt};

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub hostname: Option<String>,
    pub os_version: Option<String>,
    pub total_memory: u64,
    pub user: Option<String>,
}

pub fn collect_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let user = sys.users().get(0).map(|u| u.name().to_string());

    SystemInfo {
        hostname: sys.host_name(),
        os_version: sys.long_os_version(),
        total_memory: sys.total_memory(),
        user,
    }
}
