use crate::base32::Base32Variant;

/// Use [`Base32Variant::builder`] to construct.
pub struct Builder<'a>(pub(crate) Base32Variant<'a>);

impl<'a> Builder<'a> {
    const _ALLOWED_CHARS: &'static [u8; 75] =
        b"0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz";

    pub const fn build(self) -> Base32Variant<'a> {
        {
            let mut i = 0;
            while i < self.0.byte5_to_default_code.len() {
                let ch = self.0.byte5_to_default_code[i];
                if Base32Variant::get_code_id(ch) == 255 {
                    panic!("Found invalid char in self.bin_char");
                }
                i += 1;
            }
        }
        {
            let mut i = 0;
            let mut map = [false; 32];
            while i < self.0.code_id_to_byte5.len() {
                let byte5 = self.0.code_id_to_byte5[i];
                if byte5 < 32 {
                    map[byte5 as usize] = true;
                }
                i += 1;
            }
            i = 0;
            while i < 32 {
                if !map[i] {
                    panic!("Not all bytes can be decoded to.");
                }
                i += 1;
            }
        }

        self.0
    }

    pub const fn name(mut self, name: &'a str) -> Self {
        self.0.name = Some(name);
        self
    }

    pub const fn padding(mut self, padding: bool) -> Self {
        self.0.padding = padding;
        if padding {
            self.0.code_id_to_byte5[(b'=' - b'0') as usize] = 0;
        }
        self
    }

    pub const fn mapping(mut self, chars: &[u8; 32]) -> Self {
        let mut i: u8 = 0;
        while i < 32 {
            let ch = chars[i as usize];
            let id = Base32Variant::get_code_id(ch);

            if id != 255 {
                if self.0.byte5_to_default_code[i as usize] == 255 {
                    self.0.byte5_to_default_code[i as usize] = ch;
                }
                self.0.code_id_to_byte5[id as usize] = i;
            }

            i += 1;
        }
        self
    }

    pub const fn map_chars(mut self, chars: &[u8], code: u8) -> Self {
        let mut i = 0;
        while i < chars.len() {
            let ch = chars[i];
            let id = Base32Variant::get_code_id(ch);
            if id != 255 {
                self.0.code_id_to_byte5[id as usize] = code;
            }
            i += 1;
        }
        self
    }
}
