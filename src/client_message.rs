use serde::{Deserialize, Serialize};

pub const IMAGE_READY_TYPE: u8 = 128;
pub const MONITOR_CONNECTION_TYPE: u8 = 129;
pub const DEVICE_CONNECTION_TYPE: u8 = 130;

#[derive(Serialize, Deserialize)]
pub struct ClientMessage {
    pub connection_type: u8,
    pub sender_id: u64,
    pub data: String,
}

impl ClientMessage {
    pub fn new_empty() -> Self {
        Self {
            connection_type: 0,
            sender_id: 0,
            data: String::new()
        }
    }

    pub fn new(connection_type: u8, sender_id: u64, data: String) -> Self {
        Self {
            connection_type,
            sender_id,
            data
        }
    }

    pub fn connection_type(&self) -> u8 {
        self.connection_type
    }

    pub fn sender_id(&self) -> u64 {
        self.sender_id
    }
}
