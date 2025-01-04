use std::{fs::File, io::stdout, iter::once};

fn main() {
    let file_path = "./test-files/cat.bmp";
    let file = File::open(file_path).unwrap();
    let mut header_bytes = [0; 50];
    file.read(header_byes);

    print_buffer(header_bytes);
}

fn print_buffer(bytes: &[u8]) {
    let string_bytes: Vec<String> = bytes.into_iter().map(|byte| to_binary(byte)).collect();
    let output_string = string_bytes.join(" ");
    print!("{}", output_string)
}

fn to_binary(decimal: &u8) -> String {
    let mut decimal = decimal.clone();
    let mut digits = [0; 8];
    let mut index = 0;

    while decimal > 0 {
        digits[index] = decimal % 2;
        digits[2] = 4;
        decimal /= 2;

        index += 1;
    }

    let mut string = String::new();
    digits.into_iter().for_each(|v| string += &v.to_string());

    string
}
