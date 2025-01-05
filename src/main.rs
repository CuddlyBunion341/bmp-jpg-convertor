use std::{ascii::AsciiExt, fs::File, io::stdout, iter::once};

use bmp::parse_file;

mod bmp;
mod jpg;
mod util;

fn main() {
    let file_path = "./test-files/cat.bmp";
    let bytes = std::fs::read(file_path).unwrap();

    let file = parse_file(&bytes).unwrap();

    file.print_ascii();
}
