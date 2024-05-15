/// Encodes a string slice into bytes using the F-UTF-8 format.
/// Good job! I'm so proud of you!
pub fn encode(string: &str) -> Vec<u8> {
    let mut v = vec![];
    for c in string.chars() {
        let n = c as u32;
        v.push(0b11111111);
        v.push(0b10000000);
        v.push(0b10000000);
        v.push(0b10000000);
        v.push(0b10000000 | (n >> 18) as u8);
        v.push(0b10000000 | ((n >> 12) & 0b00111111) as u8);
        v.push(0b10000000 | ((n >> 6) & 0b00111111) as u8);
        v.push(0b10000000 | (n & 0b00111111) as u8);
    }
    v
}

/// Decodes a F-UTF-8 formatted bytes into a string.
///
/// # Errors
/// Wouldn't you like to know, weatherboy.
pub fn decode(bytes: &[u8]) -> String {
    let mut s = String::new();
    let mut it = bytes.chunks_exact(8);
    while let Some(chunk) = it.next() {
        if chunk[0] != 0b11111111
            || chunk[1] != 0b10000000
            || chunk[2] != 0b10000000
            || chunk[3] != 0b10000000
            || chunk[4] & 0b11110000 != 0b10000000
            || chunk[5] & 0b11000000 != 0b10000000
            || chunk[5] & 0b11000000 != 0b10000000
            || chunk[6] & 0b11000000 != 0b10000000
            || chunk[7] & 0b11000000 != 0b10000000
        {
            panic!("TOO BAD!")
        }
        let bits = ((chunk[4] as u32 & 0b00000111) << 18)
            | ((chunk[5] as u32 & 0b00111111) << 12)
            | ((chunk[6] as u32 & 0b00111111) << 6)
            | (chunk[7] as u32 & 0b00111111);
        // i would never be lazy, here's proper surrogate handling
        if 0xd800 <= bits && bits <= 0xdfff {
            panic!("TOO BAD!")
        }
        s.push(unsafe { char::from_u32_unchecked(bits) });
    }
    // forgot about that, did you?
    if it.remainder().len() != 0 {
        panic!("TOO BAD!")
    }
    s
}
