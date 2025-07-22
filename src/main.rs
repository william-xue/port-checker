use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::*;
use serde_json;
use tabled::{Table, Tabled};

mod port_scanner;
use port_scanner::{PortInfo, scan_ports, scan_specific_port};

#[derive(Parser)]
#[command(name = "port-checker")]
#[command(about = "A fast and user-friendly command-line tool to check port usage")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all occupied ports on the system
    List {
        /// Filter by protocol (tcp/udp)
        #[arg(short, long)]
        protocol: Option<String>,
        /// Show only listening ports
        #[arg(short, long)]
        listening: bool,
        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },
    /// Check if a specific port is in use
    Check {
        /// Port number to check
        port: u16,
        /// Protocol to check (tcp/udp)
        #[arg(short, long)]
        protocol: Option<String>,
    },
    /// Find the process using a specific port
    Find {
        /// Port number to find
        port: u16,
        /// Protocol type (default: tcp)
        #[arg(short, long, default_value = "tcp")]
        protocol: String,
    },
    /// Display port usage statistics
    Stats,
    /// Kill process using a specific port
    Kill {
        /// Port number to kill
        port: u16,
        /// Protocol (tcp/udp)
        #[arg(short, long, default_value = "tcp")]
        protocol: String,
        /// Force kill without confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Tabled)]
struct PortDisplay {
    #[tabled(rename = "Protocol")]
    protocol: String,
    #[tabled(rename = "Local Address")]
    local_address: String,
    #[tabled(rename = "Remote Address")]
    remote_address: String,
    #[tabled(rename = "State")]
    state: String,
    #[tabled(rename = "PID")]
    pid: String,
    #[tabled(rename = "Process Name")]
    process_name: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { protocol, listening, json } => {
            handle_list_command(protocol, listening, json)?
        }
        Commands::Check { port, protocol } => {
            handle_check_command(port, protocol)?
        }
        Commands::Find { port, protocol } => {
            handle_find_command(port, protocol)?
        }
        Commands::Stats => {
            handle_stats_command()?
        }
        Commands::Kill { port, protocol, force } => {
            handle_kill_command(port, protocol, force)?
        }
    }

    Ok(())
}

fn handle_list_command(protocol: Option<String>, listening: bool, json: bool) -> Result<()> {
    let mut ports = scan_ports()?;

    // 过滤协议
    if let Some(proto) = protocol {
        ports.retain(|p| p.protocol.to_lowercase().starts_with(&proto.to_lowercase()));
    }

    // 过滤监听端口
    if listening {
        ports.retain(|p| p.state == "LISTEN" || p.state == "UDP");
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&ports)?);
    } else {
        display_ports_table(&ports);
        println!("\n{}: {} ports found", "Total".bold(), ports.len());
    }

    Ok(())
}

fn handle_check_command(port: u16, protocol: Option<String>) -> Result<()> {
    let protocols = if let Some(proto) = protocol {
        vec![proto]
    } else {
        vec!["tcp".to_string(), "udp".to_string()]
    };

    for proto in protocols {
        match scan_specific_port(port, &proto)? {
            Some(_) => {
                println!("{} Port {} ({}) is {}", 
                    "🔴".red(), 
                    port, 
                    proto.to_uppercase(), 
                    "IN USE".red().bold());
            }
            None => {
                println!("{} Port {} ({}) is {}", 
                    "🟢".green(), 
                    port, 
                    proto.to_uppercase(), 
                    "FREE".green().bold());
            }
        }
    }

    Ok(())
}

