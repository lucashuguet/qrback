use crate::Variant;

pub const CHUNK_SIZE: usize = 512;

pub const QRCODE_DEFAULT_SIZE: u32 = 77;
pub const QRCODE_SIZE: i64 = 1048;

pub const GRID_SIZE: usize = 6;
pub const MARGIN_X: i64 = 128;
pub const MARGIN_Y: i64 = 48;
pub const PAGE_HEIGHT: u32 = 3508;
pub const PAGE_WIDTH: u32 = 2480;

pub const FONT_SIZE: f32 = 64.;
pub const TEXT_HEIGHT: i64 = 54;
pub const TEXT_POS_Y: f32 = 40.;

impl Variant {
    pub fn data_density(&self) -> f32 {
        match &self {
            Variant::QRCode => 1.,
            Variant::Color4 => 2.,
            Variant::Color8 => 3.,
            Variant::JabCode4 => 1.7,
            Variant::JabCode8 => 2.5,
        }
    }
}
