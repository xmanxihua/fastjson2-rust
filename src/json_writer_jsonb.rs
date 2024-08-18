use std::io::{Bytes, SeekFrom};
use std::str::EncodeUtf16;
use crate::{constants, io_utils};
use crate::constants::{BC_CHAR, BC_INT32, BC_INT32_BYTE_ZERO, BC_INT32_NUM_MAX, BC_INT32_NUM_MIN, BC_INT32_SHORT_ZERO, BC_NULL, BC_STR_ASCII, BC_STR_ASCII_FIX_MIN, BC_STR_UTF8, features, INT32_BYTE_MAX, INT32_BYTE_MIN, INT32_SHORT_MAX, INT32_SHORT_MIN, STR_ASCII_FIX_LEN, BIG_ENDIAN};
pub struct JSONWriterJSONB {
    bytes: Vec<i8>,
    off: i32,
    features: u64,
}

impl JSONWriterJSONB {
    pub fn new() -> Self {
        JSONWriterJSONB {
            bytes: vec![],
            off: 0,
            features: 0,
        }
    }
    pub fn write_none(&mut self) {
        if self.off == self.bytes.len() as i32 {
            self.ensure_capacity(self.off + 1);
        }
        self.bytes[self.off as usize] = BC_NULL;
        self.off += 1;
    }
    pub fn write_raw(&mut self, b: i8) {
        if self.off == self.bytes.len() as i32 {
            self.ensure_capacity(self.off + 1);
        }
        self.bytes[self.off as usize] = b;
        self.off += 1;
    }

    pub fn write_option_name(&mut self, name: Option<String>) {
        self.write_option_string_utf16(name);
    }

    pub fn write_option_string_utf16(&mut self, s: Option<String>) {
        if let Some(s) = s {
            let chars: Vec<u16> = s.encode_utf16().collect();
            let ascii = s.is_ascii();
            self.write_utf16_vec(&chars, ascii);
        } else {
            self.write_none();
        }
    }
    fn write_utf16_vec(&mut self, chars: &Vec<u16>, ascii: bool) {
        let mut off = self.off;
        let strlen = chars.len() as i32;
        if strlen < STR_ASCII_FIX_LEN {
            if ascii {
                let min_capacity = off + 1 + strlen;
                if min_capacity - self.bytes.len() as i32 > 0 {
                    self.ensure_capacity(min_capacity);
                }

                self.bytes[off as usize] = strlen as i8 + BC_STR_ASCII_FIX_MIN;
                for ch in chars.iter() {
                    self.bytes[off as usize] = *ch as i8;
                    off += 1;
                }

                self.off = off;
                return;
            }
        }


        let mut min_capacity = if ascii { strlen } else { strlen * 3 } + off + 5 /*max str len*/ + 1;

        if (min_capacity - self.bytes.len() as i32 > 0) {
            self.ensure_capacity(min_capacity);
        }

        if ascii {
            if strlen <= STR_ASCII_FIX_LEN {
                self.bytes[off as usize] = (strlen + BC_STR_ASCII_FIX_MIN as i32) as i8;
                off += 1;
            } else if strlen <= INT32_BYTE_MAX {
                Self::put_string_size_small(&mut self.bytes, off, strlen);
                off += 3;
            } else {
                off += Self::put_string_size_large(&mut self.bytes, off, strlen);
            }
            for i in 0..strlen {
                self.bytes[off as usize] = chars[i as usize] as i8;
                off += 1;
            }
        } else {
            let max_size = strlen * 3;
            let len_byte_cnt = Self::size_of_int(max_size);
            self.ensure_capacity(off + max_size + len_byte_cnt + 1);
            let result = io_utils::encode_utf8(&chars, 0, strlen, &mut self.bytes, off + len_byte_cnt + 1);

            let utf8len = result - off - len_byte_cnt - 1;
            let utf8len_byte_cnt = Self::size_of_int(utf8len);
            if len_byte_cnt != utf8len_byte_cnt {
                let (left, right) = self.bytes.split_at_mut((off + utf8len_byte_cnt + 1) as usize);
                let x = &left[(off + len_byte_cnt + 1) as usize..(off + len_byte_cnt + 1 + utf8len) as usize];
                right.copy_from_slice(&x);
            }
            let bytes = &mut self.bytes;
            bytes[off as usize] = BC_STR_UTF8;
            off += 1;
            if (utf8len >= BC_INT32_NUM_MIN as i32 && utf8len <= BC_INT32_NUM_MAX as i32) {
                bytes[off as usize] = utf8len as i8;
                off += 1;
            } else if (utf8len >= INT32_BYTE_MIN && utf8len <= INT32_BYTE_MAX) {
                bytes[off as usize] = (BC_INT32_BYTE_ZERO as i32 + (utf8len >> 8)) as i8;
                bytes[(off + 1) as usize] = utf8len as i8;
                off += 2;
            } else {
                off += Self::write_int32_from_off(bytes, off, utf8len);
            }
            off += utf8len;
        }
        self.off = off;
    }

