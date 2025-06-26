use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt, PidExt};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cpu_usage: f32,
    pub sha256: Option<String>,
    pub md5: Option<String>,
    pub exe_path: Option<String>,
    pub folder: Option<String>,
}

pub fn get_processes() -> Vec<ProcessInfo> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything())
    );
    sys.refresh_processes();

    sys.processes()
        .into_iter()
        .map(|(pid, proc_)| {
            let exe_path = proc_.exe().to_str().map(|s| s.to_string());
            let folder = proc_.exe().parent().and_then(|p| p.to_str().map(|s| s.to_string()));

            let (sha256, md5) = if let Ok(mut file) = File::open(proc_.exe()) {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_ok() {
                    let sha256 = format!("{:x}", Sha256::digest(&buffer));
                    let md5 = format!("{:x}", md5::compute(&buffer));
                    (Some(sha256), Some(md5))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

            ProcessInfo {
                pid: pid.as_u32() as i32,
                name: proc_.name().to_string(),
                cpu_usage: proc_.cpu_usage(),
                sha256,
                md5,
                exe_path,
                folder,
            }
        })
        .collect()
}
