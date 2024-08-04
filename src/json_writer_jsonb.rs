use std::io::{Bytes, SeekFrom};
use crate::constants;
use crate::constants::{BC_CHAR, BC_INT32, BC_INT32_BYTE_ZERO, BC_INT32_NUM_MAX, BC_INT32_NUM_MIN, BC_INT32_SHORT_ZERO, BC_NULL, BC_STR_ASCII_FIX_MIN, features, INT32_BYTE_MAX, INT32_BYTE_MIN, INT32_SHORT_MAX, INT32_SHORT_MIN, STR_ASCII_FIX_LEN};
const BIG_ENDIAN: bool = cfg!(target_endian = "little");
pub struct JSONWriterJSONB {
    bytes: Vec<i8>,
    off: usize,
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

    pub fn write_raw(&mut self, b: i8) {
        if (self.off == self.bytes.len()) {
            self.ensure_capacity(self.off + 1);
        }
        self.bytes[self.off] = b;
        self.off += 1;
    }

    pub fn write_option_name(&mut self, name:Option<String>) {
        self.write_option_string(name);
    }

    pub fn write_option_string(&mut self, s:Option<String>) {
    // if (str == null) {
    // writeNull();
    // return;
    // }

        if let Some(s) = s {
            let mut off =self.off;
            let mut ascii  = true;
            let chars = s.chars();
            let strlen = chars.count();
            if strlen < STR_ASCII_FIX_LEN as usize {
                let minCapacity = off + 1 + strlen;
                if (minCapacity - self.bytes.len() > 0) {
                    self.ensure_capacity(minCapacity);
                }

                self.bytes[off] =  strlen as i8 + BC_STR_ASCII_FIX_MIN ;
                for ch in s.chars() {


                    if (ch as u64 > 0x00FF) {
                        ascii = false;
                        break;
                    }
                    self.bytes[off] =  ch as i8;
                    off+=1;
                }

                if (ascii) {
                    self.off = off;
                    return;
                } else {
                    off = self.off;
                }
            }

            {
                let chars = s.chars();
                let upperBound = strlen & !3;
                let mut j = 0;
                for i in (0..upperBound).step_by(4) {
                    j = i;
                let c0 = chars[i];
                let c1 = chars[i + 1];
                let c2 = chars[i + 2];
                let c3 = chars[i + 3];
                if (c0 > 0x00FF || c1 > 0x00FF || c2 > 0x00FF || c3 > 0x00FF) {
                    ascii = false;
                    break;
                }
                }
                j+=4;
                if (ascii) {
                    for i in j ..strlen {
                        if (chars[i] > 0x00FF) {
                            ascii = false;
                            break;
                        }
                    }
                }
            }

            let mut minCapacity = if ascii {strlen} else { strlen * 3} + off + 5 /*max str len*/ + 1;

        if (minCapacity - self.bytes.len() > 0) {
        self. ensure_capacity(minCapacity);
        }

        if (ascii) {
        if strlen <= STR_ASCII_FIX_LEN as usize{
        self.bytes[off] =  (strlen + BC_STR_ASCII_FIX_MIN as usize) as i8;
        } else if strlen <= INT32_BYTE_MAX as usize {
        // putStringSizeSmall(bytes, off, strlen);
        off += 3;
        } else {
        // off += putStringSizeLarge(bytes, off, strlen);
        }
            let chars = s.chars();
        for i in 0..strlen {
        self.bytes[off] =  chars[i];
        }
        } else {
        let maxSize = strlen * 3;
        let lenByteCnt = 0;//sizeOfInt(maxSize);
        self.ensure_capacity(off + maxSize + lenByteCnt + 1);
        let result = 0;//IOUtils.encodeUTF8(chars, 0, chars.length, bytes, off + lenByteCnt + 1);

        let utf8len = result - off - lenByteCnt - 1;
        let utf8lenByteCnt = 0;//sizeOfInt(utf8len);
        if (lenByteCnt != utf8lenByteCnt) {
        System.arraycopy(bytes, off + lenByteCnt + 1, bytes, off + utf8lenByteCnt + 1, utf8len);
        }
        final byte[] bytes = this.bytes;
        bytes[off++] = BC_STR_UTF8;
        if (utf8len >= BC_INT32_NUM_MIN && utf8len <= BC_INT32_NUM_MAX) {
        bytes[off++] = (byte) utf8len;
        } else if (utf8len >= INT32_BYTE_MIN && utf8len <= INT32_BYTE_MAX) {
        bytes[off] = (byte) (BC_INT32_BYTE_ZERO + (utf8len >> 8));
        bytes[off + 1] = (byte) (utf8len);
        off += 2;
        } else {
        off += writeInt32(bytes, off, utf8len);
        }
        off += utf8len;
        }
        this.off = off;
        }


    }




    pub fn write_u16(&mut self, ch: u16) {
        if (self.off == self.bytes.len()) {
            self.ensure_capacity(self.off + 1);
        }
        self.bytes[self.off] = BC_CHAR;
        self.write_int32(ch as i32);
    }


    pub fn write_option_i32(&mut self, val: Option<i32>) {
        if let Some(val) = val {
            self.write_int32(val);
        } else {
            let min_capacity = self.off + 5;
            if min_capacity >= self.bytes.len() {
                self.ensure_capacity(min_capacity);
            }
            let mut off = self.off;
            let mut bytes = &mut self.bytes;

            if (self.features & (features::NULL_AS_DEFAULT_VALUE | features::WRITE_NULL_NUMBER_AS_ZERO)) == 0 {
                bytes[off] = BC_NULL;
            } else {
                bytes[off] = 0;
            }
            self.off += 1;
        }
    }

    pub fn write_int32(&mut self, val: i32) {
        let min_capacity = self.off + 5;
        if min_capacity >= self.bytes.len() {
            self.ensure_capacity(min_capacity);
        }
        let mut off = self.off;
        let mut bytes = &mut self.bytes;
        let mut size = 0;
        if val >= BC_INT32_NUM_MIN as i32 && val <= BC_INT32_NUM_MAX as i32 {
            bytes[off] = val as i8;
            size = 1;
        } else if (val >= INT32_BYTE_MIN as i32 && val <= INT32_BYTE_MAX as i32) {
            bytes[off] = BC_INT32_BYTE_ZERO + (val >> 8) as i8;
            bytes[off + 1] = val as i8;
            size = 2;
        } else if (val >= INT32_SHORT_MIN && val <= INT32_SHORT_MAX) {
            bytes[off] = BC_INT32_SHORT_ZERO + (val >> 16) as i8;
            bytes[off + 1] = (val >> 8) as i8;
            bytes[off + 2] = (val) as i8;
            size = 3;
        } else {
            bytes[off] = BC_INT32;
            size += 1;
            for (index, v) in Self::i32_to_array(val).iter().enumerate() {
                bytes[off + index + 1] = *v;
                size += 1;
            }
        }
        self.off += size;
    }

    pub fn get_bytes(mut self) -> Vec<i8> {
        self.bytes[..self.off].to_vec()
    }

    fn ensure_capacity(&mut self, capacity: usize) {
        self.bytes.resize(capacity, 0);
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
