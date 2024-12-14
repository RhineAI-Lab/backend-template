use config::{ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // 使用 ConfigBuilder 构建配置
        let builder = config::Config::builder()
            .add_source(Environment::default()); // 添加环境变量作为配置源
        let cfg = builder.build()?; // 构建 Config 对象
        cfg.try_deserialize() // 尝试将配置反序列化为结构体
    }
}
