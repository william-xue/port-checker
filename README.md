# Port Checker 🔍

A fast and user-friendly command-line tool to check port usage on your system. Built with Rust for maximum performance and reliability.

## Features

- 🚀 **Fast**: Blazingly fast port scanning with minimal system overhead
- 🎨 **Beautiful Output**: Colorized and well-formatted terminal output
- 🔍 **Comprehensive**: Check TCP, UDP, IPv4, and IPv6 connections
- 📊 **Statistics**: View detailed port usage statistics
- 🔎 **Process Detection**: Find which process is using a specific port
- 📋 **Multiple Formats**: Output in table, JSON, YAML, or CSV format
- ⚙️ **Configuration File**: Customizable settings with TOML configuration
- 🌐 **Cross-Platform**: Works on Linux, macOS, and Windows
- 🚀 **Concurrent Scanning**: High-performance parallel port scanning
- 🎯 **Smart Port Allocation**: Automatically find and allocate available ports
- 🔒 **Port Reservation**: Reserve ports and bind them to process lifecycles
- 🔗 **Process Integration**: Run commands with pre-allocated ports

## Installation

### From crates.io (Recommended)

```bash
cargo install port-checker
```

### From source

```bash
git clone https://github.com/william-xue/port-checker.git
cd port-checker
cargo install --path .
```

### Pre-built binaries

