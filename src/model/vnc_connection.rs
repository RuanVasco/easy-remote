use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct VncConnection {
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "@ip")]
    pub ip: String,
    #[serde(rename = "@port")]
    pub port: String,
}

impl VncConnection {
    pub fn connect(&self) {
        println!("Conectando em {}:{}", self.ip, self.port);
    }
}
