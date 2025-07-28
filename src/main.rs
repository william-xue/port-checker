use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::*;

use std::str::FromStr;

mod port_scanner;
mod port_manager;
mod config;
mod formatter;

use port_scanner::{PortInfo, scan_ports, scan_specific_port};
use port_manager::{PortGuard, bind_random_port};
use config::{Config, OutputFormat};
use formatter::format_ports;
use tabled::{Tabled, Table};

#[derive(Parser)]
#[command(name = "port-checker")]
#[command(about = "A fast and user-friendly command-line tool to check port usage")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Output format
    #[arg(short = 'f', long = "format", global = true)]
    format: Option<String>,
    
    /// Config file path
    #[arg(short = 'c', long = "config", global = true)]
    config: Option<String>,
    
    /// Verbose output
    #[arg(short = 'v', long = "verbose", global = true)]
    verbose: bool,
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
    /// Allocate a random available port in specified range
    Pick {
        /// Start of port range
        #[arg(short, long, default_value = "8000")]
        start: u16,
        /// End of port range
        #[arg(short, long, default_value = "9000")]
        end: u16,
        /// Keep the port reserved (don't exit immediately)
        #[arg(short, long)]
        keep: bool,
    },
    /// Reserve a specific port and optionally run a command
    Reserve {
        /// Port number to reserve
        port: u16,
        /// Protocol (tcp/udp)
        #[arg(short, long, default_value = "tcp")]
        protocol: String,
        /// Command to run with the reserved port
        #[arg(short, long)]
        command: Option<String>,
        /// Keep the port reserved after command exits
        #[arg(short, long)]
        keep: bool,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show,
    /// Create default configuration file
    Init,
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
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
    
    // 加载配置
    let mut config = if let Some(config_path) = &cli.config {
        Config::load_from_file(config_path)?
    } else {
        Config::load_default()?
    };
    
    // 覆盖配置中的verbose设置
    if cli.verbose {
        config.verbose = true;
    }
    
    // 确定输出格式
    let output_format = if let Some(format_str) = &cli.format {
        OutputFormat::from_str(format_str)?
    } else {
        config.default_format.clone()
    };

    match cli.command {
        Commands::List { protocol, listening, json } => {
            handle_list_command(protocol, listening, json, &config, &output_format)?
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
        Commands::Pick { start, end, keep } => {
            handle_pick_command(start, end, keep)?
        }
        Commands::Reserve { port, protocol, command, keep } => {
            handle_reserve_command(port, protocol, command, keep)?
        }
        Commands::Config { action } => {
            handle_config_command(&action, &config)?
        }
    }

    Ok(())
}

fn display_ports_table(ports: &[PortInfo]) {
    if ports.is_empty() {
        println!("No ports found");
        return;
    }

    let display_ports: Vec<PortDisplay> = ports.iter().map(|p| PortDisplay {
        protocol: p.protocol.clone(),
        local_address: p.local_addr.clone(),
        remote_address: p.remote_addr.clone().unwrap_or_else(|| "-".to_string()),
        state: p.state.clone(),
        pid: p.pid.map_or_else(|| "-".to_string(), |pid| pid.to_string()),
        process_name: p.process_name.clone().unwrap_or_else(|| "-".to_string()),
    }).collect();

    let table = Table::new(display_ports);
    println!("{}", table);
}

fn handle_config_command(action: &ConfigAction, config: &Config) -> Result<()> {
    match action {
        ConfigAction::Show => {
            println!("Current configuration:");
            println!("  Default format: {:?}", config.default_format);
            println!("  Scan timeout: {}s", config.scan_timeout);
            println!("  Concurrent threads: {}", config.concurrent_threads);
            println!("  Verbose: {}", config.verbose);
            println!("  Show process info: {}", config.show_process_info);
            println!("  Port range: {}-{}", config.port_range.start, config.port_range.end);
            println!("  Default reserve duration: {}s", config.default_reserve_duration);
        }
        ConfigAction::Init => {
            Config::create_default_config()?;
        }
        ConfigAction::Set { key, value } => {
            println!("Setting {} = {}", key, value);
            // TODO: 实现配置设置功能
            println!("⚠️  Configuration setting not yet implemented");
        }
    }
    Ok(())
}

fn handle_pick_command(start: u16, end: u16, keep: bool) -> Result<()> {
    use std::io;
    
    println!("{} Allocating random port in range {}..{}", "🎲".blue(), start, end);
    
    let guard = bind_random_port(start, end)?;
    let port = guard.port();
    
    println!("{} Successfully allocated port: {}", "✅".green(), port.to_string().bold().green());
    println!("  {}: 127.0.0.1:{}", "Address".bold(), port);
    println!("  {}: TCP", "Protocol".bold());
    
    if keep {
        println!("\n{} Port {} is reserved. Press Ctrl+C to release.", "🔒".yellow(), port);
        
        // 设置 Ctrl+C 处理器
        let port_for_handler = port;
        ctrlc::set_handler(move || {
            println!("\n{} Releasing port {}...", "🔓".yellow(), port_for_handler);
            std::process::exit(0);
        }).expect("Error setting Ctrl-C handler");
        
        // 保持程序运行
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    } else {
        println!("\n{} Port {} allocated successfully. Use it quickly before this program exits!", "⚡".yellow(), port);
        println!("{} Press Enter to release the port...", "💡".blue());
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
    }
    
    Ok(())
}

fn handle_reserve_command(port: u16, protocol: String, command: Option<String>, keep: bool) -> Result<()> {
    use std::io;
    
    println!("{} Reserving port {} ({})", "🔒".blue(), port, protocol.to_uppercase());
    
    let mut guard = if protocol.to_lowercase() == "tcp" {
        PortGuard::bind_tcp(port)?
    } else {
        return Err(anyhow::anyhow!("UDP reservation not yet implemented"));
    };
    
    println!("{} Port {} successfully reserved", "✅".green(), port.to_string().bold().green());
    
    if let Some(cmd) = command {
        println!("{} Starting command: {}", "🚀".blue(), cmd.bold());
        
        match guard.spawn_child(&cmd) {
            Ok(child) => {
                println!("{} Command started with PID: {}", "✅".green(), 
                    child.id().to_string().bold());
                
                if keep {
                    println!("\n{} Port {} reserved with running command. Press Ctrl+C to stop.", 
                        "🔒".yellow(), port);
                    
                    // 设置 Ctrl+C 处理器
                    let port_for_handler = port;
                    ctrlc::set_handler(move || {
                        println!("\n{} Stopping command and releasing port {}...", 
                            "🛑".yellow(), port_for_handler);
                        std::process::exit(0);
                    }).expect("Error setting Ctrl-C handler");
                    
                    // 监控子进程
                    loop {
                        if !guard.is_child_running() {
                            println!("\n{} Command has exited. Port {} is still reserved.", 
                                "⚠️".yellow(), port);
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(500));
                    }
                    
                    if keep {
                        println!("{} Keeping port {} reserved. Press Ctrl+C to release.", 
                            "🔒".yellow(), port);
                        loop {
                            std::thread::sleep(std::time::Duration::from_secs(1));
                        }
                    }
                } else {
                    println!("{} Waiting for command to complete...", "⏳".blue());
                    
                    // 等待子进程完成
                    while guard.is_child_running() {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    
                    println!("{} Command completed. Port {} released.", "✅".green(), port);
                }
            }
            Err(e) => {
                println!("{} Failed to start command: {}", "❌".red(), e);
                
                if keep {
                    println!("{} Port {} is still reserved. Press Enter to release...", 
                        "🔒".yellow(), port);
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                }
            }
        }
    } else {
        if keep {
            println!("\n{} Port {} is reserved. Press Ctrl+C to release.", "🔒".yellow(), port);
            
            // 设置 Ctrl+C 处理器
            let port_for_handler = port;
            ctrlc::set_handler(move || {
                println!("\n{} Releasing port {}...", "🔓".yellow(), port_for_handler);
                std::process::exit(0);
            }).expect("Error setting Ctrl-C handler");
            
            // 保持程序运行
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        } else {
            println!("{} Press Enter to release the port...", "💡".blue());
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
        }
    }
    
    Ok(())
}

fn handle_list_command(protocol: Option<String>, listening: bool, json: bool, config: &Config, output_format: &OutputFormat) -> Result<()> {
    let mut ports = scan_ports()?;

    // 过滤协议
    if let Some(proto) = protocol {
        ports.retain(|p| p.protocol.to_lowercase().starts_with(&proto.to_lowercase()));
    }

    // 过滤监听端口
    if listening {
        ports.retain(|p| p.state == "LISTEN" || p.state == "UDP");
    }

    // 使用配置的输出格式，但命令行参数优先
    if json || matches!(output_format, OutputFormat::Json) {
        println!("{}", serde_json::to_string_pretty(&ports)?);
    } else {
        let output = format_ports(&ports, output_format)?;
         println!("{}", output);
        if config.verbose {
            println!("\n{}: {} ports found", "Total".bold(), ports.len());
        }
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