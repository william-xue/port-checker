use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub protocol: String,
    pub local_addr: String,
    pub remote_addr: Option<String>,
    pub state: String,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}

#[cfg(target_os = "linux")]
pub fn scan_ports() -> Result<Vec<PortInfo>> {
    use std::fs;
    use std::process::Command;
    
    let mut ports = Vec::new();
    
    // 读取TCP连接
    if let Ok(tcp_content) = fs::read_to_string("/proc/net/tcp") {
        ports.extend(parse_proc_net(&tcp_content, "TCP")?);
    }
    
    // 读取TCP6连接
    if let Ok(tcp6_content) = fs::read_to_string("/proc/net/tcp6") {
        ports.extend(parse_proc_net(&tcp6_content, "TCP6")?);
    }
    
    // 读取UDP连接
    if let Ok(udp_content) = fs::read_to_string("/proc/net/udp") {
        ports.extend(parse_proc_net(&udp_content, "UDP")?);
    }
    
    // 读取UDP6连接
    if let Ok(udp6_content) = fs::read_to_string("/proc/net/udp6") {
        ports.extend(parse_proc_net(&udp6_content, "UDP6")?);
    }
    
    // 获取进程信息
    add_process_info(&mut ports)?;
    
    Ok(ports)
}

#[cfg(target_os = "macos")]
pub fn scan_ports() -> Result<Vec<PortInfo>> {
    use std::process::Command;
    
    let output = Command::new("netstat")
        .args(&["-an", "-p", "tcp"])
        .output()?;
    
    let mut ports = Vec::new();
    let tcp_output = String::from_utf8_lossy(&output.stdout);
    ports.extend(parse_macos_netstat(&tcp_output, "TCP")?);
    
    let output = Command::new("netstat")
        .args(&["-an", "-p", "udp"])
        .output()?;
    
    let udp_output = String::from_utf8_lossy(&output.stdout);
    ports.extend(parse_macos_netstat(&udp_output, "UDP")?);
    
    // 获取进程信息
    add_macos_process_info(&mut ports)?;
    
    Ok(ports)
}

#[cfg(target_os = "linux")]
fn parse_proc_net(content: &str, protocol: &str) -> Result<Vec<PortInfo>> {
    let mut ports = Vec::new();
    
    for (i, line) in content.lines().enumerate() {
        if i == 0 { continue; } // 跳过头部
        
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 10 { continue; }
        
        let local_addr = parse_address(fields[1])?;
        let remote_addr = if fields[2] == "00000000:0000" || fields[2] == "00000000000000000000000000000000:0000" {
            None
        } else {
            Some(parse_address(fields[2])?)
        };
        
        let state = parse_connection_state(fields[3], protocol)?;
        let inode = fields[9].parse::<u32>().unwrap_or(0);
        
        ports.push(PortInfo {
            protocol: protocol.to_string(),
            local_addr,
            remote_addr,
            state,
            pid: None, // 将在后续步骤中填充
            process_name: None,
        });
    }
    
    Ok(ports)
}

#[cfg(target_os = "linux")]
fn parse_address(addr_str: &str) -> Result<String> {
    let parts: Vec<&str> = addr_str.split(':').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid address format"));
    }
    
    let addr_hex = parts[0];
    let port_hex = parts[1];
    
    // 解析地址（小端序）
    let addr = if addr_hex.len() == 8 {
        // IPv4
        let addr_num = u32::from_str_radix(addr_hex, 16)?;
        format!("{}.{}.{}.{}",
            addr_num & 0xFF,
            (addr_num >> 8) & 0xFF,
            (addr_num >> 16) & 0xFF,
            (addr_num >> 24) & 0xFF
        )
    } else if addr_hex.len() == 32 {
        // IPv6 - 简化处理，显示为hex格式
        format!("[{}]", addr_hex)
    } else {
        addr_hex.to_string()
    };
    
    // 解析端口
    let port = u16::from_str_radix(port_hex, 16)?;
    
    Ok(format!("{}:{}", addr, port))
}

