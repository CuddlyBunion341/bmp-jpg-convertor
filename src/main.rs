use std::{fs::File, io::stdout, iter::once};

fn main() {
    let file_path = "./test-files/cat.bmp";
    let bytes = std::fs::read(file_path).unwrap();

    let header_bytes: &[u8] = bytes.as_slice();
    let header_bytes: [u8; 40] = header_bytes[0..40].try_into().expect("Incorrect length");

    let parse_file = parse_file(&bytes);

    // let header = parse_header(header_bytes).unwrap();

    print_buffer(bytes);
}

struct BMPFile {}

fn parse_file(bytes: &Vec<u8>) -> Result<BMPFile, String> {
    // https://docs.fileformat.com/image/bmp/
    // File Header	No	14	To store general information about the bitmap image file
    // DIB Header	No	Fixed-Size	To store detailed information about the bitmap image and define the pixel format
    // Extra Bit Masks	Yes	12 or 16 bytes	To define the pixel format
    // Colour Palette	Semi-optional	Variable-size	To define colours used by the bitmap image data
    // Gap1	Yes	Variable-size	Structure alignment
    // Pixel Array	No	Variable-size	Pixel format is defined by the DIB header or Extra bit masks.
    // Gap2	Yes	Variable-size	Structure alignment
    // ICC Color profile	Yes	Variable-size	To define the colour profile for colour management

    Ok(BMPFile {})
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
