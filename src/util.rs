pub struct Matrix {
    width: usize,
    height: usize,
    data: Vec<u8>
}

impl Matrix {
    pub fn new(width: u32, height: u32) -> Matrix {
        Matrix {
            width: width as usize,
            height: height as usize,
            data: vec!()
        }
    }

    pub fn get(&self, x: u32, y: u32) -> u8 {
        let index = y * (self.width as u32) + x;
        self.data[index as usize]
    }

    pub fn set(&mut self, x: u32, y: u32, value: u8) {
        let index = y * (self.width as u32) + x;
        self.data[index as usize] = value;
    }
}

fn print_buffer(bytes: Vec<u8>) {
    let string_bytes: Vec<String> = bytes.into_iter().map(|byte| to_hex(byte)).collect();
    let output_string = string_bytes.join(" ");
    println!("{}", output_string)
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
