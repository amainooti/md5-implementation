fn checksum(msg: &[u8]) -> Vec<u8> {
    // len() returns usize which can sometimes behave unpredictably. so
    // it can return u32 depending; this means in order to avoid this
    // situation we type cast to rust 64-bit now we have a fixed width
    let length_in_bits = (msg.len() as u64) * 8;
    let mut message: Vec<u8> = msg.to_vec();

    message.push(0x80);

    while message.len() % 64 != 56 {
        message.push(0);
    }

    let length_bytes = length_in_bits.to_le_bytes(); // [u8; 8]little edian
    message.extend_from_slice(&length_bytes);

    assert_eq!(
        message.len() % 64,
        0,
        "Padded message length must be a multiple of 64 bytes"
    );
    message
}

fn main() {
    let str_text = "hello";
    let message = str_text.as_bytes();
    checksum(message);
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_of_64() {
        let msg = "";
        let message = msg.as_bytes();

        checksum(message);

        // we’ll add assertions next
    }
}
