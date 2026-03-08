use anyhow::Result;
use bollard::Docker;
use bollard::container::{ListContainersOptions, StartContainerOptions, StopContainerOptions, RestartContainerOptions};

use crate::containers::container_info::{ContainerInfo, ContainerStatus, PortMapping};

pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    pub async fn new() -> Result<Self> {
        let docker = Docker::connect_with_socket_defaults()?;
        Ok(Self { docker })
    }
    
    pub async fn list_containers(&self, all: bool) -> Result<Vec<ContainerInfo>> {
        let options = ListContainersOptions::<String> {
            all,
            ..Default::default()
        };
        
        let containers = self.docker.list_containers(Some(options)).await?;
        let mut container_infos = Vec::new();
        
        for container in containers {
            let id = container.id.clone().unwrap_or_default();
            let short_id = if id.len() > 12 { id[..12].to_string() } else { id.clone() };
            
            let names = container.names.clone().unwrap_or_default();
            let name = names.first()
                .map(|n| n.trim_start_matches('/').to_string())
                .unwrap_or_else(|| short_id.clone());
            
            let image = container.image.clone().unwrap_or_default();
            let state = container.state.clone().unwrap_or_default();
            let status = ContainerStatus::from_docker_state(&state);
            
            let created = container.created.unwrap_or(0);
            let created_dt = chrono::DateTime::from_timestamp(created, 0)
                .unwrap_or_else(chrono::Utc::now);
            
            let ports = container.ports.clone().unwrap_or_default()
                .into_iter()
                .map(|p| PortMapping {
                    container_port: p.private_port,
                    host_port: p.public_port.unwrap_or(0),
                    protocol: p.typ.map(|t| format!("{:?}", t).to_lowercase()).unwrap_or_default(),
                    host_ip: p.ip.unwrap_or_default(),
                })
                .collect();
            
            container_infos.push(ContainerInfo {
                id: id.clone(),
                short_id,
                name,
                image,
                status,
                state,
                created: created_dt,
                ports,
                cpu_percent: 0.0,
                memory_usage: 0,
                memory_limit: 0,
                memory_percent: 0.0,
                network_rx: 0,
                network_tx: 0,
                is_running: status == ContainerStatus::Running,
            });
        }
        
        Ok(container_infos)
    }
    
    pub async fn start_container(&self, container_id: &str) -> Result<()> {
        self.docker.start_container(container_id, None::<StartContainerOptions<String>>).await?;
        Ok(())
    }
    
    pub async fn stop_container(&self, container_id: &str) -> Result<()> {
        self.docker.stop_container(container_id, Some(StopContainerOptions { t: 10 })).await?;
        Ok(())
    }
    
    pub async fn restart_container(&self, container_id: &str) -> Result<()> {
        self.docker.restart_container(container_id, Some(RestartContainerOptions { t: 10 })).await?;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub async fn remove_container(&self, container_id: &str) -> Result<()> {
        self.docker.remove_container(container_id, None).await?;
        Ok(())
    }
}
