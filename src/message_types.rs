use serde::{Deserialize, Serialize};

pub const VIDEO_FRAME_TYPE: u8 = 128;
pub const CONN_FRAME_TYPE: u8 = 129;

#[derive(Serialize, Deserialize)]
pub struct DataFrame {
    pub stream_type: u8,
    pub stream_id: u64,
    pub data: String,
}

impl DataFrame {
    pub fn new_empty() -> Self {
        Self {
            stream_type: 0,
            stream_id: 0,
            data: String::new()
        }
    }

    pub fn new(stream_type: u8, stream_id: u64, data: String) -> Self {
        Self {
            stream_type,
            stream_id,
            data
        }
    }
}

impl DataFrame {
    pub fn stream_type(&self) -> u8 {
        self.stream_type
    }

    pub fn stream_id(&self) -> u64 {
        self.stream_id
    }
}
