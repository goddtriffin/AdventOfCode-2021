pub fn binary_string(num: u16) -> String {
    let mut builder = String::new();
    for i in 0..16 {
        builder.push_str(&format!("{}", num >> (15 - i) & 1));
    }
    builder
}

pub fn print_binary(num: u16) {
    print!("{}:\t", num);
    println!("{}", binary_string(num));
}

pub fn get_bit(num: u16, bit_offset: u16) -> u16 {
    if bit_offset < 1 {
        panic!(
            "Smallest bit_offset index is 1 - bit_offset {} was attempted",
            bit_offset
        );
    }
    if bit_offset > 16 {
        panic!(
            "Maximum u16 size is 16 bits long - bit_offset {} was attempted",
            bit_offset
        );
    }

    num >> (bit_offset - 1) & 1
}

pub fn set_bit(num: u16, bit_offset: u16, value: bool) -> u16 {
    if bit_offset < 1 {
        panic!(
            "Smallest bit_offset index is 1 - bit_offset {} was attempted",
            bit_offset
        );
    }
    if bit_offset > 16 {
        panic!(
            "Maximum u16 size is 16 bits long - bit_offset {} was attempted",
            bit_offset
        );
    }

    let mask = 1 << (bit_offset - 1);
    if value {
        // set as 1
        num | mask
    } else {
        // set as 0
        num & !mask
    }
}
