use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub id: String,
    pub short_id: String,
    pub name: String,
    pub image: String,
    pub status: ContainerStatus,
    pub state: String,
    pub created: DateTime<Utc>,
    pub ports: Vec<PortMapping>,
    pub cpu_percent: f64,
    pub memory_usage: u64,
    pub memory_limit: u64,
    pub memory_percent: f64,
    pub network_rx: u64,
    pub network_tx: u64,
    pub is_running: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerStatus {
    Running,
    Paused,
    Exited,
    Created,
    Restarting,
    Dead,
    Unknown,
}

impl ContainerStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Running => "Running",
            Self::Paused => "Paused",
            Self::Exited => "Exited",
            Self::Created => "Created",
            Self::Restarting => "Restarting",
            Self::Dead => "Dead",
            Self::Unknown => "Unknown",
        }
    }
    
    pub fn from_docker_state(state: &str) -> Self {
        match state.to_lowercase().as_str() {
            "running" => Self::Running,
            "paused" => Self::Paused,
            "exited" => Self::Exited,
            "created" => Self::Created,
            "restarting" => Self::Restarting,
            "dead" => Self::Dead,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub container_port: u16,
    pub host_port: u16,
    pub protocol: String,
    pub host_ip: String,
}
