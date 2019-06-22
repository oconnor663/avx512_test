#![feature(stdsimd, avx512_target_feature)]

use core_arch::x86_64::*;
use std::mem::transmute;

#[target_feature(enable = "avx2")]
pub unsafe fn add8x32(a: [u32; 8], b: [u32; 8]) -> [u32; 8] {
    transmute(_mm256_add_epi32(transmute(a), transmute(b)))
}

#[target_feature(enable = "avx2")]
pub unsafe fn add4x64(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    transmute(_mm256_add_epi64(transmute(a), transmute(b)))
}

#[target_feature(enable = "avx512f")]
pub unsafe fn add16x32(a: [u32; 16], b: [u32; 16]) -> [u32; 16] {
    transmute(_mm512_add_epi32(transmute(a), transmute(b)))
}

#[target_feature(enable = "avx512f")]
pub unsafe fn add8x64(a: [u64; 8], b: [u64; 8]) -> [u64; 8] {
    transmute(_mm512_add_epi64(transmute(a), transmute(b)))
}

#[target_feature(enable = "avx512f", enable = "avx512vl")]
pub unsafe fn rrotate8x32(a: [u32; 8]) -> [u32; 8] {
    transmute(_mm256_ror_epi32(transmute(a), 1))
}

#[target_feature(enable = "avx512f", enable = "avx512vl")]
pub unsafe fn rrotate4x64(a: [u64; 4]) -> [u64; 4] {
    transmute(_mm256_ror_epi64(transmute(a), 1))
}

#[target_feature(enable = "avx512f")]
pub unsafe fn rrotate16x32(a: [u32; 16]) -> [u32; 16] {
    transmute(_mm512_ror_epi32(transmute(a), 1))
}

#[target_feature(enable = "avx512f")]
pub unsafe fn rrotate8x64(a: [u64; 8]) -> [u64; 8] {
    transmute(_mm512_ror_epi64(transmute(a), 1))
}

fn main() {
    let a_8x32 = [0u32, 1, 2, 4, 8, 16, 32, 64];
    let a_4x64 = [0u64, 1, 2, 4];

    let b_8x32 = [100; 8];
    let b_4x64 = [100; 4];

    assert!(is_x86_feature_detected!("avx2"));
    unsafe {
        eprintln!("add4x64 {:?}", add4x64(a_4x64, b_4x64));
        eprintln!("add8x32 {:?}", add8x32(a_8x32, b_8x32));
    }

    let a_16x32 = [
        0u32, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384,
    ];
    let a_8x64 = [0u64, 1, 2, 4, 8, 16, 32, 64];

    let b_16x32 = [100; 16];
    let b_8x64 = [100; 8];

    assert!(is_x86_feature_detected!("avx512f"));
    assert!(is_x86_feature_detected!("avx512vl"));
    unsafe {
        eprintln!("add 16x32 {:?}", add16x32(a_16x32, b_16x32));
        eprintln!("add 8x64  {:?}", add8x64(a_8x64, b_8x64));

        eprintln!("rrotate4x64 {:?}", rrotate4x64(a_4x64));
        eprintln!("rrotate8x32 {:?}", rrotate8x32(a_8x32));

        eprintln!("rrotate8x64  {:?}", rrotate8x64(a_8x64));
        eprintln!("rrotate16x32 {:?}", rrotate16x32(a_16x32));
    }
}