#[cfg(target_os = "linux")]
fn parse_connection_state(state_hex: &str, protocol: &str) -> Result<String> {
    if protocol.starts_with("UDP") {
        return Ok("UDP".to_string());
    }
    
    let state_num = u8::from_str_radix(state_hex, 16)?;
    let state = match state_num {
        0x01 => "ESTABLISHED",
        0x02 => "SYN_SENT",
        0x03 => "SYN_RECV",
        0x04 => "FIN_WAIT1",
        0x05 => "FIN_WAIT2",
        0x06 => "TIME_WAIT",
        0x07 => "CLOSE",
        0x08 => "CLOSE_WAIT",
        0x09 => "LAST_ACK",
        0x0A => "LISTEN",
        0x0B => "CLOSING",
        _ => "UNKNOWN",
    };
    
    Ok(state.to_string())
}

#[cfg(target_os = "macos")]
fn parse_macos_netstat(output: &str, protocol: &str) -> Result<Vec<PortInfo>> {
    let mut ports = Vec::new();
    
    for line in output.lines() {
        let line = line.trim();
        if !line.starts_with("tcp") && !line.starts_with("udp") {
            continue;
        }
        
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 6 { continue; }
        
        let local_addr = fields[3].to_string();
        let remote_addr = if fields[4] == "*.*" || fields[4].starts_with("0.0.0.0") {
            None
        } else {
            Some(fields[4].to_string())
        };
        
        let state = if protocol == "UDP" {
            "UDP".to_string()
        } else {
            fields[5].to_string()
        };
        
        ports.push(PortInfo {
            protocol: protocol.to_string(),
            local_addr,
            remote_addr,
            state,
            pid: None,
            process_name: None,
        });
    }
    
    Ok(ports)
}

#[cfg(target_os = "macos")]
fn add_macos_process_info(ports: &mut Vec<PortInfo>) -> Result<()> {
    use std::process::Command;
    
    for port_info in ports.iter_mut() {
        // 提取端口号 (macOS netstat使用点号分隔)
        let port = if let Some(dot_pos) = port_info.local_addr.rfind('.') {
            port_info.local_addr[dot_pos + 1..].parse::<u16>().unwrap_or(0)
        } else {
            continue;
        };
        
        if port == 0 {
            continue;
        }
        
        // 使用lsof查找特定端口的进程信息
        let protocol = if port_info.protocol.to_lowercase().contains("tcp") {
            "tcp"
        } else {
            "udp"
        };
        
        let output = Command::new("lsof")
            .args(["-i", &format!("{}:{}", protocol, port), "-P", "-n"])
            .output();
            
        if let Ok(output) = output {
            if output.status.success() {
                let lsof_output = String::from_utf8_lossy(&output.stdout);
                if let Some((pid, process_name)) = parse_lsof_for_port(&lsof_output) {
                    port_info.pid = Some(pid);
                    port_info.process_name = Some(process_name);
                }
            }
        }
    }
    
    Ok(())
}

