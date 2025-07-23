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

## Support

If you encounter any issues or have questions, please [open an issue](https://github.com/william-xue/port-checker/issues) on GitHub.