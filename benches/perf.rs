#![allow(non_snake_case)]

use criterion::*;

use ark_ec::ProjectiveCurve;
use ark_bls12_381::{Fr,G1Projective};
use rand::{thread_rng, Rng, rngs::ThreadRng};

#[path = "../src/main.rs"]
mod main;
use crate::main::*;

fn vanity_generation(rng: &mut ThreadRng, n_bits: u32, G: G1Projective) {
    let k: Fr = rng.gen();

    let num = find_vanity(n_bits, k, G);
    assert!(num.leading_zeros() >= n_bits);
}

fn benchmark_vanity_generation(c: &mut Criterion) {
    let mut rng = thread_rng();
    let G = G1Projective::prime_subgroup_generator();

    let mut group = c.benchmark_group("vanity");
    for n_bits in [16u32, 24u32, 27u32, 32u32].iter() {
        group.throughput(Throughput::Elements(*n_bits as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n_bits), n_bits, |b, &n_bits| {
            b.iter(|| vanity_generation(&mut rng, n_bits, G));
        });
    }
    group.finish();
}

criterion_group!{name = benches;
                 // Each measurement should take one minute
                 config = Criterion::default().sample_size(10);
                 targets = benchmark_vanity_generation
                }

criterion_main!(benches);
