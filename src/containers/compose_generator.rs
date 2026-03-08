use crate::containers::ContainerInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeFile {
    pub version: String,
    pub services: HashMap<String, ComposeService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeService {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ports: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
}

pub struct ComposeGenerator;

impl ComposeGenerator {
    pub fn generate(containers: &[ContainerInfo]) -> Result<String, serde_yaml::Error> {
        let mut services = HashMap::new();
        
        for container in containers {
            let service_name = Self::sanitize_name(&container.name);
            
            let ports: Vec<String> = container.ports.iter()
                .map(|p| format!("{}:{}:{}", p.host_port, p.container_port, p.protocol))
                .collect();
            
            let restart = if container.is_running {
                Some("unless-stopped".to_string())
            } else {
                None
            };
            
            let service = ComposeService {
                image: container.image.clone(),
                container_name: Some(container.name.clone()),
                ports,
                restart,
            };
            
            services.insert(service_name, service);
        }
        
        let compose = ComposeFile {
            version: "3.8".to_string(),
            services,
        };
        
        serde_yaml::to_string(&compose)
    }
    
    fn sanitize_name(name: &str) -> String {
        name.chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
            .collect::<String>()
            .to_lowercase()
    }
}
