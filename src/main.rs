fn checksum(msg: &[u8]) {
    // len() returns usize which can sometimes behave unpredictably. so
    // it can return u32 depending; this means in order to avoid this
    // situation we type cast to rust 64-bit now we have a fixed width
    let _length_in_bits = (msg.len() as u64) * 8;
    let mut message: Vec<u8> = msg.to_vec();

    message.push(0x80);

    while message.len() % 64 != 56 {
        message.push(0);
    }

    println!("After padding zeros, length: {}", message.len());
}

fn main() {
    let str_text = "";
    let message = str_text.as_bytes();
    checksum(message);
}