    pub fn write_int32_from_off(bytes: &mut Vec<i8>, mut off: i32, val: i32) -> i32 {
        if (val >= BC_INT32_NUM_MIN as i32 && val <= BC_INT32_NUM_MAX as i32) {
            bytes[off as usize] = val as i8;
            return 1;
        } else if (val >= INT32_BYTE_MIN && val <= INT32_BYTE_MAX) {
            bytes[off as usize] = (BC_INT32_BYTE_ZERO as i32 + (val >> 8)) as i8;
            bytes[(off + 1) as usize] = (val) as i8;
            return 2;
        } else if (val >= INT32_SHORT_MIN && val <= INT32_SHORT_MAX) {
            bytes[off as usize] = (BC_INT32_SHORT_ZERO as i32 + (val >> 16)) as i8;
            bytes[(off + 1) as usize] = (val >> 8) as i8;
            bytes[(off + 2) as usize] = (val) as i8;
            return 3;
        } else {
            bytes[off as usize] = BC_INT32;
            let vec = Self::i32_to_array(val);
            for (index, v) in vec.iter().enumerate() {
                bytes[off as usize] = *v;
                off += 1;
            }

            return 5;
        }
    }


    fn put_string_size_small(bytes: &mut Vec<i8>, off: i32, val: i32) {
        bytes[off as usize] = BC_STR_ASCII;
        bytes[(off + 1) as usize] = (BC_INT32_BYTE_ZERO as i32 + (val >> 8)) as i8;
        bytes[(off + 2) as usize] = val as i8;
    }

    fn put_string_size_large(bytes: &mut Vec<i8>, off: i32, strlen: i32) -> i32 {
        if strlen <= INT32_SHORT_MAX {
            bytes[off as usize] = BC_STR_ASCII;
            bytes[(off + 1) as usize] = (BC_INT32_SHORT_ZERO as i32 + (strlen >> 16)) as i8;
            bytes[(off + 2) as usize] = (strlen >> 8) as i8;
            bytes[(off + 3) as usize] = strlen as i8;
            return 4;
        }

        bytes[off as usize] = BC_STR_ASCII;
        bytes[(off + 1) as usize] = BC_INT32;

        let vec = Self::i32_to_array(strlen);
        for v in vec.iter().enumerate() {
            bytes[off as usize + 2 + v.0] = *v.1;
        }
        return 6;
    }

    fn size_of_int(i: i32) -> i32 {
        if (i >= BC_INT32_NUM_MIN as i32 && i <= BC_INT32_NUM_MAX as i32) {
            return 1;
        }
        if (i >= INT32_BYTE_MIN && i <= INT32_BYTE_MAX) {
            return 2;
        }
        if (i >= INT32_SHORT_MIN && i <= INT32_SHORT_MAX) {
            return 3;
        }

        return 5;
    }


    pub fn write_u16(&mut self, ch: u16) {
        if (self.off as usize == self.bytes.len()) {
            self.ensure_capacity(self.off + 1);
        }
        self.bytes[self.off as usize] = BC_CHAR;
        self.write_int32(ch as i32);
    }


    pub fn write_option_i32(&mut self, val: Option<i32>) {
        if let Some(val) = val {
            self.write_int32(val);
        } else {
            let min_capacity = self.off + 5;
            if min_capacity as usize >= self.bytes.len() {
                self.ensure_capacity(min_capacity);
            }
            let mut off = self.off;
            let mut bytes = &mut self.bytes;

            if (self.features & (features::NULL_AS_DEFAULT_VALUE | features::WRITE_NULL_NUMBER_AS_ZERO)) == 0 {
                bytes[off as usize] = BC_NULL;
            } else {
                bytes[off as usize] = 0;
            }
            self.off += 1;
        }
    }

    pub fn write_int32(&mut self, val: i32) {
        let min_capacity = self.off + 5;
        if min_capacity as usize >= self.bytes.len() {
            self.ensure_capacity(min_capacity);
        }
        let mut off = self.off;
        let mut bytes = &mut self.bytes;
        let mut size = 0;
        if val >= BC_INT32_NUM_MIN as i32 && val <= BC_INT32_NUM_MAX as i32 {
            bytes[off as usize] = val as i8;
            size = 1;
        } else if (val >= INT32_BYTE_MIN as i32 && val <= INT32_BYTE_MAX as i32) {
            bytes[off as usize] = BC_INT32_BYTE_ZERO + (val >> 8) as i8;
            bytes[(off + 1) as usize] = val as i8;
            size = 2;
        } else if (val >= INT32_SHORT_MIN && val <= INT32_SHORT_MAX) {
            bytes[off as usize] = BC_INT32_SHORT_ZERO + (val >> 16) as i8;
            bytes[(off + 1) as usize] = (val >> 8) as i8;
            bytes[(off + 2) as usize] = (val) as i8;
            size = 3;
        } else {
            bytes[off as usize] = BC_INT32;
            size += 1;
            for (index, v) in Self::i32_to_array(val).iter().enumerate() {
                bytes[off as usize + index + 1] = *v;
                size += 1;
            }
        }
        self.off += size;
    }

    pub fn get_bytes(mut self) -> Vec<i8> {
        self.bytes[..self.off as usize].to_vec()
    }

    fn ensure_capacity(&mut self, capacity: i32) {
        self.bytes.resize(capacity as usize, 0);
    }

    fn reverse_bytes(i: i32) -> i32 {
        return (i << 24)
            | ((i & 0xff00) << 8)
            | ((i >> 8) & 0xff00)
            | (i >> 24);
    }
    fn i32_to_array(i: i32) -> Vec<i8> {
        if BIG_ENDIAN {
            i.to_be_bytes().map(|x| x as i8).into_iter().collect()
        } else {
            i.to_le_bytes().map(|x| x as i8).into_iter().collect()
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::json_writer_jsonb::JSONWriterJSONB;
    use super::*;

    #[test]
    fn it_works() {
        let mut jsonb = JSONWriterJSONB::new();
        let vec = JSONWriterJSONB::i32_to_array(0x010203);
        println!("{:?}", vec);
    }
}
