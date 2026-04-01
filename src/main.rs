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

fn compute_t() -> [u32; 64] {
    let mut t = [0u32; 64];
    for i in 0..64 {
        t[i] = ((i as f64 + 1.0).sin().abs() * 4294967296.0_f64) as u32;
    }
    t
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

const S: [u32; 64] = [
    // Round 1 (F function) - 7,12,17,22 repeated 4 times
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
    // Round 2 (G function) - 5,9,14,20 repeated 4 times
    5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
    // Round 3 (H function) - 4,11,16,23 repeated 4 times
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
    // Round 4 (I function) - 6,10,15,21 repeated 4 times
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

fn process_block(state: &mut MD5State, block: &[u8], t: &[u32; 64]) {
    // 1. Break block into 16 u32 words (little-endian)
    let mut m = [0u32; 16];
    for i in 0..16 {
        m[i] = u32::from_le_bytes([
            block[i * 4],
            block[i * 4 + 1],
            block[i * 4 + 2],
            block[i * 4 + 3],
        ]);
    }

    // 2. Initialize working vars
    let mut a = state.a;
    let mut b = state.b;
    let mut c = state.c;
    let mut d = state.d;

    // Save original state
    let (aa, bb, cc, dd) = (a, b, c, d);

    // 3. Main loop
    for i_idx in 0..64 {
        let (f_val, k) = match i_idx {
            0..=15 => (f(b, c, d), i_idx),
            16..=31 => (g(b, c, d), (5 * i_idx + 1) % 16),
            32..=47 => (h(b, c, d), (3 * i_idx + 5) % 16),
            _ => (i(b, c, d), (7 * i_idx) % 16),
        };

        let temp = a
            .wrapping_add(f_val)
            .wrapping_add(m[k])
            .wrapping_add(t[i_idx]);

        let new_b = b.wrapping_add(temp.rotate_left(S[i_idx]));

        // Rotate state
        a = d;
        d = c;
        c = b;
        b = new_b;
    }

    // 4. Feed-forward
    state.a = state.a.wrapping_add(aa);
    state.b = state.b.wrapping_add(bb);
    state.c = state.c.wrapping_add(cc);
    state.d = state.d.wrapping_add(dd);
}

fn main() {
    let num: i32 = 15;
    let mut arr: Vec<i32> = Vec::new();
    let mut counter = 0;

    while counter < num {
        arr.push(counter);
        counter += 1;
    }

    println!("{:?}", arr);

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
