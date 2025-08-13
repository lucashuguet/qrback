use super::constants::{
    GRID_SIZE, MARGIN_X, MARGIN_Y, PAGE_HEIGHT, PAGE_WIDTH, QRCODE_SIZE, TEXT_HEIGHT,
};
use super::utils::create_text_image;

use std::fs::File;
use std::io::{Cursor, Write};

use anyhow::Result;
use image::DynamicImage;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

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
                MARGIN_Y,
                &format!("PAGE {:03}/{:03} {hash}", j + 1, n_pages),
            );
            image::imageops::overlay(&mut page, &info_text_image, 0, MARGIN_Y);

            let msg_text_image =
                create_text_image(PAGE_WIDTH, MARGIN_Y, &format!("{time} {message}"));
            image::imageops::overlay(&mut page, &msg_text_image, 0, MARGIN_Y + TEXT_HEIGHT);

            for (i, qr) in chunk.iter().enumerate() {
                let i = i as i64;
                image::imageops::overlay(
                    &mut page,
                    qr,
                    MARGIN_X + i % 2 * (QRCODE_SIZE + MARGIN_X),
                    2 * MARGIN_Y + 2 * TEXT_HEIGHT + i / 2 * (QRCODE_SIZE + MARGIN_Y),
                );
            }

            page
        })
        .collect())
}
