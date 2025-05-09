use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct NamesiloConfig {
    pub url: String,
    pub key: String,
    pub domain: String,
    pub rrhost: String,
    pub rrttl: String,
}
