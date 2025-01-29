extern crate std;
use crate::base32::Base32Variant;
use crate::{Decoder, Encoder, base32};

#[test]
fn simple_example() {
    let data = 0x000000B93246A429u64.to_be_bytes().to_vec();
    assert_eq!(&data, &[0x00, 0x00, 0x00, 0xB9, 0x32, 0x46, 0xa4, 0x29]);

    // Encoding
    let encoded = base32::RFC4648_PAD.encode(&data);
    assert_eq!(encoded, "AAAABOJSI2SCS===");

    // Decoding
    let decoded = base32::RFC4648_PAD.decode(&encoded).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn customized_alphabet() {
    const CUSTOM_BASE32: Base32Variant = Base32Variant::builder()
        .name("Custom")
        .mapping(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ123456")
        .build();

    std::println!("{}", CUSTOM_BASE32.encode(b"Hello, world!"));
}
