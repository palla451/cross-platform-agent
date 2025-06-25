use serde::{Serialize, Deserialize};

use super::info::SystemInfo;
use super::process::ProcessInfo;
use super::network::ConnectionInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub system: SystemInfo,
    pub processes: Vec<ProcessInfo>,
    pub connections: Vec<ConnectionInfo>,
}