Download the latest release from the [releases page](https://github.com/william-xue/port-checker/releases).

## Usage

### List all occupied ports

```bash
# List all ports
port-checker list

# List only TCP ports
port-checker list --protocol tcp

# List only listening ports
port-checker list --listening

# Output in different formats
port-checker list --format json
port-checker list --format yaml
port-checker list --format csv
port-checker list --format table  # default
```

### Check if a specific port is in use

```bash
# Check port 8080 (both TCP and UDP)
port-checker check 8080

# Check port 8080 TCP only
port-checker check 8080 --protocol tcp
```

### Find which process is using a port

```bash
# Find process using TCP port 8080
port-checker find 8080

# Find process using UDP port 53
port-checker find 53 --protocol udp
```

### Kill a process using a port

```bash
# Kill process using TCP port 8080
port-checker kill 8080

# Kill process using UDP port 53
port-checker kill 53 --protocol udp
```

### Show port usage statistics

```bash
port-checker stats
```

### Smart port allocation

```bash
# Allocate a random available port in range
port-checker pick --start 8000 --end 9000

# Keep the port reserved (don't exit immediately)
port-checker pick --start 8080 --end 8090 --keep
```

### Reserve a specific port

```bash
# Reserve a port temporarily
port-checker reserve 8888

# Reserve a port and run a command with it
port-checker reserve 8888 --command "python3 -m http.server"

# Keep the port reserved after command exits
port-checker reserve 8888 --command "node server.js" --keep
```

### Configuration Management

```bash
# Show current configuration
port-checker config show

# Initialize default configuration file
port-checker config init

# Use custom configuration file
port-checker list --config /path/to/config.toml

# Override settings with command line options
port-checker list --format yaml --verbose
```

## Examples

### Example Output

```
$ port-checker list --listening
┌──────────┬─────────────────┬─────────────────┬─────────┬──────┬──────────────┐
│ Protocol │ Local Address   │ Remote Address  │ State   │ PID  │ Process Name │
├──────────┼─────────────────┼─────────────────┼─────────┼──────┼──────────────┤
│ TCP      │ 0.0.0.0:22      │ -               │ LISTEN  │ 1234 │ sshd         │
│ TCP      │ 127.0.0.1:5432  │ -               │ LISTEN  │ 5678 │ postgres     │
│ TCP      │ 0.0.0.0:80      │ -               │ LISTEN  │ 9012 │ nginx        │
│ TCP      │ 0.0.0.0:443     │ -               │ LISTEN  │ 9012 │ nginx        │
└──────────┴─────────────────┴─────────────────┴─────────┴──────┴──────────────┘

Total: 4 ports found
```

```
$ port-checker check 8080
🟢 Port 8080 (TCP) is FREE
🟢 Port 8080 (UDP) is FREE
```

```
$ port-checker find 22
🔍 Process using port 22 (TCP):

  Process ID: 1234
  Process Name: sshd
  Local Address: 0.0.0.0:22
  State: LISTEN
```

```
$ port-checker pick --start 8080 --end 8090
🎲 Allocating random port in range 8080..8090
✅ Successfully allocated port: 8085
  Address: 127.0.0.1:8085
  Protocol: TCP

⚡ Port 8085 allocated successfully. Use it quickly before this program exits!
💡 Press Enter to release the port...
```

```
$ port-checker reserve 8888 --command "python3 -m http.server"
🔒 Reserving port 8888 (TCP)
✅ Port 8888 successfully reserved
🚀 Starting command: python3 -m http.server
✅ Command started with PID: 12345
⏳ Waiting for command to complete...
Serving HTTP on 0.0.0.0 port 8888 (http://0.0.0.0:8888/) ...
```

## Command Reference

### Global Options

These options are available for all commands:

- `-f, --format <FORMAT>`: Output format (table, json, yaml, csv)
- `-c, --config <FILE>`: Use custom configuration file
- `-v, --verbose`: Enable verbose output

### `port-checker list [OPTIONS]`

List all occupied ports on the system.

**Options:**
- `-p, --protocol <PROTOCOL>`: Filter by protocol (tcp/udp)
- `-l, --listening`: Show only listening ports

### `port-checker check <PORT> [OPTIONS]`

Check if a specific port is in use.

**Arguments:**
- `<PORT>`: Port number to check

**Options:**
- `-p, --protocol <PROTOCOL>`: Protocol to check (tcp/udp)

### `port-checker find <PORT> [OPTIONS]`

Find the process using a specific port.

**Arguments:**
- `<PORT>`: Port number to find

**Options:**
- `-p, --protocol <PROTOCOL>`: Protocol type (default: tcp)

### `port-checker kill <PORT> [OPTIONS]`

Kill the process using a specific port.

**Arguments:**
- `<PORT>`: Port number to find the process to kill

**Options:**
- `-p, --protocol <PROTOCOL>`: Protocol type (default: tcp)

### `port-checker stats`

Display port usage statistics.

### `port-checker pick [OPTIONS]`

Allocate a random available port in specified range.

**Options:**
- `-s, --start <START>`: Start of port range (default: 8000)
- `-e, --end <END>`: End of port range (default: 9000)
- `-k, --keep`: Keep the port reserved (don't exit immediately)

### `port-checker reserve <PORT> [OPTIONS]`

Reserve a specific port and optionally run a command.

**Arguments:**
- `<PORT>`: Port number to reserve

**Options:**
- `-p, --protocol <PROTOCOL>`: Protocol (tcp/udp) (default: tcp)
- `-c, --command <COMMAND>`: Command to run with the reserved port
- `-k, --keep`: Keep the port reserved after command exits

### `port-checker config <ACTION>`

Manage configuration settings.

**Actions:**
- `show`: Display current configuration
- `init`: Create default configuration file at `~/.config/port-checker/config.toml`
- `set`: Set configuration values (planned feature)

## Configuration File

Port Checker supports configuration files in TOML format. The default location is `~/.config/port-checker/config.toml`.

### Example Configuration

```toml
# Default output format
default_format = "Table"  # Table, Json, Yaml, Csv

# Scan timeout in seconds
scan_timeout = 30

# Number of concurrent threads for scanning
concurrent_threads = 4

# Enable verbose output by default
verbose = false

# Show process information
show_process_info = true

# Default reserve duration in seconds
default_reserve_duration = 3600  # 1 hour

# Port allocation range
[port_range]
start = 8000
end = 9000
```

### Configuration Priority

1. Command-line arguments (highest priority)
2. Custom config file specified with `--config`
3. Default config file (`~/.config/port-checker/config.toml`)
4. Built-in defaults (lowest priority)

## Requirements

- **Linux**: No additional requirements
- **macOS**: No additional requirements  
- **Windows**: Requires `netstat` and `tasklist` commands (included by default)

## Performance

Port Checker is designed to be fast and lightweight:

- ⚡ Scans thousands of ports in milliseconds
- 💾 Minimal memory usage
- 🔋 Low CPU overhead
- 📊 Efficient data structures

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- Built with [clap](https://github.com/clap-rs/clap) for command-line parsing
- Uses [colored](https://github.com/mackwic/colored) for terminal colors
- Table formatting with [tabled](https://github.com/zhiburt/tabled)

## 🚀 Roadmap

### ✅ Completed Features

- [x] Configuration file support (TOML)
- [x] Multiple output formats (Table, JSON, YAML, CSV)
- [x] Concurrent port scanning
- [x] Cross-platform support
- [x] Process information detection
- [x] Port reservation system
- [x] Smart port allocation
- [x] Process integration

### 🚧 Planned Features

我们正在积极开发以下高级功能，让 Port Checker 成为更强大的云原生端口治理平台：

#### 🎯 端口调度器 (Scheduler) - v0.4.0
```bash
# K8s Dynamic Admission Webhook 集成
port-checker scheduler --webhook --port 9443

# Pod 创建时自动注入空闲端口
port-checker scheduler --auto-inject --namespace default

# 冲突实时重试机制
port-checker scheduler --retry-on-conflict --max-retries 3
```

#### 🔒 安全策略引擎 - v0.5.0
```bash
# 端口白名单管理
port-checker security --whitelist-add 80,443,8080
port-checker security --whitelist-remove 3000

# 基于身份的 ACL 控制
port-checker security --acl-user john --allow 8080-8090
port-checker security --acl-group developers --deny 22

# 异常流量自动封禁
port-checker security --auto-ban --threshold 100req/s
port-checker security --ban-duration 1h
```

#### 📊 观测与审计平台 - v0.6.0
```bash
# 端口使用热力图
port-checker observe --heatmap --time-range 24h
port-checker observe --heatmap --export png

# 泄露检测与告警
port-checker observe --leak-detection --alert-webhook https://hooks.slack.com/...
port-checker observe --scan-external --report

# 一键回收闲置端口
port-checker observe --cleanup --idle-time 7d
port-checker observe --cleanup --dry-run
```

#### ⚡ 高性能扫描
```bash
# 网络接口过滤
port-checker scan --interface eth0 --ports 1-65535
port-checker scan --interface wlan0 --protocol tcp

# SYN 扫描支持
port-checker scan --syn-scan 192.168.1.0/24
port-checker scan --stealth-mode --timeout 1s

# 扫描网段的端口范围
port-checker scan 192.168.1.0/24 --ports 1-1000

# 使用预设扫描常见服务端口
port-checker scan --preset web
port-checker scan --preset database
```

#### 🔍 服务识别与指纹识别
```bash
# Banner 抓取与服务版本检测
port-checker identify 80 --banner-grab
port-checker identify 22 --service-version

# 安全风险评估
port-checker identify --security-scan --cve-check
port-checker identify --risk-report --format pdf

# 健康检查预设服务
port-checker health-check --preset database
port-checker health-check --custom-script /path/to/check.sh
```

#### 📊 监控模式
```bash
# 持续监控指定端口
port-checker monitor --watch 80,443 --alert
port-checker monitor --connection-stats --interval 5s

# 实时仪表板
port-checker dashboard --refresh 1s
port-checker dashboard --web-ui --port 9090

# 历史数据记录
port-checker monitor --history --retention 30d
port-checker monitor --export-metrics prometheus
```

#### 🌐 集成与导出
```bash
# API 接口服务
port-checker api --server --port 8080
port-checker api --client --endpoint http://localhost:8080

# 多格式报告导出
port-checker export --format xml --output report.xml
port-checker export --format html --template custom.html

# 第三方工具集成
port-checker integrate --grafana --datasource
port-checker integrate --prometheus --metrics-endpoint
```

#### 📋 预设配置
- **web**: 80, 443, 8080, 8443, 3000, 5000
- **database**: 3306, 5432, 27017, 6379
- **development**: 3000, 8000, 8080, 9000
- **security**: 22, 443, 993, 995
- **kubernetes**: 6443, 2379, 2380, 10250, 10251, 10252
- **microservices**: 8080-8090, 9090-9099

### 🎯 发展愿景

**短期目标 (v0.4.0 - v0.6.0)**：
- 实现云原生端口调度器
- 构建企业级安全策略引擎
- 开发全面的观测与审计平台

**长期目标 (v1.0.0+)**：
- 成为 Kubernetes 生态的标准端口治理组件
- 提供完整的企业级端口安全解决方案
- 建立活跃的开源社区和商业生态

这些功能将让 Port Checker 从简单的端口查看工具发展为专业的云原生端口治理平台！

## Support

If you encounter any issues or have questions, please [open an issue](https://github.com/william-xue/port-checker/issues) on GitHub.