mod document;
mod qrcode;
mod utils;

use document::{generate_pages, save_document};
use qrcode::generate_qrcodes;

use std::fs::File;
use std::io;

use anyhow::Result;
use clap::Parser;
use sha2::{Digest, Sha256};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    file: String,

    #[arg(short, default_value = "a user editable message")]
    message: String,
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

    let qrcodes = generate_qrcodes(&mut file)?;
    let pages = generate_pages(qrcodes, &hash, &time, &args.message)?;

    save_document(&pages, &format!("{}.cbz", &args.file))?;

    Ok(())
}
