use std::{ascii::AsciiExt, fs::File, io::stdout, iter::once};

fn main() {
    let file_path = "./test-files/cat.bmp";
    let bytes = std::fs::read(file_path).unwrap();

    let header_bytes: &[u8] = bytes.as_slice();
    let header_bytes: [u8; 40] = header_bytes[0..40].try_into().expect("Incorrect length");

    let parse_file = parse_file(&bytes);

    // let header = parse_header(header_bytes).unwrap();

    print_buffer(bytes);
}

struct BMPFile {
    file_header: BitmapFileHeader,
    dib_header: DibHeader,
    _extra_bit_masks: String,
    _colour_palette: String,
    _gap1: String,
    pixel_array: Vec<Pixel>,
    _gap2: String,
    _icc_color_profile: String,
}

struct Pixel {
    red: u32,
    green: u32,
    blue: u32,
    _alpha: u32,
}

struct DibHeader {}

fn parse_file(bytes: &Vec<u8>) -> Result<BMPFile, String> {
    // https://en.wikipedia.org/wiki/BMP_file_format#File_structure

    let file_header = parse_bitmap_file_header(bytes[0..13].try_into().expect("err parse file_header"));
    let dib_end = file_header.pixel_array_byte_offset as usize;
    let dib_header = parse_dib_header(bytes[14..dib_end].try_into().expect("err parse dib_header"));

    Ok(BMPFile {
        file_header,
        dib_header,
        _extra_bit_masks: String::new(),
        _colour_palette: String::new(),
        _gap1: String::new(),
        pixel_array: Vec::new(),
        _gap2: String::new(),
        _icc_color_profile: String::new(),
    })
}

enum BMPType {
    BM,
    BA,
    CI,
    CP,
    IC,
    PT,
}

struct BitmapFileHeader {
    header_field: BMPType,
    size: u32,
    pixel_array_byte_offset: u32,
}

fn parse_bitmap_file_header(bytes: [u8; 14]) -> BitmapFileHeader {
    // https://en.wikipedia.org/wiki/BMP_file_format#Bitmap_file_header

    let header_field_bytes = &bytes[0..2];
    if !header_field_bytes.is_ascii() {
        panic!("Bytes not ascii!");
    }

    (BitmapFileHeader {
        header_field: BMPType::BM,
        size: 0,
        pixel_array_byte_offset: 0,
    })
}

fn parse_dib_header(bytes: [u8; 40]) -> DibHeader {
    (DibHeader {})
}

// presentation

fn print_buffer(bytes: Vec<u8>) {
    let string_bytes: Vec<String> = bytes.into_iter().map(|byte| to_hex(byte)).collect();
    let output_string = string_bytes.join(" ");
    print!("{}", output_string)
}

fn to_binary(decimal: u8) -> String {
    let mut decimal = decimal;
    let mut digits = [0; 8];
    let mut index = 0;

    while decimal > 0 {
        digits[index] = decimal % 2;
        decimal /= 2;

        index += 1;
    }

    let mut string = String::new();
    digits.into_iter().for_each(|v| string += &v.to_string());

    string
}

fn to_hex(decimal: u8) -> String {
    let num1 = decimal & 0b00001111;
    let num2 = decimal / 16;

    format!("{}{}", hex_char(num1), hex_char(num2))
}

fn hex_char(hex_num: u8) -> char {
    match hex_num {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        _ => panic!("Unknown number! {}", hex_num),
    }
}
