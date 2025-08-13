use crate::Variant;

use super::constants::{FONT_SIZE, TEXT_HEIGHT, TEXT_POS_Y};

use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rusttype::{point, Font, Scale};

impl Variant {
    pub fn data_density(&self) -> usize {
        match &self {
            Variant::QRCode => 1,
            Variant::Color4 => 2,
            Variant::Color8 => 3,
        }
    }
}

pub fn create_text_image(width: u32, margin: i64, text: &str) -> DynamicImage {
    let font_data = include_bytes!("noto.ttf");
    let font = Font::try_from_bytes(font_data).expect("Error constructing Font");

    let mut image = DynamicImage::new_rgb8(width, TEXT_HEIGHT as u32);
    image.invert();

    let scale = Scale {
        x: FONT_SIZE,
        y: FONT_SIZE,
    };
    let start = point(margin as f32, TEXT_POS_Y);
    let color = Rgba([0, 0, 0, 255]);

    for glyph in font.layout(text, scale, start) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 0 && x < width as i32 && y >= 0 && y < TEXT_HEIGHT as i32 {
                    let pixel = image.get_pixel(x as u32, y as u32);
                    let blend = |c: u8| (v * c as f32 + (1.0 - v) * pixel[0] as f32) as u8;
                    image.put_pixel(
                        x as u32,
                        y as u32,
                        Rgba([blend(color[0]), blend(color[1]), blend(color[2]), 255]),
                    );
                }
            });
        }
    }

    image
}
