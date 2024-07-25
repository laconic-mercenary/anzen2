
const SENDER_ID_START: [u8; 2] = [0x07, 0x0C];
const SENDER_ID_LENGTH: u8 = 7;

pub fn extract_sender_id(app_section_data: &[u8]) -> Option<u64> {
    let mut sender_id_bytes = Vec::new();
    let mut app_section_data_iter = app_section_data.iter();
    while let Some(byte) = app_section_data_iter.next() {
        if byte == &SENDER_ID_START[0] {
            if let Some(next_byte) = app_section_data_iter.next() {
                if next_byte == &SENDER_ID_START[1] {
                    for _ in 0..SENDER_ID_LENGTH {
                        if let Some(next_byte) = app_section_data_iter.next() {
                            sender_id_bytes.push(*next_byte);
                        } else {
                            return None;
                        }
                    }
                    let sender_id = sender_id_bytes.iter().fold(0u64, |acc, &byte| {
                        acc * 10 + (byte as u64)
                    });
                    return Some(sender_id);                
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_sender_id_valid() {
        let app_section_data = vec![0x07, 0x0C, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(extract_sender_id(&app_section_data), Some(1234567));
    }

    #[test]
    fn test_extract_sender_id_invalid_start() {
        let app_section_data = vec![0x07, 0x0D, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(extract_sender_id(&app_section_data), None);
    }

    #[test]
    fn test_extract_sender_id_incomplete() {
        let app_section_data = vec![0x07, 0x0C, 1, 2, 3, 4, 5];
        assert_eq!(extract_sender_id(&app_section_data), None);
    }

    #[test]
    fn test_extract_sender_id_empty() {
        let app_section_data = vec![];
        assert_eq!(extract_sender_id(&app_section_data), None);
    }

    #[test]
    fn test_extract_sender_id_no_start_sequence() {
        let app_section_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(extract_sender_id(&app_section_data), None);
    }
}
