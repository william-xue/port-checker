//! 高级端口管理功能
//! 提供智能端口分配、生命周期管理和测试支持

use anyhow::{Result, anyhow};
use std::net::{TcpListener, UdpSocket, SocketAddr};
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::time::{Duration, Instant};
use rand::Rng;

/// 端口绑定守卫，自动管理端口生命周期
pub struct PortGuard {
    port: u16,
    _listener: Option<TcpListener>,
    _socket: Option<UdpSocket>,
    child_process: Option<Child>,
}

impl PortGuard {
    /// 绑定指定端口
    pub fn bind_tcp(port: u16) -> Result<Self> {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr)
            .map_err(|e| anyhow!("Failed to bind TCP port {}: {}", port, e))?;
        
        Ok(PortGuard {
            port,
            _listener: Some(listener),
            _socket: None,
            child_process: None,
        })
    }
    
    /// 绑定UDP端口
    pub fn bind_udp(port: u16) -> Result<Self> {
        let addr = format!("127.0.0.1:{}", port);
        let socket = UdpSocket::bind(&addr)
            .map_err(|e| anyhow!("Failed to bind UDP port {}: {}", port, e))?;
        
        Ok(PortGuard {
            port,
            _listener: None,
            _socket: Some(socket),
            child_process: None,
        })
    }
    
    /// 启动子进程并绑定到端口生命周期
    /// 注意：这会释放当前的端口绑定，让子进程能够绑定该端口
    pub fn spawn_child<S: AsRef<str>>(&mut self, command: S) -> Result<&mut Child> {
        let cmd_str = command.as_ref();
        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
        
        if parts.is_empty() {
            return Err(anyhow!("Empty command"));
        }
        
        let mut cmd = Command::new(parts[0]);
        if parts.len() > 1 {
            cmd.args(&parts[1..]);
        }
        
        // 设置环境变量，让子进程知道绑定的端口
        cmd.env("BOUND_PORT", self.port.to_string());
        
        // 释放端口绑定，让子进程能够使用
        self._listener = None;
        self._socket = None;
        
        let child = cmd.spawn()
            .map_err(|e| anyhow!("Failed to spawn child process: {}", e))?;
        
        self.child_process = Some(child);
        Ok(self.child_process.as_mut().unwrap())
    }
    
    /// 获取绑定的端口号
    pub fn port(&self) -> u16 {
        self.port
    }
    
    /// 检查子进程是否还在运行
    pub fn is_child_running(&mut self) -> bool {
        if let Some(ref mut child) = self.child_process {
            match child.try_wait() {
                Ok(Some(_)) => false, // 进程已退出
                Ok(None) => true,     // 进程仍在运行
                Err(_) => false,      // 出错，假设进程已退出
            }
        } else {
            false
        }
    }
}

impl Drop for PortGuard {
    fn drop(&mut self) {
        // 终止子进程
        if let Some(mut child) = self.child_process.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        // 端口会在 TcpListener/UdpSocket drop 时自动释放
    }
}

/// 智能端口分配器
pub struct PortAllocator {
    used_ports: Arc<Mutex<HashSet<u16>>>,
    range_start: u16,
    range_end: u16,
}

impl PortAllocator {
    /// 创建新的端口分配器
    pub fn new(range_start: u16, range_end: u16) -> Self {
        Self {
            used_ports: Arc::new(Mutex::new(HashSet::new())),
            range_start,
            range_end,
        }
    }
    
    /// 在指定范围内随机分配一个可用端口
    pub fn allocate_random(&self) -> Result<PortGuard> {
        let mut rng = rand::thread_rng();
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 100;
        
        while attempts < MAX_ATTEMPTS {
            let port = rng.gen_range(self.range_start..=self.range_end);
            
            // 检查端口是否已被此分配器使用
            {
                let used_ports = self.used_ports.lock().unwrap();
                if used_ports.contains(&port) {
                    attempts += 1;
                    continue;
                }
            }
            
            // 尝试绑定端口
            match PortGuard::bind_tcp(port) {
                Ok(guard) => {
                    // 标记端口为已使用
                    {
                        let mut used_ports = self.used_ports.lock().unwrap();
                        used_ports.insert(port);
                    }
                    return Ok(guard);
                }
                Err(_) => {
                    attempts += 1;
                    continue;
                }
            }
        }
        
        Err(anyhow!("Failed to allocate port after {} attempts", MAX_ATTEMPTS))
    }
    
