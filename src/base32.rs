use crate::{Decoder, Encoder};
use alloc::string::String;
use alloc::vec::Vec;
use builder::Builder;
use core::cmp::min;
use core::fmt;

pub mod builder;
#[cfg(test)]
mod test;

pub struct Base32Variant<'a> {
    name: Option<&'a str>,
    padding: bool,
    byte5_to_default_code: [u8; 32],
    /// Used in decoding.
    ///
    /// `byte5 >= 32` means the `code` with the given `code_id` is unexpected in the encoded string.
    ///
    /// Refer: [`Base32Variant::get_code_id`]
    code_id_to_byte5: [u8; 75],
}

impl<'a> Base32Variant<'a> {
    pub const fn name(&self) -> &'a str {
        if let Some(name) = self.name {
            name
        } else {
            "<Unnamed>"
        }
    }
    /// ascii char --> code id
    ///   `0..255` --> `0..75` | `255`
    ///
    /// ### Returns
    ///
    /// - `0..75` if ch is valid code
    /// - `255` if ch is invalid
    #[inline]
    const fn get_code_id(ch: u8) -> u8 {
        match ch {
            b'0'..=b'z' => ch - b'0',
            _ => 255,
        }
    }

    #[inline]
    const fn code(&self, byte5: u8) -> u8 {
        self.byte5_to_default_code[byte5 as usize]
    }

    /// ```rust
    /// use base_n::base32::Base32Variant;
    ///
    /// let yidu = Base32Variant::builder()
    ///     .mapping(b"0123456789ABCDFGHIJKMNPQSTUVWXYZ")
    ///     .mapping(b"          abcdfghijkmnpqstuvwxyz")
    ///     .map_chars(b"Oo", 0)
    ///     .build();
    /// ```
    pub const fn builder() -> Builder<'a> {
        Builder(Self {
            name: None,
            padding: false,
            byte5_to_default_code: [255; 32],
            code_id_to_byte5: [0u8; 75],
        })
    }
    pub const fn padding(&self) -> bool {
        self.padding
    }
}
impl<'a> fmt::Display for Base32Variant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Encoder for Base32Variant<'_> {
    fn encode(&self, data: &[u8]) -> String {
        // Modified from: [base32::encode](https://github.com/andreasots/base32/blob/eed2049099a52e51a29a1b4ed0502404481b194f/src/lib.rs#L29)

        let mut encoded = Vec::with_capacity((data.len() + 3) / 4 * 5);

        for chunk in data.chunks(5) {
            let buf = {
                let mut buf = [0u8; 5];
                for (i, &b) in chunk.iter().enumerate() {
                    buf[i] = b;
                }
                buf
            };
            encoded.push(self.code((buf[0] & 0b11111000) >> 3));
            encoded.push(self.code(((buf[0] & 0b00000111) << 2) | ((buf[1] & 0b11000000) >> 6)));
            encoded.push(self.code((buf[1] & 0b00111110) >> 1));
            encoded.push(self.code(((buf[1] & 0x00000001) << 4) | ((buf[2] & 0b11110000) >> 4)));
            encoded.push(self.code(((buf[2] & 0b00001111) << 1) | (buf[3] >> 7)));
            encoded.push(self.code((buf[3] & 0b1111100) >> 2));
            encoded.push(self.code(((buf[3] & 0b00000011) << 3) | ((buf[4] & 0b11100000) >> 5)));
            encoded.push(self.code(buf[4] & 0b00011111));
        }

        if data.len() % 5 != 0 {
            let len = encoded.len();
            let num_extra = 8 - (data.len() % 5 * 8 + 4) / 5;
            if self.padding {
                for i in 1..num_extra + 1 {
                    encoded[len - i] = b'=';
                }
            } else {
                encoded.truncate(len - num_extra);
            }
        }
        String::from_utf8(encoded).unwrap()
    }
}

