pub fn parse_key(b: u8) -> Option<char> {
    if (32..127).contains(&b) {
        Some(b as u8 as char)
    } else {
        None
    }
}

