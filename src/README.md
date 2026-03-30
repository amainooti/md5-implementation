- our ticket: implement a function fn compute_t() -> [u32; 64] that computes all 64 values. You'll need f64::sin() in Rust, f64::abs(), and you're multiplying by 2^32 which is 4294967296.0_f64 or (u32::MAX as f64 + 1.0).
The cast at the end — as u32 — truncates the decimal, which is exactly what floor does here.
