use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::RwLock;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: RwLock<Option<Config>> = RwLock::new(None);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub search: SearchConfig,
    pub storage: StorageConfig,
    pub performance: PerformanceConfig,
    pub features: FeaturesConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchConfig {
    pub max_results: usize,
    pub enable_fuzzy: bool,
    pub enable_vector: bool,
    pub min_score: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageConfig {
    pub path: String,
    pub cache_size_mb: usize,
    pub persist_interval_secs: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceConfig {
    pub worker_threads: usize,
    pub max_concurrent_searches: usize,
    pub batch_size: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeaturesConfig {
    pub enable_metrics: bool,
    pub enable_logging: bool,
    pub enable_analytics: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            search: SearchConfig {
                max_results: 100,
                enable_fuzzy: true,
                enable_vector: true,
                min_score: 0.1,
            },
            storage: StorageConfig {
                path: "./data".to_string(),
                cache_size_mb: 1024,
                persist_interval_secs: 60,
            },
            performance: PerformanceConfig {
                worker_threads: num_cpus::get(),
                max_concurrent_searches: 100,
                batch_size: 1000,
            },
            features: FeaturesConfig {
                enable_metrics: true,
                enable_logging: true,
                enable_analytics: false,
            },
        }
    }
}

pub fn init_config(path: Option<&str>) -> Result<(), String> {
    let config = match path {
        Some(p) => load_config(p)?,
        None => Config::default(),
    };
    
    if let Some(mut config_lock) = CONFIG.write().ok() {
        *config_lock = Some(config);
        Ok(())
    } else {
        Err("Failed to acquire write lock for config".to_string())
    }
}

pub fn get_config() -> Option<Config> {
    if let Ok(guard) = CONFIG.read() {
        guard.clone()
    } else {
        None
    }
}

fn load_config(path: &str) -> Result<Config, String> {
    let config_str = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    
    let config: Config = toml::from_str(&config_str)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    Ok(config)
}

pub fn save_config(path: &str, config: &Config) -> Result<(), String> {
    let config_str = toml::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    
    fs::write(path, config_str)
        .map_err(|e| format!("Failed to write config file: {}", e))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let config_str = toml::to_string_pretty(&config).unwrap();
        let _: Config = toml::from_str(&config_str).unwrap();
    }
    
    #[test]
    fn test_config_file_io() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let config = Config::default();
        
        save_config(config_path.to_str().unwrap(), &config).unwrap();
        let loaded = load_config(config_path.to_str().unwrap()).unwrap();
        
        assert_eq!(config.search.max_results, loaded.search.max_results);
    }
}