fn handle_find_command(port: u16, protocol: String) -> Result<()> {
    match scan_specific_port(port, &protocol)? {
        Some(port_info) => {
            println!("{} Process using port {} ({}):\n", 
                "🔍".blue(), 
                port, 
                protocol.to_uppercase());
            
            println!("  {}: {}", "Process ID".bold(), 
                port_info.pid.map_or("Unknown".to_string(), |p| p.to_string()));
            println!("  {}: {}", "Process Name".bold(), 
                port_info.process_name.unwrap_or("Unknown".to_string()));
            println!("  {}: {}", "Local Address".bold(), port_info.local_addr);
            if let Some(remote) = port_info.remote_addr {
                println!("  {}: {}", "Remote Address".bold(), remote);
            }
            println!("  {}: {}", "State".bold(), port_info.state);
        }
        None => {
            println!("{} No process found using port {} ({})", 
                "❌".red(), 
                port, 
                protocol.to_uppercase());
        }
    }

    Ok(())
}

fn handle_stats_command() -> Result<()> {
    let ports = scan_ports()?;
    
    let mut tcp_count = 0;
    let mut udp_count = 0;
    let mut listening_count = 0;
    let mut established_count = 0;
    
    for port in &ports {
        if port.protocol.to_lowercase().starts_with("tcp") {
            tcp_count += 1;
            if port.state == "LISTEN" {
                listening_count += 1;
            } else if port.state == "ESTABLISHED" {
                established_count += 1;
            }
        } else if port.protocol.to_lowercase().starts_with("udp") {
            udp_count += 1;
        }
    }
    
    println!("{}", "📊 Port Usage Statistics".bold().blue());
    println!();
    println!("  {}: {}", "Total Ports".bold(), ports.len());
    println!("  {}: {}", "TCP Ports".bold(), tcp_count);
    println!("  {}: {}", "UDP Ports".bold(), udp_count);
    println!("  {}: {}", "Listening Ports".bold(), listening_count);
    println!("  {}: {}", "Established Connections".bold(), established_count);
    
    Ok(())
}

fn display_ports_table(ports: &[PortInfo]) {
    let display_ports: Vec<PortDisplay> = ports.iter().map(|p| PortDisplay {
        protocol: p.protocol.clone(),
        local_address: p.local_addr.clone(),
        remote_address: p.remote_addr.clone().unwrap_or("-".to_string()),
        state: p.state.clone(),
        pid: p.pid.map_or("-".to_string(), |pid| pid.to_string()),
        process_name: p.process_name.clone().unwrap_or("-".to_string()),
    }).collect();

    let table = Table::new(display_ports);
    println!("{}", table);
}

fn handle_kill_command(port: u16, protocol: String, force: bool) -> Result<()> {
    use std::process::Command;
    use std::io::{self, Write};
    
    // 查找占用端口的进程
    if let Some(port_info) = scan_specific_port(port, &protocol)? {
        if let Some(pid) = port_info.pid {
            let process_name = port_info.process_name.as_deref().unwrap_or("Unknown");
            
            println!("🔍 Found process using port {} ({}):", port, protocol.to_uppercase());
            println!("  Process ID: {}", pid);
            println!("  Process Name: {}", process_name);
            println!("  Local Address: {}", port_info.local_addr);
            
            if !force {
                print!("\n⚠️  Are you sure you want to kill this process? (y/N): ");
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("❌ Operation cancelled.");
                    return Ok(());
                }
            }
            
            // 终止进程
            #[cfg(unix)]
            let result = Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .status();
                
            #[cfg(windows)]
            let result = Command::new("taskkill")
                .args(["/F", "/PID", &pid.to_string()])
                .status();
            
            match result {
                Ok(status) if status.success() => {
                    println!("✅ Successfully killed process {} ({})", pid, process_name);
                    println!("🎉 Port {} is now available!", port);
                }
                Ok(_) => {
                    println!("❌ Failed to kill process {} ({})", pid, process_name);
                    println!("💡 You may need to run with sudo/administrator privileges");
                }
                Err(e) => {
                    println!("❌ Error killing process: {}", e);
                }
            }
        } else {
            println!("❌ Could not determine process ID for port {}", port);
        }
    } else {
        println!("🟢 Port {} ({}) is not in use", port, protocol.to_uppercase());
    }
    
    Ok(())
}