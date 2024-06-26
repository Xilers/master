use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSpec {
    pub id: String,
    pub os: String,
    pub os_version: String,
    pub core_num: u8,
    pub ip_addr: String,
    pub port: u16,
    pub status: String,
    pub updated_at: String,
}