#[cfg(target_os = "macos")]
fn parse_lsof_for_port(lsof_output: &str) -> Option<(u32, String)> {
    for line in lsof_output.lines().skip(1) { // 跳过标题行
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() >= 2 {
            let process_name = fields[0].to_string();
            if let Ok(pid) = fields[1].parse::<u32>() {
                return Some((pid, process_name));
            }
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn parse_lsof_output(output: &str, ports: &mut Vec<PortInfo>) -> Result<()> {
    for line in output.lines() {
        if line.starts_with("COMMAND") { continue; }
        
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 9 { continue; }
        
        let process_name = fields[0];
        let pid = fields[1].parse::<u32>().ok();
        let name_field = fields[8];
        
        // 解析地址信息
        if let Some(arrow_pos) = name_field.find("->") {
            let local_part = &name_field[..arrow_pos];
            if let Some(colon_pos) = local_part.rfind(':') {
                let port_str = &local_part[colon_pos + 1..];
                
                // 查找匹配的端口并更新信息
                for port in ports.iter_mut() {
                    if port.local_addr.ends_with(&format!(":{}", port_str)) {
                        port.pid = pid;
                        port.process_name = Some(process_name.to_string());
                    }
                }
            }
        } else if let Some(colon_pos) = name_field.rfind(':') {
            let port_str = &name_field[colon_pos + 1..];
            
            // 查找匹配的端口并更新信息
            for port in ports.iter_mut() {
                if port.local_addr.ends_with(&format!(":{}", port_str)) {
                    port.pid = pid;
                    port.process_name = Some(process_name.to_string());
                }
            }
        }
    }
    
    Ok(())
}

#[cfg(target_os = "linux")]
fn add_process_info(ports: &mut Vec<PortInfo>) -> Result<()> {
    // 这里可以通过 lsof 或者读取 /proc/*/fd/* 来获取进程信息
    // 为了简化，我们使用 netstat 命令
    use std::process::Command;
    
    let output = Command::new("netstat")
        .args(&["-tlnp"])
        .output();
    
    if let Ok(output) = output {
        let netstat_str = String::from_utf8_lossy(&output.stdout);
        parse_netstat_output(&netstat_str, ports)?;
    }
    
    Ok(())
}

#[cfg(target_os = "linux")]
fn parse_netstat_output(output: &str, ports: &mut Vec<PortInfo>) -> Result<()> {
    for line in output.lines() {
        if !line.starts_with("tcp") && !line.starts_with("udp") {
            continue;
        }
        
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 7 { continue; }
        
        let local_addr = fields[3];
        let pid_process = fields[6];
        
        if let Some(slash_pos) = pid_process.find('/') {
            if let Ok(pid) = pid_process[..slash_pos].parse::<u32>() {
                let process_name = &pid_process[slash_pos + 1..];
                
                // 查找匹配的端口并更新信息
                for port in ports.iter_mut() {
                    if port.local_addr.contains(&extract_port_from_addr(local_addr)) {
                        port.pid = Some(pid);
                        port.process_name = Some(process_name.to_string());
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn extract_port_from_addr(addr: &str) -> String {
    if let Some(colon_pos) = addr.rfind(':') {
        addr[colon_pos + 1..].to_string()
    } else {
        addr.to_string()
    }
}

// Windows 实现
#[cfg(windows)]
pub fn scan_ports() -> Result<Vec<PortInfo>> {
    use std::process::Command;
    
    let output = Command::new("netstat")
        .args(&["-ano"])
        .output()?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    parse_windows_netstat(&output_str)
}

#[cfg(windows)]
fn parse_windows_netstat(output: &str) -> Result<Vec<PortInfo>> {
    let mut ports = Vec::new();
    
    for line in output.lines() {
        let line = line.trim();
        if !line.starts_with("TCP") && !line.starts_with("UDP") {
            continue;
        }
        
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 4 { continue; }
        
        let protocol = fields[0].to_string();
        let local_addr = fields[1].to_string();
        let remote_addr = if fields[2] == "*:*" || fields[2] == "0.0.0.0:0" {
            None
        } else {
            Some(fields[2].to_string())
        };
        
        let (state, pid) = if protocol == "UDP" {
            ("UDP".to_string(), fields.get(3).and_then(|s| s.parse().ok()))
        } else {
            let state = fields.get(3).unwrap_or(&"UNKNOWN").to_string();
            let pid = fields.get(4).and_then(|s| s.parse().ok());
            (state, pid)
        };
        
        ports.push(PortInfo {
            protocol,
            local_addr,
            remote_addr,
            state,
            pid,
            process_name: None, // 可以通过 tasklist 获取
        });
    }
    
    // 获取进程名称
    if let Ok(tasklist_output) = Command::new("tasklist").args(&["/fo", "csv"]).output() {
        let tasklist_str = String::from_utf8_lossy(&tasklist_output.stdout);
        let pid_to_name = parse_tasklist(&tasklist_str);
        
        for port in &mut ports {
            if let Some(pid) = port.pid {
                port.process_name = pid_to_name.get(&pid).cloned();
            }
        }
    }
    
    Ok(ports)
}

#[cfg(windows)]
fn parse_tasklist(output: &str) -> HashMap<u32, String> {
    let mut pid_to_name = HashMap::new();
    
    for (i, line) in output.lines().enumerate() {
        if i == 0 { continue; } // 跳过头部
        
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 2 {
            let name = fields[0].trim_matches('"');
            if let Ok(pid) = fields[1].trim_matches('"').parse::<u32>() {
                pid_to_name.insert(pid, name.to_string());
            }
        }
    }
    
    pid_to_name
}

pub fn scan_specific_port(port: u16, protocol: &str) -> Result<Option<PortInfo>> {
    let all_ports = scan_ports()?;
    
    for port_info in all_ports {
        if port_info.protocol.to_lowercase().starts_with(&protocol.to_lowercase()) {
            // 处理不同的地址格式
            let port_str = port.to_string();
            if port_info.local_addr.contains(&format!(":{}", port_str)) ||
               port_info.local_addr.contains(&format!(".{}", port_str)) ||
               port_info.local_addr.ends_with(&port_str) {
                return Ok(Some(port_info));
            }
        }
    }
    
    Ok(None)
}