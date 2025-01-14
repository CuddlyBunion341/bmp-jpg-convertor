impl BMPFile {
    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Self, String> {
        parse_file(bytes)
    }

    pub fn print_ascii(&self) {
        for y in 0..self.height {
            let mut row = String::new();
            for x in 0..self.width {
                let index = y * self.height + x;
                let pixel = &self.pixels[index as usize];

                row += Self::pixel_to_ascii(&pixel).to_string().as_str();
            }

            println!("{}", row);
        }
    }

    fn pixel_to_ascii(pixel: &Pixel) -> char {
        let value: u8 = pixel.red / 3 + pixel.green / 3 + pixel.blue / 3;

        let value = value / 32;

        match value {
            0 => ' ',
            1 => '.',
            2 => '_',
            3 => 'f',
            4 => '0',
            5 => '$',
            6 => '#',
            7 => '@',
            _ => panic!("UNKWON VLAUE")
        }
    }
}

pub struct BMPFile {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Pixel>,
}

pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn parse_file(bytes: &Vec<u8>) -> Result<BMPFile, String> {
    // https://en.wikipedia.org/wiki/BMP_file_format#File_structure

    // let width = le_32(bytes[18..(18 + 4)].try_into().expect(""));
    // let height = le_32(bytes[22..(22 + 4)].try_into().expect(""));
    let width = 1704; // FIXME: remove hardcoding
    let height = 2272; // FIXME: remove hardcoding
    let color_depth = le_16(bytes[28..(28 + 2)].try_into().expect(""));

    let pixel_offset = pixel_byte_offset(&bytes[0..14]);

    Ok(BMPFile {
        width,
        height,
        pixels: parse_pixel_array(&bytes[(pixel_offset as usize)..], color_depth),
    })
}

fn parse_pixel_array(bytes: &[u8], color_depth: u32) -> Vec<Pixel> {
    if color_depth != 32 {
        panic!("Unknown color depth: {}", color_depth);
    }

    let mut values: Vec<(u8, u8, u8, u8)> = vec![];

    for pixel_index in 0..(bytes.len() / 4) {
        values.push((
            bytes[pixel_index * 4 + 0],
            bytes[pixel_index * 4 + 1],
            bytes[pixel_index * 4 + 2],
            bytes[pixel_index * 4 + 3],
        ))
    }

    values
        .into_iter()
        .map(|tuple| {
            let (red, green, blue, _alpha) = tuple;

            Pixel { red, green, blue }
        })
        .collect()
}

fn pixel_byte_offset(bytes: &[u8]) -> u32 {
    // https://en.wikipedia.org/wiki/BMP_file_format#Bitmap_file_header

    let header_field_bytes = &bytes[0..2];
    if !header_field_bytes.is_ascii() {
        panic!("Bytes not ascii!");
    }

    let format = format!(
        "{}{}",
        repr_u8_as_ascii(header_field_bytes[0].to_be()),
        repr_u8_as_ascii(header_field_bytes[1].to_be())
    );

    if format != "BM" {
        panic!("Unefined format: {}", format)
    }

    let pixel_offset: [u8; 4] = bytes[10..14].try_into().expect("");
    let pixel_offset = le_32(pixel_offset);

    pixel_offset
}

fn be_32(bytes: [u8; 4]) -> u32 {
    let mut num: u32 = 0;

    num = (num << 0) + bytes[0] as u32;
    num = (num << 8) + bytes[1] as u32;
    num = (num << 8) + bytes[2] as u32;
    num = (num << 8) + bytes[3] as u32;

    num
}

fn le_32(bytes: [u8; 4]) -> u32 {
    be_32([bytes[3], bytes[2], bytes[1], bytes[0]])
}

fn le_16(bytes: [u8; 2]) -> u32 {
    le_32([bytes[0], bytes[1], 0, 0])
}

fn be_16(bytes: [u8; 2]) -> u32 {
    le_16([bytes[0], bytes[1]])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_be_32() {
        assert_eq!(
            be_32([0b01001001, 0b10010110, 0b00000010, 0b11010010]),
            1234567890
        );
    }

    #[test]
    fn test_le_32() {
        assert_eq!(
            le_32([0b11010010, 0b00000010, 0b10010110, 0b01001001]),
            1234567890
        );
    }
}

fn repr_u8_as_ascii(digit: u8) -> char {
    char::from_u32(digit.try_into().unwrap()).unwrap()
}
