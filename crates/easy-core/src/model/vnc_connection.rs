use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct VncConnection {
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "@ip")]
    pub ip: String,
    #[serde(rename = "@port")]
    pub port: u16,
}

impl VncConnection {
    pub fn address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl fmt::Display for VncConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{name} ({ip})", name = self.label, ip = self.ip)
    }
}
