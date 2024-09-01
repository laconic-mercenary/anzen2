use std::num::ParseIntError;

const SENDER_ID_LENGTH: usize = 7;

pub fn extract_sender_id(image_data: &Vec<u8>) -> Option<u64> {
    let sender_id_bytes: Vec<u8> = image_data.iter()
                                            .rev()
                                            .take(SENDER_ID_LENGTH)
                                            .cloned()
                                            .collect();
    match bytes_to_integer(&sender_id_bytes) {
        Ok(sender_id) => { 
            return Some(sender_id)
        },
        Err(err) => { 
            log::error!("failed to parse sender id: {:?}", err);
            return None
        }
    };
}

pub fn remove_sender_id(image_data: &mut Vec<u8>) {
    image_data.truncate(image_data.len() - SENDER_ID_LENGTH);
}

fn bytes_to_integer(bytes: &[u8]) -> Result<u64, ParseIntError> {
    let s = bytes.iter().map(|&b| (b + b'0') as char).rev().collect::<String>();
    s.parse::<u64>()
}