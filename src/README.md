- our ticket: implement a function fn compute_t() -> [u32; 64] that computes all 64 values. You'll need f64::sin() in Rust, f64::abs(), and you're multiplying by 2^32 which is 4294967296.0_f64 or (u32::MAX as f64 + 1.0).
The cast at the end — as u32 — truncates the decimal, which is exactly what floor does here.


for each 64-byte block in padded message:
    split block into 16 × u32 words (little-endian) → M
    
    save current state: (aa, bb, cc, dd) = (a, b, c, d)
    
    for i in 0..64:
        determine which function and k based on i
        sum = a.wrapping_add(func(b,c,d))
               .wrapping_add(M[k])
               .wrapping_add(T[i])
        new_b = b.wrapping_add(sum.rotate_left(S[i]))
        (a, b, c, d) = (d, new_b, b, c)
    
    // feed-forward
    a = a.wrapping_add(aa)
    b = b.wrapping_add(bb)
    c = c.wrapping_add(cc)
    d = d.wrapping_add(dd)
