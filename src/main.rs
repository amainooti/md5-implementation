fn md5_pad(msg: &[u8]) -> Vec<u8> {
    let length_in_bits = (msg.len() as u64) * 8;
    let mut message: Vec<u8> = msg.to_vec();

    message.push(0x80);

    while message.len() % 64 != 56 {
        message.push(0);
    }

    let length_bytes = length_in_bits.to_le_bytes();
    message.extend_from_slice(&length_bytes);

    assert_eq!(
        message.len() % 64,
        0,
        "Padded message length must be a multiple of 64 bytes"
    );

    message
}

// ====================== MD5 Round Functions (Ticket) ======================

/// F(b, c, d) = (b & c) | (!b & d)
fn f(b: u32, c: u32, d: u32) -> u32 {
    // b = 0 & c = 1 = F
    (b & c) | (!b & d)
}

/// G(b, c, d) = (b & d) | (c & !d)
fn g(b: u32, c: u32, d: u32) -> u32 {
    (b & d) | (c & !d)
}

/// H(b, c, d) = b ^ c ^ d
fn h(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}

/// I(b, c, d) = c ^ (b | !d)
fn i(b: u32, c: u32, d: u32) -> u32 {
    c ^ (b | !d)
}

struct MD5State {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl MD5State {
    fn new() -> Self {
        Self {
            a: 0x67452301,
            b: 0xefcdab89,
            c: 0x98badcfe,
            d: 0x10325476,
        }
    }
}

fn main() {
    let str_text = "hello";
    let message = str_text.as_bytes();
    let padded = md5_pad(message);
    println!("Padded length: {} bytes", padded.len());
}

// ====================== TESTS ======================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_pad_empty() {
        let msg = b"";
        let padded = md5_pad(msg);
        assert_eq!(
            padded.len(),
            64,
            "Empty message should pad to exactly 64 bytes"
        );
    }

    #[test]
    fn test_md5_pad_hello() {
        let msg = b"hello";
        let padded = md5_pad(msg);
        assert_eq!(padded.len() % 64, 0);
    }
}
