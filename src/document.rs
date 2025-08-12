use super::utils::create_text_image;

use std::fs::File;
use std::io::{Cursor, Write};

use anyhow::Result;
use image::DynamicImage;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

pub const QRSIZE: i64 = 1048;

const GRID_SIZE: usize = 6;
const PAGE_WIDTH: u32 = 2480;
const PAGE_HEIGHT: u32 = 3508;
const MARGIN: i64 = 128;
const HEADER_MARGIN: i64 = 108;

pub fn save_document(pages: &[DynamicImage], output: &str) -> Result<()> {
    let mut output = File::create(output)?;
    let mut zip = ZipWriter::new(&mut output);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for (i, page) in pages.iter().enumerate() {
        zip.start_file(format!("page{i:06}.png"), options)?;

        let mut buffer = Vec::new();
        page.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)?;

        zip.write_all(&buffer)?;
    }

    zip.finish()?;

    Ok(())
}

pub fn generate_pages(
    qrcodes: Vec<DynamicImage>,
    hash: &str,
    time: &str,
    message: &str,
) -> Result<Vec<DynamicImage>> {
    let n_pages = qrcodes.len() / GRID_SIZE + if qrcodes.len() % GRID_SIZE != 0 { 1 } else { 0 };
    Ok(qrcodes
        .chunks(GRID_SIZE)
        .enumerate()
        .map(|(j, chunk)| {
            let mut page = DynamicImage::new_rgb8(PAGE_WIDTH, PAGE_HEIGHT);
            page.invert();

            let info_text_image = create_text_image(
                PAGE_WIDTH,
                100,
                (64, 64),
                &format!("PAGE {:03}/{:03} {hash}", j + 1, n_pages),
            );
            image::imageops::overlay(&mut page, &info_text_image, 0, 0);

            let msg_text_image =
                create_text_image(PAGE_WIDTH, 72, (64, 48), &format!("{time} {message}"));
            image::imageops::overlay(&mut page, &msg_text_image, 0, 80);

            for (i, qr) in chunk.iter().enumerate() {
                let i = i as i64;
                let offset = (QRSIZE - qr.width() as i64) / 2;
                image::imageops::overlay(
                    &mut page,
                    qr,
                    MARGIN + i % 2 * (QRSIZE + MARGIN) + offset,
                    HEADER_MARGIN + MARGIN / 2 + i / 2 * (QRSIZE + MARGIN / 2) + offset,
                );
            }

            page
        })
        .collect())
}
