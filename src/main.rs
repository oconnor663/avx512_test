#![feature(stdsimd, avx512_target_feature)]

use core_arch::x86_64::*;
use std::mem::transmute;

#[target_feature(enable = "avx2")]
pub unsafe fn add4(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    transmute(_mm256_add_epi64(transmute(a), transmute(b)))
}

#[target_feature(enable = "avx512f")]
pub unsafe fn add8(a: [u64; 8], b: [u64; 8]) -> [u64; 8] {
    transmute(_mm512_add_epi64(transmute(a), transmute(b)))
}

#[target_feature(enable = "avx512f")]
pub unsafe fn rrotate4(a: [u64; 4], n: i32) -> [u64; 4] {
    transmute(_mm256_ror_epi64(transmute(a), n))
}

#[target_feature(enable = "avx512f")]
pub unsafe fn rrotate8(a: [u64; 8], n: i32) -> [u64; 8] {
    transmute(_mm512_ror_epi64(transmute(a), n))
}

fn main() {
    let a4 = [0, 1, 2, 4];
    let b4 = [100; 4];

    assert!(is_x86_feature_detected!("avx2"));
    unsafe {
        eprintln!("{:?}", add4(a4, b4));
    }

    let a8 = [0, 1, 2, 4, 8, 16, 32, 64];
    let b8 = [100; 8];

    assert!(is_x86_feature_detected!("avx512f"));
    unsafe {
        eprintln!("{:?}", add8(a8, b8));
        eprintln!("{:?}", rrotate4(a4, 1));
        eprintln!("{:?}", rrotate8(a8, 1));
    }
}
