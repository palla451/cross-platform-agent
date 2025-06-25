use std::process::Command;
use std::str;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub protocol: String,
    pub state: Option<String>,
    pub local_address: String,
    pub remote_address: String,
    pub pid_program: Option<String>,
}

/// Raccoglie le connessioni di rete attive usando `ss`.
pub fn collect_network_connections() -> Vec<ConnectionInfo> {
    let which_ss = Command::new("which")
        .arg("ss")
        .output()
        .expect("failed to run which ss");

    println!("ss path: {}", String::from_utf8_lossy(&which_ss.stdout).trim());

    // Prima prova con -tunap
    match try_collect_with_args(&["-tunap"]) {
        Ok(conns) => conns,
        Err(e) => {
            eprintln!("⚠️  ss -tunap fallito: {}. Provo con -tuna senza -p...", e);
            // Fallback senza -p
            match try_collect_with_args(&["-tuna"]) {
                Ok(conns) => conns,
                Err(e) => {
                    eprintln!("❌ anche ss -tuna fallito: {}", e);
                    vec![]
                }
            }
        }
    }
}

fn try_collect_with_args(args: &[&str]) -> Result<Vec<ConnectionInfo>, String> {
    let output = Command::new("ss")
        .args(args)
        .output()
        .map_err(|e| format!("Errore comando ss: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(parse_ss_output(&stdout))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Status {:?}, stderr: {}", output.status, stderr))
    }
}

/// Analizza l'output di `ss` e costruisce un vettore di ConnectionInfo.
fn parse_ss_output(output: &str) -> Vec<ConnectionInfo> {
    let mut connections = Vec::new();

    for line in output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            continue;
        }

        let protocol = parts[0].to_string();
        let state = Some(parts[1].to_string());
        let local_address = parts[4].to_string();
        let remote_address = parts[5].to_string();
        let pid_program = extract_pid_program(line);

        connections.push(ConnectionInfo {
            protocol,
            state,
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
        let substring = &line[start + 6..]; // salta "users:"
        if let Some(end) = substring.find(')') {
            return Some(substring[..=end].to_string());
        }
    }
    None
}
