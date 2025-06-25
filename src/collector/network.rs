use serde::Serialize;
use std::process::Command;

#[derive(Serialize, Debug)]
pub struct NetConnection {
    pub protocol: String,
    pub local_address: String,
    pub remote_address: String,
    pub pid: Option<u32>,
}

pub fn collect_network_connections() -> Vec<NetConnection> {
    let output = Command::new("ss")
        .args(&["-tunp"])
        .output()
        .expect("failed to run ss");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut connections = Vec::new();

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            continue;
        }

        let protocol = parts[0].to_string();
        let local_address = parts[4].to_string();
        let remote_address = parts[5].to_string();

        let pid = line
            .split("pid=")
            .nth(1)
            .and_then(|s| s.split(',').next())
            .and_then(|s| s.parse().ok());

        connections.push(NetConnection {
            protocol,
            local_address,
            remote_address,
            pid,
        });
    }

    connections
}
