use anyhow::Result;
use serde_json;
use serde_yaml;
use csv;
use tabled::{Table, Tabled};
use crate::port_scanner::PortInfo;
use crate::config::OutputFormat;

#[derive(Tabled)]
struct PortInfoTable {
    #[tabled(rename = "Protocol")]
    protocol: String,
    #[tabled(rename = "Local Address")]
    local_addr: String,
    #[tabled(rename = "Remote Address")]
    remote_addr: String,
    #[tabled(rename = "State")]
    state: String,
    #[tabled(rename = "PID")]
    pid: String,
    #[tabled(rename = "Process")]
    process_name: String,
}

impl From<&PortInfo> for PortInfoTable {
    fn from(port: &PortInfo) -> Self {
        Self {
            protocol: port.protocol.clone(),
            local_addr: port.local_addr.clone(),
            remote_addr: port.remote_addr.clone().unwrap_or("-".to_string()),
            state: port.state.clone(),
            pid: port.pid.map(|p| p.to_string()).unwrap_or("-".to_string()),
            process_name: port.process_name.clone().unwrap_or("-".to_string()),
        }
    }
}

pub fn format_ports(ports: &[PortInfo], format: &OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Table => format_table(ports),
        OutputFormat::Json => format_json(ports),
        OutputFormat::Yaml => format_yaml(ports),
        OutputFormat::Csv => format_csv(ports),
    }
}

fn format_table(ports: &[PortInfo]) -> Result<String> {
    if ports.is_empty() {
        return Ok("No ports found.".to_string());
    }
    
    let table_data: Vec<PortInfoTable> = ports.iter().map(PortInfoTable::from).collect();
    let table = Table::new(table_data).to_string();
    Ok(table)
}

fn format_json(ports: &[PortInfo]) -> Result<String> {
    let json = serde_json::to_string_pretty(ports)?;
    Ok(json)
}

fn format_yaml(ports: &[PortInfo]) -> Result<String> {
    let yaml = serde_yaml::to_string(ports)?;
    Ok(yaml)
}

fn format_csv(ports: &[PortInfo]) -> Result<String> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    
    // 写入标题行
    wtr.write_record(&["Protocol", "Local Address", "Remote Address", "State", "PID", "Process"])?;
    
    // 写入数据行
    for port in ports {
        wtr.write_record(&[
            &port.protocol,
            &port.local_addr,
            &port.remote_addr.clone().unwrap_or("-".to_string()),
            &port.state,
            &port.pid.map(|p| p.to_string()).unwrap_or("-".to_string()),
            &port.process_name.clone().unwrap_or("-".to_string()),
        ])?;
    }
    
    let data = String::from_utf8(wtr.into_inner()?)?;
    Ok(data)
}

/// 格式化单个端口信息（用于pick和reserve命令）
pub fn format_single_port(port: u16, status: &str, process: Option<&str>) -> String {
    match process {
        Some(proc) => format!("Port {}: {} ({})", port, status, proc),
        None => format!("Port {}: {}", port, status),
    }
}

/// 格式化端口列表（简化版本）
pub fn format_port_list(ports: &[u16]) -> String {
    if ports.is_empty() {
        "No ports".to_string()
    } else if ports.len() == 1 {
        format!("Port {}", ports[0])
    } else {
        format!("Ports: {}", ports.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "))
    }
}