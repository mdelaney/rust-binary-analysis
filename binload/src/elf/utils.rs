pub fn get_null_terminated_string_from_vec(vec: &[u8], offset: usize) -> String {
    let mut length: usize = 0;
    for (i, byte) in vec.iter().enumerate().skip(offset) {
        if *byte == 0x00 {
            length = i;
            break;
        }
    }
    let mut string = std::string::String::with_capacity(length);
    for byte in vec.iter().take(length).skip(offset) {
        string.push(*byte as char);
    }
    string
}
