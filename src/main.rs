mod utils;

use utils::{create_text_image, generate_qr_image};

use anyhow::Result;
use image::{ImageFormat, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use std::fs::{File, OpenOptions};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};

const READ_SIZE: usize = 512;
const PAGE_WIDTH: u32 = 2480;
const PAGE_HEIGHT: u32 = 3508;
const QRSIZE: i64 = 1048;
const MARGIN: i64 = 128;
const HEADER_MARGIN: i64 = PAGE_HEIGHT as i64 - 3 * QRSIZE - 2 * MARGIN;
const FONT_SIZE: f32 = 64.0;

fn main() -> Result<()> {
    let filename = "test.gpg";
    let msg = "super encrypted file";

    let time = chrono::offset::Local::now()
        .format("%d-%m-%Y %H:%M:%S")
        .to_string();

    let mut file = File::open(filename)?;
    let size = file.metadata()?.len() as usize;

    let mut sha256 = Sha256::new();
    io::copy(&mut file, &mut sha256)?;
    let hash = format!("{:x}", sha256.finalize());

    let mut images = Vec::new();

    let full_chunks = size / READ_SIZE;
    let remainder = size % READ_SIZE;

    let bar = ProgressBar::new((full_chunks + 1) as u64).with_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] [{pos}/{len}]")
            .unwrap()
            .progress_chars("##-"),
    );

    let mut buffer = vec![0u8; READ_SIZE];

    for i in 0..full_chunks {
        file.seek(SeekFrom::Start((i * READ_SIZE) as u64))?;
        file.read_exact(&mut buffer)?;

        let img = generate_qr_image(&buffer)?;
        images.push(img);

        bar.inc(1);
    }

    if remainder != 0 {
        let mut last_buf = vec![0u8; remainder];
        file.seek(SeekFrom::Start((full_chunks * READ_SIZE) as u64))?;
        file.read_exact(&mut last_buf)?;

        let img = generate_qr_image(&last_buf)?;
        images.push(img);

        bar.inc(1);
    }

    bar.finish();

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{filename}.cbz"))?;
    let mut zip = ZipWriter::new(&mut output);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    let n_pages = images.len() / 6 + if images.len() % 6 != 0 { 1 } else { 0 };
    for (j, chunk) in images.chunks(6).enumerate() {
        let mut page =
            image::ImageBuffer::from_pixel(PAGE_WIDTH, PAGE_HEIGHT, Rgb([255, 255, 255]));

        let info_text_image = create_text_image(
            PAGE_WIDTH,
            100,
            (64, 64),
            &format!("PAGE {:03}/{:03} {hash}", j + 1, n_pages),
        );
        image::imageops::overlay(&mut page, &info_text_image, 0, 0);

        let msg_text_image = create_text_image(PAGE_WIDTH, 72, (64, 48), &format!("{time} {msg}"));
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

        zip.start_file(format!("page{j:03}.png"), options)?;

        let mut bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        page.write_to(&mut bytes, ImageFormat::Png)?;

        zip.write_all(&bytes.into_inner())?;
    }

    zip.finish()?;

    Ok(())
}
