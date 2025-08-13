mod constants;
mod document;
mod qrcode;
mod utils;

use document::{generate_pages, save_document};
use qrcode::generate_qrcodes;

use std::io::{self, SeekFrom};
use std::{fs::File, io::Seek};

use anyhow::Result;
use clap::{Parser, ValueEnum};
use sha2::{Digest, Sha256};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    file: String,

    #[arg(short, long, default_value = "a user editable message")]
    message: String,

    #[arg(short, long, value_enum, default_value = "qr-code")]
    variant: Variant,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Variant {
    QRCode,
    Color8,
    Color4,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let time = chrono::offset::Local::now()
        .format("%d-%m-%Y %H:%M:%S")
        .to_string();

    let mut file = File::open(&args.file)?;

    let hash = {
        let mut sha256 = Sha256::new();
        io::copy(&mut file, &mut sha256)?;
        format!("{:x}", sha256.finalize())
    };

    file.seek(SeekFrom::Start(0))?;

    let qrcodes = generate_qrcodes(&mut file, &args.variant)?;
    let pages = generate_pages(qrcodes, &hash, &time, &args.message)?;

    save_document(&pages, &format!("{}.cbz", &args.file))?;

    Ok(())
}
