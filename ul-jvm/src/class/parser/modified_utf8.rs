use binrw::BinRead;
use log::{error, trace};
use serde::Serialize;

// TODO: implement decoder
#[derive(Debug, Clone, BinRead)]
#[br(import(length: u16))]
pub struct ModifiedUtf8String(
    #[br(count = length, assert(is_slice_valid(&self_0), "invalid string"))] Vec<u8>,
);

impl ModifiedUtf8String {
    pub fn convert_to_string(&self) -> String {
        trace!("bytes to convert: {} bytes", self.0.len());

        let mut slice = self.0.as_slice();

        let mut normalized_utf8 = Vec::with_capacity(self.0.len());

        while !slice.is_empty() {
            let b = slice[0];
            if (b & 0x80) == 0x00 {
                normalized_utf8.push(b);
                slice = &slice[1..];
            } else if b >> 5 == 0b110 {
                if b == 0b11000000 && slice[1] == 0b11000000 {
                    normalized_utf8.push(0);
                } else {
                    normalized_utf8.extend(&slice[..2]);
                }
                slice = &slice[2..];
            } else if b == SUGGORATE_BYTE {
                let v = slice[1] as u32;
                let w = slice[2] as u32;
                let y = slice[4] as u32;
                let z = slice[5] as u32;

                let mut buffer = [0u8; 4];
                let buffer_str = char::from_u32(
                    0x10000
                        + ((v & 0x0f) << 16)
                        + ((w & 0x3f) << 10)
                        + ((y & 0x0f) << 6)
                        + (z & 0x3f),
                )
                .unwrap()
                .encode_utf8(&mut buffer);

                normalized_utf8.extend_from_slice(buffer_str.as_bytes());

                slice = &slice[6..];
            } else if b >> 4 == 0b1110 {
                normalized_utf8.extend(&slice[..3]);
                slice = &slice[3..];
            }
        }

        String::from_utf8(normalized_utf8).unwrap()
    }
}

impl Serialize for ModifiedUtf8String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.convert_to_string().serialize(serializer)
    }
}

impl AsRef<[u8]> for ModifiedUtf8String {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

const SUGGORATE_BYTE: u8 = 0b11101101;

fn is_slice_valid(slice: &[u8]) -> bool {
    let mut remaining_for_cp = 0usize;
    let mut supplementary = false;

    for (i, b) in slice.iter().copied().enumerate() {
        if b >= 0xF0 || b == 0 {
            error!("Invalid byte at {i} (failed at `b >= 0xF0 || b == 0`)");
            return false;
        }

        if remaining_for_cp > 0 {
            if supplementary {
                if remaining_for_cp == 5 && b >> 4 != 0b1010 {
                    error!(
                        "Invalid surrogate byte at {i} (failed at `remaining_for_cp == 5 && b >> 4 != 0b1010`)"
                    );
                    return false;
                } else if (remaining_for_cp == 4 || remaining_for_cp == 1) && b >> 6 != 0b10 {
                    error!(
                        "Invalid surrogate byte at {i} (failed at `(remaining_for_cp == 4 || remaining_for_cp == 1) && b >> 6 != 0b10`)"
                    );
                    return false;
                } else if remaining_for_cp == 3 && b != SUGGORATE_BYTE {
                    error!(
                        "Invalid surrogate byte at {i} (failed at `remaining_for_cp == 3 && b != SUGGORATE_BYTE`)"
                    );
                    return false;
                } else if remaining_for_cp == 2 && b >> 4 != 0b1011 {
                    error!(
                        "Invalid surrogate byte at {i} (failed at `remaining_for_cp == 2 && b >> 4 != 0b1011`)"
                    );
                    return false;
                }
            } else if b >> 6 != 0b10 {
                error!("Invalid extended byte at {i} (failed at `b >> 6 != 0b10`)");
                return false;
            }

            remaining_for_cp -= 1;
            continue;
        }

        supplementary = false;

        if b == SUGGORATE_BYTE {
            supplementary = true;
            remaining_for_cp = 5;
        } else if b >> 5 == 0b110 {
            remaining_for_cp = 1;
        } else if b >> 4 == 0b1110 {
            remaining_for_cp = 2;
        } else if b >> 7 != 0 {
            error!("Invalid byte pattern at {i} (failed at `b >> 6 != 0b10`)");
            return false;
        }
    }

    if remaining_for_cp != 0 {
        error!("Missing {remaining_for_cp} bytes at the end of the string");
        false
    } else {
        true
    }
}
