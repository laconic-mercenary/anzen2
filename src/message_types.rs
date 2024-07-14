use serde::{Deserialize, Serialize};

pub const VIDEO_FRAME_TYPE: u8 = 128;
pub const CONN_STREAM_FRAME_TYPE: u8 = 129;
pub const CONN_DEVICE_FRAME_TYPE: u8 = 130;

#[derive(Serialize, Deserialize)]
pub struct DataFrame {
    pub stream_type: u8,
    pub sender_id: u64,
    pub data: String,
}

impl DataFrame {
    pub fn new_empty() -> Self {
        Self {
            stream_type: 0,
            sender_id: 0,
            data: String::new()
        }
    }

    pub fn new(stream_type: u8, sender_id: u64, data: String) -> Self {
        Self {
            stream_type,
            sender_id,
            data
        }
    }
}

impl DataFrame {
    pub fn stream_type(&self) -> u8 {
        self.stream_type
    }

    pub fn sender_id(&self) -> u64 {
        self.sender_id
    }
}
