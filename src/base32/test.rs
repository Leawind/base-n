extern crate std;

use crate::base32::{
    Base32Variant, CROCKFORD, RFC4648_HEX, RFC4648_HEX_LOWER, RFC4648_HEX_LOWER_PAD,
    RFC4648_HEX_PAD, RFC4648_HIGHER, RFC4648_HIGHER_PAD, RFC4648_LOWER, RFC4648_LOWER_PAD, YIDU, Z,
};
use crate::base32::{RFC4648, RFC4648_PAD, YIDU_PAD};
use crate::{Decoder, Encoder};
use alloc::vec;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

const VARIANTS: &[Base32Variant] = &[
    CROCKFORD,
    RFC4648,
    RFC4648_PAD,
    RFC4648_HIGHER,
    RFC4648_HIGHER_PAD,
    RFC4648_LOWER,
    RFC4648_LOWER_PAD,
    RFC4648_HEX,
    RFC4648_HEX_PAD,
    RFC4648_HEX_LOWER,
    RFC4648_HEX_LOWER_PAD,
    Z,
    YIDU,
    YIDU_PAD,
];

#[test]
fn test_rfc4648_lower_pad() {
    let variant = RFC4648_LOWER_PAD;
    let data = vec![0b00010011; 2];
    /*
     *      byte | 00010011 00010011
     *           | 0^^^^1^^ ^^2^^^^3 ^^^^4^^^ ^5^^^^6^ ^^^7^^^^
     *     byte5 | 00010, 01100, 01001, 10000, 00000, 00000, 00000, 00000
     *     byte5 | 2, 12, 9, 16, 0, 0, 0, 0
     *      code | 99, 109, 106, 113, 61, 61, 61, 61
     *      code | c, m, j, q, =, =, =, =
     *   encoded | cmjq====
     */
    let encoded = variant.encode(&data);
    std::println!("encoded: '{}'", encoded);
    std::println!("encoded: {:?}", encoded.as_bytes());
    /*
     *   encoded | cmjq====
     *     code  | 99, 109, 106, 113, 61, 61, 61, 61
     *   code_id | 51, 61, 58, 65, 13, 13, 13, 13
     *    bytes5 | 2, 12, 9, 16, 0, 0, 0, 0
     */
    let decoded = variant.decode(&encoded).expect("Failed to decode");
    assert_eq!(data, decoded);
}

#[test]
fn test_all() {
    const SIZE: usize = 1024;
    const ROUNDS: u64 = 64;
    const SEED: u64 = 7355608;

    let mut rng = StdRng::seed_from_u64(SEED);
    let mut data = vec![0u8; SIZE];

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:>7}/{len:<7} {msg}")
        .unwrap()
        .progress_chars("=> ");

    let pb_total = m.add(ProgressBar::new(ROUNDS * VARIANTS.len() as u64));
    pb_total.set_style(sty.clone());
    pb_total.set_message("total");

    let pb_rounds = m.add(ProgressBar::new(ROUNDS));
    pb_rounds.set_style(sty.clone());
    pb_rounds.set_message("rounds");

    for variant in VARIANTS {
        pb_rounds.reset();
        for _ in 0..ROUNDS {
            rng.fill_bytes(&mut data);
            let encoded = variant.encode(&data);
            let decoded = variant.decode(&encoded).expect("Failed to decode");
            assert_eq!(data, decoded);
            pb_rounds.inc(1);
            pb_total.inc(1);
        }
    }
}
