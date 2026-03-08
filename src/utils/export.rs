use crate::containers::ContainerInfo;

pub struct Exporter;

impl Exporter {
    pub fn to_json(containers: &[ContainerInfo]) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(containers)
    }
    
    pub fn to_csv(containers: &[ContainerInfo]) -> Result<String, Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        
        wtr.write_record(["ID", "Name", "Image", "Status", "CPU %", "Memory"])?;
        
        for container in containers {
            wtr.write_record([
                &container.short_id,
                &container.name,
                &container.image,
                container.status.as_str(),
                &format!("{:.1}%", container.cpu_percent),
                &format_bytes(container.memory_usage),
            ])?;
        }
        
        let data = wtr.into_inner()?;
        Ok(String::from_utf8_lossy(&data).to_string())
    }
}

fn format_bytes(bytes: u64) -> String {
    const MB: u64 = 1024 * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else {
        format!("{} KB", bytes / 1024)
    }
}
