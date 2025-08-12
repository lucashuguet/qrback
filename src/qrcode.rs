use crate::Variant;

use super::document::QRSIZE;

use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};

use anyhow::Result;
use fast_qr::convert::image::ImageBuilder;
use fast_qr::convert::{Builder, Color};
use fast_qr::QRBuilder;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, GenericImageView, ImageReader, Rgba};

const CHUNK_SIZE: usize = 512;
// const CHUNK_SIZE: usize = 2 * 512;
// const CHUNK_SIZE: usize = 3 * 512;

fn generate_4color_qrcode(data: &[u8]) -> Result<DynamicImage> {
    if data.len() < 2 * 512 {
        return Ok(DynamicImage::new_rgb8(QRSIZE as u32, QRSIZE as u32));
    }

    let mut qr1 = generate_qrcode(&data[0..512], "#0000ff".into())?; // blue
    let qr2 = generate_qrcode(&data[512..1024], "#ffff00".into())?; // yellow

    for i in 0..qr1.width() {
        for j in 0..qr1.width() {
            let p1 = qr1.get_pixel(i, j);
            let p2 = qr2.get_pixel(i, j);

            let r = (p1[0] as u16 * p2[0] as u16) / 255;
            let g = (p1[1] as u16 * p2[1] as u16) / 255;
            let b = (p1[2] as u16 * p2[2] as u16) / 255;

            let pixel = Rgba([r as u8, g as u8, b as u8, 255]);

            qr1.put_pixel(i, j, pixel);
        }
    }

    Ok(qr1)
}

fn generate_8color_qrcode(data: &[u8]) -> Result<DynamicImage> {
    if data.len() < 3 * 512 {
        return Ok(DynamicImage::new_rgb8(QRSIZE as u32, QRSIZE as u32));
    }

    let mut qr1 = generate_qrcode(&data[0..512], "#00ffff".into())?; // cyan
    let qr2 = generate_qrcode(&data[512..1024], "#ff00ff".into())?; // magenta
    let qr3 = generate_qrcode(&data[1024..1536], "#ffff00".into())?; // yellow

    for i in 0..qr1.width() {
        for j in 0..qr1.width() {
            let p1 = qr1.get_pixel(i, j);
            let p2 = qr2.get_pixel(i, j);
            let p3 = qr3.get_pixel(i, j);

            let r = (p1[0] as u32 * p2[0] as u32 * p3[0] as u32) / (255 * 255);
            let g = (p1[1] as u32 * p2[1] as u32 * p3[1] as u32) / (255 * 255);
            let b = (p1[2] as u32 * p2[2] as u32 * p3[2] as u32) / (255 * 255);

            let pixel = Rgba([r as u8, g as u8, b as u8, 255]);

            qr1.put_pixel(i, j, pixel);
        }
    }

    Ok(qr1)
}

fn generate_qrcode(data: &[u8], color: Color) -> Result<DynamicImage> {
    let qr = QRBuilder::new(data.to_vec())
        .mode(fast_qr::Mode::Byte)
        .ecl(fast_qr::ECL::L)
        .build()?;

    // 77 is the width of a 512 bytes qrcode
    let bytes = ImageBuilder::default()
        .margin(0)
        // .fit_width((qr.size * QRSIZE as usize / 77) as u32)
        .fit_width(qr.size as u32)
        .module_color(color)
        .to_bytes(&qr)?;

    Ok(ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?)
}

fn generate_variant(data: &[u8], variant: &Variant) -> Result<DynamicImage> {
    let image = match &variant {
        Variant::QRCode => generate_qrcode(data, "#000000".into())?,
        Variant::Color8 => generate_8color_qrcode(data)?,
        Variant::Color4 => generate_4color_qrcode(data)?,
    };

    Ok(image.resize(QRSIZE as u32, QRSIZE as u32, FilterType::Nearest))
}

pub fn generate_qrcodes(file: &mut File, variant: &Variant) -> Result<Vec<DynamicImage>> {
    let file_size = file.metadata()?.len() as usize;
    let mut data = Vec::new();

    let full_chunks = file_size / CHUNK_SIZE;
    let remainder = file_size % CHUNK_SIZE;

    let mut buffer = vec![0u8; CHUNK_SIZE];

    for i in 0..full_chunks {
        file.seek(SeekFrom::Start((i * CHUNK_SIZE) as u64))?;
        file.read_exact(&mut buffer)?;

        let qrcode = generate_variant(&buffer, variant)?;
        data.push(qrcode);
    }

    if remainder != 0 {
        let mut buffer = vec![0u8; remainder];
        file.seek(SeekFrom::Start((full_chunks * CHUNK_SIZE) as u64))?;
        file.read_exact(&mut buffer)?;

        let qrcode = generate_variant(&buffer, variant)?;
        data.push(qrcode);
    }

    Ok(data)
}
