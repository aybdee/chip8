pub fn str_to_u8(string: &str) -> u8 {
    u8::from_str_radix(string, 16).unwrap()
}

pub fn str_to_u16(string: &str) -> u16 {
    u16::from_str_radix(string, 16).unwrap()
}
