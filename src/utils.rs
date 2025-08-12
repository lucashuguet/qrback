use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rusttype::{point, Font, Scale};

const FONT_SIZE: f32 = 64.0;

pub fn create_text_image(
    width: u32,
    height: u32,
    text_position: (i32, i32),
    text: &str,
) -> DynamicImage {
    let font_data = include_bytes!("noto.ttf");
    let font = Font::try_from_bytes(font_data).expect("Error constructing Font");

    let mut image = DynamicImage::new_rgb8(width, height);
    image.invert();

    let scale = Scale {
        x: FONT_SIZE,
        y: FONT_SIZE,
    };
    let start = point(text_position.0 as f32, text_position.1 as f32);
    let color = Rgba([0, 0, 0, 255]);

    for glyph in font.layout(text, scale, start) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
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
