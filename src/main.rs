use std::io;
use std::convert::TryInto;

fn main() {
    let mut hex_value = String::new();

    //TODO handle error in non hex char
    io::stdin()
        .read_line(&mut hex_value)
        .expect("Failed to read line");

    hex_value.pop();

    let mut hex_len = hex_value.chars().count();

    if hex_len > 8 {
        hex_value.truncate(8);
        hex_len = 8;
    }

    if hex_len % 2 != 0 {
        hex_value = "0".to_owned() + &hex_value;
        hex_len += 1;
    }

    let mut hex_value_32 = hex_value.clone();

    let mut hex_value_16 = hex_value.clone();

    match hex_len {
        2 => {
            hex_value_32 += &"000000".to_owned();
             hex_value_16 += &"00".to_owned()
         }
        4 => hex_value_32 += &"0000".to_owned(),
        6 => hex_value_32 += &"00".to_owned(),
        _ => (),
    }
    hex_value_16.truncate(4);

    //get reverse order for big endian
    let hex_value_rev_32 = reverse_hex(&hex_value_32);
    let hex_value_rev_16 = reverse_hex(&hex_value_16);

    let uint32_little = hex_to_uint(&hex_value_rev_32);
    let uint32_big = hex_to_uint(&hex_value_32);
    let int32_little = hex_to_int(&hex_value_rev_32);
    let int32_big = hex_to_int(&hex_value_32);
    let uint16_little = hex_to_uint(&hex_value_rev_16);
    let uint16_big = hex_to_uint(&hex_value_16);
    let int16_little = hex_to_int(&hex_value_rev_16);
    let int16_big = hex_to_int(&hex_value_16);

    int32_little.to_string();
    int32_big.to_string();
    uint32_little.to_string();
    uint32_big.to_string();
    uint16_little.to_string();
    uint16_big.to_string();
    int16_little.to_string();
    int16_big.to_string();

    println!("--32 bit--");
    println!("Hex: {}", hex_value_32);
    println!("UINT32 - Little Endian: {}", uint32_little);
    println!("UINT32 - Big Endian: {}", uint32_big);
    println!("INT32 - Little Endian: {}", int32_little);
    println!("INT32 - Big Endian: {}", int32_big);
    println!("--16 bit--");
    println!("Hex: {}", hex_value_16);
    println!("UINT16 - Little Endian: {}", uint16_little);
    println!("UINT16 - Big Endian: {}", uint16_big);
    println!("INT16 - Little Endian: {}", int16_little);
    println!("INT16 - Big Endian: {}", int16_big);
}

fn reverse_hex(hex_value: &str) -> String {
    let mut hex_value_rev = String::new();
    let mut number = hex_value.chars().count();
    let chars = hex_value.chars();
    while number > 0 {
        let mut c_chars = chars.clone();
        hex_value_rev += &c_chars.nth(number-2).unwrap().to_string();
        hex_value_rev += &c_chars.nth(0).unwrap().to_string();
        number -= 2;
    }

    return hex_value_rev
}

fn hex_to_uint(hex_value: &str) -> u32 {
    let mut hex_len: u32 = hex_value.chars().count().try_into().unwrap();
    let mut result = 0;
    for (_i, char) in hex_value.chars().enumerate(){
        let char_as_int: u32;
        match char {
            '0' => char_as_int = 0,
            '1' => char_as_int = 1,
            '2' => char_as_int = 2,
            '3' => char_as_int = 3,
            '4' => char_as_int = 4,
            '5' => char_as_int = 5,
            '6' => char_as_int = 6,
            '7' => char_as_int = 7,
            '8' => char_as_int = 8,
            '9' => char_as_int = 9,
            'a' | 'A' => char_as_int = 10,
            'b' | 'B' => char_as_int = 11,
            'c' | 'C' => char_as_int = 12,
            'd' | 'D' => char_as_int = 13,
            'e' | 'E' => char_as_int = 14,
            'f' | 'F' => char_as_int = 15,
            _ => char_as_int = 0,
        }

        let base_value = 16_u32.pow(hex_len - 1);
        result += base_value * char_as_int;
        hex_len -= 1;

    }

    return result
}

fn hex_to_int (hex_value: &str) -> i64 {
    let mut binaries = String::new();
    for (i, char) in hex_value.chars().enumerate(){
        let char_as_bin: &str;
        match char {
            '0' => char_as_bin = "0000",
            '1' => char_as_bin = "0001",
            '2' => char_as_bin = "0010",
            '3' => char_as_bin = "0011",
            '4' => char_as_bin = "0100",
            '5' => char_as_bin = "0101",
            '6' => char_as_bin = "0110",
            '7' => char_as_bin = "0111",
            '8' => char_as_bin = "1000",
            '9' => char_as_bin = "1001",
            'a' | 'A' => char_as_bin =  "1010",
            'b' | 'B' => char_as_bin = "1011",
            'c' | 'C' => char_as_bin = "1100",
            'd' | 'D' => char_as_bin = "1101",
            'e' | 'E' => char_as_bin = "1110",
            'f' | 'F' => char_as_bin = "1111",
            _ => char_as_bin = "",
        }
        let mut iter_char_as_bin = char_as_bin.chars();
        if i == 0  && iter_char_as_bin.nth(0).unwrap() == '0'{
            return hex_to_uint(hex_value).try_into().unwrap();
        }
        binaries += char_as_bin
    }
    let binaries_len = binaries.chars().count();

    binaries.remove(0);
    let pos_binaries = binaries;

    let mut neg_binaries = String::new();
    neg_binaries += "1";
    for _i in 1..binaries_len {
        neg_binaries += "0";
    }

    let result:i64 = (binary_to_int(&pos_binaries) as i64) - (binary_to_int(&neg_binaries) as i64);

    return result
}

fn binary_to_int (bin: &str) -> u32 {
    let mut bin_len: u32 = bin.chars().count().try_into().unwrap();
    let mut result = 0;
    for (_i, c) in bin.chars().enumerate(){
        let base_value = 2_u32.pow(bin_len - 1);
        result += base_value * c.to_digit(10).unwrap();
        bin_len -= 1;
    }
    return result
}
