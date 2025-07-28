# Port Checker 🔍

A fast and user-friendly command-line tool to check port usage on your system. Built with Rust for maximum performance and reliability.

## Features

- 🚀 **Fast**: Blazingly fast port scanning with minimal system overhead
- 🎨 **Beautiful Output**: Colorized and well-formatted terminal output
- 🔍 **Comprehensive**: Check TCP, UDP, IPv4, and IPv6 connections
- 📊 **Statistics**: View detailed port usage statistics
- 🔎 **Process Detection**: Find which process is using a specific port
- 📋 **Multiple Formats**: Output in table or JSON format
- 🌐 **Cross-Platform**: Works on Linux, macOS, and Windows
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

# Output in JSON format
port-checker list --json
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

### `port-checker list [OPTIONS]`

List all occupied ports on the system.

**Options:**
- `-p, --protocol <PROTOCOL>`: Filter by protocol (tcp/udp)
- `-l, --listening`: Show only listening ports
- `-j, --json`: Output in JSON format

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

## 🚀 Roadmap - Upcoming Features

我们正在积极开发以下高级功能，让 Port Checker 成为更强大的网络诊断工具：

### 🎯 智能端口分配
```bash
# 自动选择可用端口
port-checker pick --range 8000-9000

# 预留端口一段时间
port-checker reserve 8080 --duration 1h
```

### ⚡ 高性能扫描
```bash
# 扫描网段的端口范围
port-checker scan 192.168.1.0/24 --ports 1-1000

# 使用预设扫描常见服务端口
port-checker scan --preset web
port-checker scan --preset database
```

### 📊 监控模式
```bash
# 持续监控指定端口
port-checker monitor --watch 80,443 --alert

# 实时仪表板
port-checker dashboard --refresh 1s
```

### 🔍 服务识别
```bash
# 识别端口上运行的服务
port-checker identify 80 --fingerprint

# 健康检查预设服务
port-checker health-check --preset database
```

### 📋 预设配置
- **web**: 80, 443, 8080, 8443, 3000, 5000
- **database**: 3306, 5432, 27017, 6379
- **development**: 3000, 8000, 8080, 9000
- **security**: 22, 443, 993, 995

这些功能将在后续版本中逐步发布，敬请期待！

## Support

If you encounter any issues or have questions, please [open an issue](https://github.com/william-xue/port-checker/issues) on GitHub.