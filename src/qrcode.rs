use crate::Variant;

use super::document::QRSIZE;

use std::fs::File;
use std::io::{Cursor, Read};

use anyhow::Result;
use fast_qr::convert::image::ImageBuilder;
use fast_qr::convert::{Builder, Color};
use fast_qr::QRBuilder;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, GenericImageView, ImageReader, Rgba};

const CHUNK_SIZE: usize = 512;

fn generate_4color_qrcode(data: &[u8]) -> Result<DynamicImage> {
    let size = data.len() / 2;

    let mut qr1 = generate_qrcode(&data[0..size], "#0000ff".into())?; // blue
    let qr2 = generate_qrcode(&data[size..], "#ffff00".into())?; // yellow

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
    let size = data.len() / 3;

    let mut qr1 = generate_qrcode(&data[0..size], "#00ffff".into())?; // cyan
    let qr2 = generate_qrcode(&data[size..2 * size], "#ff00ff".into())?; // magenta
    let qr3 = generate_qrcode(&data[2 * size..], "#ffff00".into())?; // yellow

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

    let bytes = ImageBuilder::default()
        .margin(0)
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
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let qrcodes = buffer
        .chunks(CHUNK_SIZE * variant.data_density())
        .map(|chunk| generate_variant(chunk, variant))
        .collect::<Result<Vec<_>>>()?;

    Ok(qrcodes)
}
