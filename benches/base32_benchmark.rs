use base_n::Decoder;
use base_n::Encoder;
use base_n::base32::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rand::prelude::StdRng;
use rand::{RngCore, SeedableRng};

const VARIANTS: &[Base32Variant] = &[YIDU, YIDU_PAD];

const TEST_SIZES: &[usize] = &[10, 100, 1000, 10000];

fn bench_base32_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("Base32 Encoding");

    let mut rng = StdRng::seed_from_u64(7355608);
    let mut test_data: Vec<u8> = vec![0; *TEST_SIZES.iter().max().unwrap()];
    rng.fill_bytes(&mut test_data);

    for variant in VARIANTS {
        for &size in TEST_SIZES {
            group.bench_with_input(
                BenchmarkId::new(variant.name(), size),
                &test_data[..size],
                |b, data| b.iter(|| black_box(variant.encode(black_box(data)))),
            );
        }
    }

    group.finish();
}

fn bench_base32_decoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("Base32 Decoding");
    let mut rng = StdRng::seed_from_u64(7355608);
    let mut test_data: Vec<u8> = vec![0; *TEST_SIZES.iter().max().unwrap()];
    rng.fill_bytes(&mut test_data);

    for variant in VARIANTS {
        for &size in TEST_SIZES {
            let encoded = variant.encode(&test_data[..size]);
            group.bench_with_input(
                BenchmarkId::new(variant.name(), size),
                &encoded,
                |b, data| b.iter(|| black_box(variant.decode(black_box(data)).unwrap())),
            );
        }
    }

    group.finish();
}

criterion_group!(benches, bench_base32_encoding, bench_base32_decoding);
criterion_main!(benches);