    /// 释放端口（通常在 PortGuard drop 时自动调用）
    pub fn release_port(&self, port: u16) {
        let mut used_ports = self.used_ports.lock().unwrap();
        used_ports.remove(&port);
    }
}

/// 测试用的模拟端口
#[cfg(test)]
pub struct MockPort {
    port: u16,
    should_fail: bool,
    bind_delay: Option<Duration>,
    fail_after: Option<Instant>,
}

#[cfg(test)]
impl MockPort {
    /// 创建一个模拟端口
    pub fn new(port: u16) -> Self {
        Self {
            port,
            should_fail: false,
            bind_delay: None,
            fail_after: None,
        }
    }
    
    /// 设置绑定应该失败
    pub fn should_fail(mut self, fail: bool) -> Self {
        self.should_fail = fail;
        self
    }
    
    /// 设置绑定延迟
    pub fn with_bind_delay(mut self, delay: Duration) -> Self {
        self.bind_delay = Some(delay);
        self
    }
    
    /// 设置在指定时间后失败
    pub fn fail_after(mut self, duration: Duration) -> Self {
        self.fail_after = Some(Instant::now() + duration);
        self
    }
    
    /// 模拟绑定操作
    pub fn bind(&self) -> Result<MockPortGuard> {
        if self.should_fail {
            return Err(anyhow!("Mock port {} is configured to fail", self.port));
        }
        
        if let Some(delay) = self.bind_delay {
            std::thread::sleep(delay);
        }
        
        Ok(MockPortGuard {
            port: self.port,
            fail_after: self.fail_after,
        })
    }
}

#[cfg(test)]
pub struct MockPortGuard {
    port: u16,
    fail_after: Option<Instant>,
}

#[cfg(test)]
impl MockPortGuard {
    pub fn port(&self) -> u16 {
        self.port
    }
    
    pub fn is_valid(&self) -> bool {
        if let Some(fail_time) = self.fail_after {
            Instant::now() < fail_time
        } else {
            true
        }
    }
}

/// 便捷函数：在指定范围内随机绑定端口
pub fn bind_random_port(range_start: u16, range_end: u16) -> Result<PortGuard> {
    let allocator = PortAllocator::new(range_start, range_end);
    allocator.allocate_random()
}

/// 便捷函数：绑定端口并启动子进程
pub fn bind_and_spawn<S: AsRef<str>>(port: u16, command: S) -> Result<PortGuard> {
    let mut guard = PortGuard::bind_tcp(port)?;
    guard.spawn_child(command)?;
    Ok(guard)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_port_guard_basic() {
        let guard = PortGuard::bind_tcp(0).unwrap(); // 0 表示系统分配
        assert!(guard.port() > 0);
    }
    
    #[test]
    fn test_port_allocator() {
        let allocator = PortAllocator::new(8000, 9000);
        let guard1 = allocator.allocate_random().unwrap();
        let guard2 = allocator.allocate_random().unwrap();
        
        assert_ne!(guard1.port(), guard2.port());
        assert!(guard1.port() >= 8000 && guard1.port() <= 9000);
        assert!(guard2.port() >= 8000 && guard2.port() <= 9000);
    }
    
    #[test]
    fn test_mock_port_success() {
        let mock = MockPort::new(8080);
        let guard = mock.bind().unwrap();
        assert_eq!(guard.port(), 8080);
        assert!(guard.is_valid());
    }
    
    #[test]
    fn test_mock_port_failure() {
        let mock = MockPort::new(8080).should_fail(true);
        assert!(mock.bind().is_err());
    }
    
    #[test]
    fn test_mock_port_delay() {
        let mock = MockPort::new(8080).with_bind_delay(Duration::from_millis(100));
        let start = Instant::now();
        let _guard = mock.bind().unwrap();
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(90)); // 允许一些误差
    }
    
    #[test]
    fn test_mock_port_fail_after() {
        let mock = MockPort::new(8080).fail_after(Duration::from_millis(100));
        let guard = mock.bind().unwrap();
        assert!(guard.is_valid());
        
        std::thread::sleep(Duration::from_millis(150));
        assert!(!guard.is_valid());
    }
}