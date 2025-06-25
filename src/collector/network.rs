use std::process::Command;
use std::str;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub protocol: String,
    pub local_address: String,
    pub remote_address: String,
    pub pid_program: Option<String>,
}

/// Raccoglie le connessioni di rete attive usando il comando `ss -tunap`.
pub fn collect_network_connections() -> Vec<ConnectionInfo> {
    let output = Command::new("ss")
        .args(["-tunap"])
        .output()
        .expect("failed to run ss");

    if !output.status.success() {
        eprintln!("Errore nell'esecuzione di ss: {}", output.status);
        return vec![];
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_ss_output(&stdout)
}

/// Analizza l'output di `ss -tunap` e costruisce un vettore di ConnectionInfo.
fn parse_ss_output(output: &str) -> Vec<ConnectionInfo> {
    let mut connections = Vec::new();

    for line in output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 6 {
            continue;
        }

        let protocol = parts[0].to_string();
        let local_address = parts[4].to_string();
        let remote_address = parts[5].to_string();
        let pid_program = extract_pid_program(line);

        connections.push(ConnectionInfo {
            protocol,
            local_address,
            remote_address,
            pid_program,
        });
    }

    connections
}

/// Estrae informazioni su PID e programma da una riga `ss` (se presenti).
fn extract_pid_program(line: &str) -> Option<String> {
    if let Some(start) = line.find("users:(") {
        let substring = &line[start..];
        if let Some(end) = substring.find(')') {
            return Some(substring[7..=end].to_string());
        }
    }
    None
}

