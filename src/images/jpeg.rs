extern crate derive_more;
use derive_more::Display;

const APP1_MARKER: [u8; 2] = [0xFF, 0xE1];

enum ExtractionPhase {
    None,
    Preamble,
    AppSection,
}

#[derive(Display)]
pub enum JpegErr {
    #[display(fmt = "APP section is missing or is invalid")]
    InvalidAppSection
}

pub fn is_jpeg(image_data: &[u8]) -> bool {
    if image_data.len() < 4 {
        return false;
    }
    
    image_data[0] == 0xFF && 
        image_data[1] == 0xD8 && 
        image_data[image_data.len() - 2] == 0xFF && 
        image_data[image_data.len() - 1] == 0xD9
}


pub fn extract_app_section(jpeg_data: &[u8]) -> Result<Vec<u8>, JpegErr> {
    let mut app_section_length = Vec::new();
    let mut app_section_data = Vec::new();
    let mut tracker = ExtractionPhase::None;
    let mut jpeg_data_iter = jpeg_data.iter();

    while let Some(byte) = jpeg_data_iter.next() {
        if let ExtractionPhase::None = tracker {
            if byte == &APP1_MARKER[0] {
                tracker = ExtractionPhase::Preamble;
            }
        } else if let ExtractionPhase::Preamble = tracker {
            if byte == &APP1_MARKER[1] {
                tracker = ExtractionPhase::AppSection;
            } else {
                tracker = ExtractionPhase::None;
            }
        } else if let ExtractionPhase::AppSection = tracker {
            if app_section_length.len() < 2 {
                app_section_length.push(*byte);
            } else {
                let app_section_len = ((app_section_length[0] as usize) << 8) | (app_section_length[1] as usize);
                if app_section_data.len() < app_section_len {
                    app_section_data.push(*byte);    
                } else {
                    return Ok(app_section_data);
                }
            }
        }
    }
    Err(JpegErr::InvalidAppSection)
}