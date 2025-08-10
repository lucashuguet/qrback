use crate::{FONT_SIZE, QRSIZE};

use anyhow::Result;
use fast_qr::convert::image::ImageBuilder;
use fast_qr::convert::Builder;
use fast_qr::QRBuilder;
use image::{ImageBuffer, ImageReader, Rgb, RgbImage};
use rusttype::{point, Font, Scale};

use std::io::Cursor;

pub fn generate_qr_image(data: &[u8]) -> Result<ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>> {
    let qr = QRBuilder::new(data.to_vec())
        .mode(fast_qr::Mode::Byte)
        .ecl(fast_qr::ECL::L)
        .build()?;

    let bytes = ImageBuilder::default()
        .margin(0)
        .fit_width((qr.size * QRSIZE as usize / 77) as u32) // 77 is the width of a 512 bytes qrcode
        .to_bytes(&qr)?;

    Ok(ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?
        .into_rgb8())
}

pub fn create_text_image(
    width: u32,
    height: u32,
    text_position: (i32, i32),
    text: &str,
) -> RgbImage {
    let font_data = include_bytes!("noto.ttf");
    let font = Font::try_from_bytes(font_data).expect("Error constructing Font");

    let mut image = RgbImage::from_pixel(width, height, Rgb([255, 255, 255]));
    let scale = Scale {
        x: FONT_SIZE,
        y: FONT_SIZE,
    };
    let start = point(text_position.0 as f32, text_position.1 as f32);
    let color = Rgb([0, 0, 0]);

    for glyph in font.layout(text, scale, start) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    let pixel = image.get_pixel_mut(x as u32, y as u32);
                    let blend = |c: u8| (v * c as f32 + (1.0 - v) * pixel[0] as f32) as u8;
                    *pixel = Rgb([blend(color[0]), blend(color[1]), blend(color[2])]);
                }
            });
        }
    }

    image
}
