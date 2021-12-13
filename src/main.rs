#![allow(non_snake_case)]

use ark_ff::{PrimeField};
use ark_ec::ProjectiveCurve;
use ark_bls12_381::{Fr, G1Projective};

// Find vanity with `n_bits` collision
pub fn find_vanity(n_bits: u32, k: Fr, G: G1Projective) -> u32 {
    let mut kG = G.mul(k.into_repr());

    let mut num = u32::MAX;
    while num.leading_zeros() != n_bits {
        kG = kG + G;
        let kG_serialized = ark_ff::to_bytes![kG].unwrap();
        num = u32::from_be_bytes(kG_serialized[0..4].try_into().unwrap());
    }

    return num;
}


#[allow(dead_code)]
// All the work actually happens in the benchmark in benches/perf.rs
fn main() {
}
