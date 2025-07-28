use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 默认输出格式
    pub default_format: OutputFormat,
    /// 扫描超时时间（秒）
    pub scan_timeout: u64,
    /// 并发线程数
    pub concurrent_threads: usize,
    /// 是否显示详细信息
    pub verbose: bool,
    /// 是否显示进程信息
    pub show_process_info: bool,
    /// 端口分配范围
    pub port_range: PortRange,
    /// 默认保留时间（秒）
    pub default_reserve_duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
    Csv,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_format: OutputFormat::Table,
            scan_timeout: 30,
            concurrent_threads: 4,
            verbose: false,
            show_process_info: true,
            port_range: PortRange {
                start: 8000,
                end: 9000,
            },
            default_reserve_duration: 3600, // 1小时
        }
    }
}

impl Config {
    /// 从配置文件加载配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;
        Ok(config)
    }
    
    /// 保存配置到文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| anyhow!("Failed to serialize config: {}", e))?;
        fs::write(path, content)?;
        Ok(())
    }
    
    /// 从默认位置加载配置
    pub fn load_default() -> Result<Self> {
        let config_paths = [
            "./port-checker.toml",
            "~/.config/port-checker/config.toml",
            "~/.port-checker.toml",
        ];
        
        for path in &config_paths {
            let expanded_path = expand_path(path);
            if expanded_path.exists() {
                return Self::load_from_file(&expanded_path);
            }
        }
        
        // 如果没有找到配置文件，返回默认配置
        Ok(Self::default())
    }
    
    /// 创建默认配置文件
    pub fn create_default_config() -> Result<()> {
        let config = Self::default();
        let config_path = expand_path("~/.config/port-checker/config.toml");
        
        // 创建目录
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        config.save_to_file(&config_path)?;
        println!("✅ Default config created at: {}", config_path.display());
        Ok(())
    }
}

/// 展开路径中的 ~ 符号
fn expand_path(path: &str) -> std::path::PathBuf {
    if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    std::path::PathBuf::from(path)
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Table => write!(f, "table"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Csv => write!(f, "csv"),
        }
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "table" => Ok(OutputFormat::Table),
            "json" => Ok(OutputFormat::Json),
            "yaml" | "yml" => Ok(OutputFormat::Yaml),
            "csv" => Ok(OutputFormat::Csv),
            _ => Err(anyhow!("Invalid output format: {}", s)),
        }
    }
}