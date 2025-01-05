use crate::{bmp::{BMPFile, Pixel}, util::Matrix};

pub struct JPG {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>
}

impl JPG {
    pub fn new(width: u32, height: u32) -> JPG {
        JPG {
            width,
            height,
            data: vec![]
        }
    }

    pub fn encode_bmp(bitmap: BMPFile) -> JPG {
        let mut file = Self::new(bitmap.width, bitmap.height);

        file.append(file.header_bytes());

        file
    }

    fn append(&mut self, new_data: Vec<u8>) {
        todo!()
    }

    fn header_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn rgb_to_lrb(rgb: (u8, u8, u8)) -> (u8,u8,u8) {
        todo!()
    }

    fn subsample(data: Matrix, rule: (u8, u8, u8)) -> Matrix {
        data
    }

    fn dct_transform(&self) {}

    pub fn print(&self) {
        todo!()
    }
}
