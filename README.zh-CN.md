# Port Checker 🔍

一个快速且用户友好的命令行工具，用于检查系统上的端口使用情况。使用 Rust 构建，以实现最佳性能和可靠性。

## 特性

- 🚀 **快速**：以最小的系统开销实现极速的端口扫描
- 🎨 **美观的输出**：色彩丰富且格式良好的终端输出
- 🔍 **全面**：检查 TCP、UDP、IPv4 和 IPv6 连接
- 📊 **统计信息**：查看详细的端口使用统计
- 🔎 **进程检测**：查找正在使用特定端口的进程
- 🔪 **进程终止**：终止正在使用特定端口的进程
- 📋 **多种格式**：以表格或 JSON 格式输出
- 🌐 **跨平台**：适用于 Linux、macOS 和 Windows

## 安装

### 从 crates.io (推荐)

```bash
cargo install port-checker
```

### 从源码

```bash
git clone https://github.com/william-xue/port-checker.git
cd port-checker
cargo install --path .
```

### 预编译二进制文件

从 [发布页面](https://github.com/william-xue/port-checker/releases) 下载最新的版本。

## 使用方法

### 列出所有占用的端口

```bash
# 列出所有端口
port-checker list

# 只列出 TCP 端口
port-checker list --protocol tcp

# 只列出监听中的端口
port-checker list --listening

# 以 JSON 格式输出
port-checker list --json
```

### 检查特定端口是否被占用

```bash
# 检查端口 8080 (TCP 和 UDP)
port-checker check 8080

# 只检查 TCP 端口 8080
port-checker check 8080 --protocol tcp
```

### 查找哪个进程正在使用某个端口

```bash
# 查找使用 TCP 端口 8080 的进程
port-checker find 8080

# 查找使用 UDP 端口 53 的进程
port-checker find 53 --protocol udp
```

### 终止使用某个端口的进程

```bash
# 终止使用 TCP 端口 8080 的进程
port-checker kill 8080

# 终止使用 UDP 端口 53 的进程
port-checker kill 53 --protocol udp
```

### 显示端口使用统计信息

```bash
port-checker stats
```

## 示例

### 示例输出

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

## 命令参考

### `port-checker list [OPTIONS]`

列出系统上所有被占用的端口。

**选项:**
- `-p, --protocol <PROTOCOL>`: 按协议 (tcp/udp) 过滤
- `-l, --listening`: 只显示监听中的端口
- `-j, --json`: 以 JSON 格式输出

### `port-checker check <PORT> [OPTIONS]`

检查特定端口是否被占用。

**参数:**
- `<PORT>`: 要检查的端口号

**选项:**
- `-p, --protocol <PROTOCOL>`: 要检查的协议 (tcp/udp)

### `port-checker find <PORT> [OPTIONS]`

查找使用特定端口的进程。

**参数:**
- `<PORT>`: 要查找的端口号

**选项:**
- `-p, --protocol <PROTOCOL>`: 协议类型 (默认为 tcp)

### `port-checker kill <PORT> [OPTIONS]`

终止使用特定端口的进程。

**参数:**
- `<PORT>`: 要查找并终止进程的端口号

**选项:**
- `-p, --protocol <PROTOCOL>`: 协议类型 (默认为 tcp)

### `port-checker stats`

显示端口使用统计信息。

## 要求

- **Linux**: 无额外要求
- **macOS**: 无额外要求
- **Windows**: 需要 `netstat` 和 `tasklist` 命令 (默认包含)

## 性能

Port Checker 设计为快速和轻量级：

- ⚡ 在几毫秒内扫描数千个端口
- 💾 最小的内存使用
- 🔋 低 CPU 开销
- 📊 高效的数据结构

## 贡献

欢迎贡献！请随时提交拉取请求。

1. Fork 本项目
2. 创建您的功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开一个拉取请求

## 许可证

本项目根据 MIT 许可证授权 - 详情请参阅 [LICENSE.md](LICENSE.md) 文件。

## 致谢

- 使用 [clap](https://github.com/clap-rs/clap) 进行命令行解析
- 使用 [colored](https://github.com/mackwic/colored) 实现终端颜色
- 使用 [tabled](https://github.com/zhiburt/tabled) 进行表格格式化

## 支持

如果您遇到任何问题或有任何疑问，请在 GitHub 上 [提出一个 issue](https://github.com/william-xue/port-checker/issues)。