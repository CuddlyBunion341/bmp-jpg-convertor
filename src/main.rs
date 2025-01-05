use std::{ascii::AsciiExt, fs::File, io::stdout, iter::once};

use bmp::BMPFile;

mod bmp;
mod jpg;
mod util;

fn main() {
    let file_path = "./test-files/cat.bmp";
    let bytes = std::fs::read(file_path).unwrap();

    let file = BMPFile::from_bytes(&bytes).unwrap();
    let pixels = file.pixels;
}
