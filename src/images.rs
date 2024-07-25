pub mod jpeg;
pub mod app;

pub enum ImageType {
    JPEG
}

pub fn get_sender_id(image_data: &[u8]) -> Option<u64> {
    // TODO: add more image types
    let image_type = if jpeg::is_jpeg(image_data) {
        Some(ImageType::JPEG)
    } else {
        None
    };
    match image_type {
        Some(ImageType::JPEG) => {
            jpeg::extract_app_section(image_data)
                .map(|app_section_data| app::extract_sender_id(&app_section_data))
                .unwrap_or_else(|err| {
                    log::error!("invalid app section in image: {}", err);
                    None
                })
        },
        _ => {
            log::error!("unsupported image type");
            None
        }
    }
}