use core_arch::x86_64::*;
use std::mem::transmute;

#[target_feature(enable = "avx2")]
pub unsafe fn add4(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    transmute(_mm256_add_epi64(transmute(a), transmute(b)))
}

fn main() {
    assert!(is_x86_feature_detected!("avx2"));
    unsafe {
        let a = [1, 2, 3, 4];
        let b = [2, 3, 4, 5];
        eprintln!("{:?}", add4(a, b));
    }
}
