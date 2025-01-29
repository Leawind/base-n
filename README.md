# base-n

A Base32 encoding/decoding library supporting multiple alphabats.

It is `#![no_std]` compatible.

## Usage

Encode data using RFC 4648 with padding

```rust
use base_n::{Encoder, base32, Decoder};

fn main() {
    let data = 0xB93246A429u64.to_be_bytes().to_vec();
    assert_eq!(&data, &[0, 0, 0, 0xB9, 0x32, 0x46, 0xa4, 0x29]);
    
    // Encoding
    let encoded = base32::RFC4648_PAD.encode(&data);
    assert_eq!(encoded, "AAAABOJSI2SCS===");
    
    // Decoding
    let decoded = base32::RFC4648_PAD.decode(&encoded).unwrap();
    assert_eq!(data, decoded);
}
```

Customized alphabet

```rust
use base_n::{Encoder, base32::Base32Variant};
const CUSTOM_BASE32: Base32Variant = Base32Variant::builder()
    .name("Custom")
    .mapping(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ123456")
    .build();
println!("{}", CUSTOM_BASE32.encode(b"Hello, world!")); // JBSWY2DPFQQHO22SNRSCC
```

## Multiple Base32 variants support

- [RFC 4648](https://en.wikipedia.org/wiki/Base32#Base_32_Encoding_per_%C2%A76)
- [RFC 4648 Hex](https://en.wikipedia.org/wiki/Base32#Base_32_Encoding_with_Extended_Hex_Alphabet_per_%C2%A77)
- [Crockford Base32](https://en.wikipedia.org/wiki/Base32#Crockford's_Base32)
- [Z-Base32](https://en.wikipedia.org/wiki/Base32#z-base-32)
- YiDu (Chinese-optimized, avoids pronouncing confusion like E~1 / R~2)
- Custom variants
