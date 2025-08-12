use super::document::QRSIZE;

use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};

use anyhow::Result;
use fast_qr::convert::image::ImageBuilder;
use fast_qr::convert::Builder;
use fast_qr::QRBuilder;
use image::{DynamicImage, ImageReader};

const CHUNK_SIZE: usize = 512;

pub fn generate_qrcode(data: &[u8]) -> Result<DynamicImage> {
    let qr = QRBuilder::new(data.to_vec())
        .mode(fast_qr::Mode::Byte)
        .ecl(fast_qr::ECL::L)
        .build()?;

    // 77 is the width of a 512 bytes qrcode
    let bytes = ImageBuilder::default()
        .margin(0)
        .fit_width((qr.size * QRSIZE as usize / 77) as u32)
        .to_bytes(&qr)?;

    Ok(ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?)
}

pub fn generate_qrcodes(file: &mut File) -> Result<Vec<DynamicImage>> {
    let file_size = file.metadata()?.len() as usize;
    let mut data = Vec::new();

    let full_chunks = file_size / CHUNK_SIZE;
    let remainder = file_size % CHUNK_SIZE;

    let mut buffer = vec![0u8; CHUNK_SIZE];

    for i in 0..full_chunks {
        file.seek(SeekFrom::Start((i * CHUNK_SIZE) as u64))?;
        file.read_exact(&mut buffer)?;

        let qrcode = generate_qrcode(&buffer)?;
        data.push(qrcode);
    }

    if remainder != 0 {
        let mut buffer = vec![0u8; remainder];
        file.seek(SeekFrom::Start((full_chunks * CHUNK_SIZE) as u64))?;
        file.read_exact(&mut buffer)?;

        let qrcode = generate_qrcode(&buffer)?;
        data.push(qrcode);
    }

    Ok(data)
}