impl Decoder for Base32Variant<'_> {
    fn decode(&self, encoded: &str) -> Option<Vec<u8>> {
        // Modified from: [base32::decode](https://github.com/andreasots/base32/blob/eed2049099a52e51a29a1b4ed0502404481b194f/src/lib.rs#L141)

        if !encoded.is_ascii() {
            return None;
        }
        let encoded = encoded.as_bytes();

        let mut unpadded_codes_length = encoded.len();
        for i in 1..min(6, encoded.len()) + 1 {
            if encoded[encoded.len() - i] != b'=' {
                break;
            }
            unpadded_codes_length -= 1;
        }

        let output_length = unpadded_codes_length * 5 / 8;
        let mut data = Vec::with_capacity((output_length + 4) / 5 * 5);
        for codes in encoded.chunks(8) {
            let code_ids = {
                let mut buf = [0u8; 8];
                for (i, &code) in codes.iter().enumerate() {
                    let id = Base32Variant::get_code_id(code);
                    if id == 255 {
                        return None;
                    }
                    match self.code_id_to_byte5[id as usize] {
                        32.. => return None,
                        byte5 => buf[i] = byte5,
                    };
                }
                buf
            };

            data.push((code_ids[0] << 3) | (code_ids[1] >> 2));
            data.push((code_ids[1] << 6) | (code_ids[2] << 1) | (code_ids[3] >> 4));
            data.push((code_ids[3] << 4) | (code_ids[4] >> 1));
            data.push((code_ids[4] << 7) | (code_ids[5] << 2) | (code_ids[6] >> 3));
            data.push((code_ids[6] << 5) | code_ids[7]);
        }
        data.truncate(output_length);
        Some(data)
    }
}

pub const CROCKFORD: Base32Variant = Base32Variant::builder()
    .name("Crockford")
    .mapping(b"0123456789ABCDEFGHJKMNPQRSTVWXYZ")
    .mapping(b"          abcdefghjkmnpqrstvwxyz")
    .map_chars(b"Oo", 0)
    .map_chars(b"IiLl", 1)
    .build();

const RFC4648_BUILDER: Builder = Base32Variant::builder()
    .mapping(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567")
    .mapping(b"abcdefghijklmnopqrstuvwxyz      ");
pub const RFC4648: Base32Variant = RFC4648_BUILDER.name("RFC4648").build();
pub const RFC4648_PAD: Base32Variant = RFC4648_BUILDER.name("RFC4648 Padded").padding(true).build();

const RFC4648_HIGHER_BUILDER: Builder =
    Base32Variant::builder().mapping(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567");
pub const RFC4648_HIGHER: Base32Variant = RFC4648_HIGHER_BUILDER.name("RFC4648 higher").build();
pub const RFC4648_HIGHER_PAD: Base32Variant = RFC4648_HIGHER_BUILDER
    .name("RFC4648 higher Padded")
    .padding(true)
    .build();

const RFC4648_LOWER_BUILDER: Builder =
    Base32Variant::builder().mapping(b"abcdefghijklmnopqrstuvwxyz234567");
pub const RFC4648_LOWER: Base32Variant = RFC4648_LOWER_BUILDER.name("RFC4648 lower").build();
pub const RFC4648_LOWER_PAD: Base32Variant = RFC4648_LOWER_BUILDER
    .name("RFC4648 lower padded")
    .padding(true)
    .build();

const RFC4648_HEX_BUILDER: Builder =
    Base32Variant::builder().mapping(b"0123456789ABCDEFGHIJKLMNOPQRSTUV");
pub const RFC4648_HEX: Base32Variant = RFC4648_HEX_BUILDER.name("RFC4648 base32hex").build();
pub const RFC4648_HEX_PAD: Base32Variant = RFC4648_HEX_BUILDER
    .name("RFC4648 base32hex padded")
    .padding(true)
    .build();

const RFC4648_HEX_LOWER_BUILDER: Builder =
    Base32Variant::builder().mapping(b"0123456789abcdefghijklmnopqrstuv");
pub const RFC4648_HEX_LOWER: Base32Variant = RFC4648_HEX_LOWER_BUILDER
    .name("RFC4648 base32hex lower")
    .build();
pub const RFC4648_HEX_LOWER_PAD: Base32Variant = RFC4648_HEX_LOWER_BUILDER
    .name("RFC4648 base32hex lower padded")
    .padding(true)
    .build();

pub const Z: Base32Variant = Base32Variant::builder()
    .mapping(b"ybndrfg8ejkmcpqxot1uwisza345h769")
    .name("z-based-32")
    .build();

const YIDU_BUILDER: Builder = Base32Variant::builder()
    .mapping(b"0123456789ABCDFGHIJKMNPQSTUVWXYZ")
    .mapping(b"          abcdfghijkmnpqstuvwxyz")
    .map_chars(b"Oo", 0);
pub const YIDU: Base32Variant = YIDU_BUILDER.name("YiDu").build();
pub const YIDU_PAD: Base32Variant = YIDU_BUILDER.name("YiDu padded").padding(true).build();
