use sysinfo::{
    ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt, PidExt,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cpu_usage: f32,
}

pub fn get_processes() -> Vec<ProcessInfo> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything())
    );
    sys.refresh_processes();

    sys.processes()
        .into_iter()
        .map(|(pid, proc_)| ProcessInfo {
            pid: pid.as_u32() as i32,
            name: proc_.name().to_string(),
            cpu_usage: proc_.cpu_usage(),
        })
        .collect()
}
