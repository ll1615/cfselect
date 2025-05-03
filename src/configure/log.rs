use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub file: FileConfig,
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FileConfig {
    pub enabled: bool,
    pub dir: String,
    pub name_prefix: String,
}