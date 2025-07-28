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
- 🎯 **智能端口分配**：自动查找并分配可用端口
- 🔒 **端口预留**：预留端口并绑定到进程生命周期
- 🔗 **进程集成**：使用预分配端口运行命令

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

### 智能端口分配

```bash
# 在指定范围内分配一个随机可用端口
port-checker pick --start 8000 --end 9000

# 保持端口预留状态（不立即退出）
port-checker pick --start 8080 --end 8090 --keep
```

### 预留特定端口

```bash
# 临时预留一个端口
port-checker reserve 8888

# 预留端口并运行命令
port-checker reserve 8888 --command "python3 -m http.server"

# 命令退出后保持端口预留
port-checker reserve 8888 --command "node server.js" --keep
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

```
$ port-checker pick --start 8080 --end 8090
🎲 在范围 8080..8090 内分配随机端口
✅ 成功分配端口：8085
  地址：127.0.0.1:8085
  协议：TCP

⚡ 端口 8085 分配成功。请在程序退出前快速使用！
💡 按 Enter 键释放端口...
```

```
$ port-checker reserve 8888 --command "python3 -m http.server"
🔒 预留端口 8888 (TCP)
✅ 端口 8888 预留成功
🚀 启动命令：python3 -m http.server
✅ 命令已启动，PID：12345
⏳ 等待命令完成...
Serving HTTP on 0.0.0.0 port 8888 (http://0.0.0.0:8888/) ...
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

### `port-checker pick [OPTIONS]`

在指定范围内智能分配一个可用端口。

**选项:**
- `-s, --start <PORT>`: 端口范围起始值 (默认: 8000)
- `-e, --end <PORT>`: 端口范围结束值 (默认: 9000)
- `-p, --protocol <PROTOCOL>`: 协议类型 (tcp/udp，默认: tcp)
- `-k, --keep`: 保持端口预留状态，不立即退出

### `port-checker reserve <PORT> [OPTIONS]`

预留指定端口并可选择运行命令。

**参数:**
- `<PORT>`: 要预留的端口号

**选项:**
- `-p, --protocol <PROTOCOL>`: 协议类型 (tcp/udp，默认: tcp)
- `-c, --command <COMMAND>`: 使用预留端口运行的命令
- `-k, --keep`: 命令退出后保持端口预留状态

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

## 🚀 发展路线图 - 即将推出的功能

我们正在积极开发以下高级功能，让 Port Checker 成为更强大的网络诊断工具：

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

## 支持

如果您遇到任何问题或有任何疑问，请在 GitHub 上 [提出一个 issue](https://github.com/william-xue/port-checker/issues)。