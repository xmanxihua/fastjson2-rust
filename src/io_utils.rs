use std::cmp::min;

pub fn encode_utf8(src: &Vec<u16>, mut offset: usize, len: i32, dst: &mut Vec<i8>, mut dp: i32) -> i32 {
    let sl = offset + len as usize;
    let dl_ascii = dp + min(len, dst.len() as i32);

    // ASCII only optimized loop
    while dp < dl_ascii && src[offset] < 0080 {
        dst[dp as usize]  = src[offset] as i8;
        dp += 1;
        offset += 1;
    }

    while offset < sl {
        let c  = src[offset] as u32;
        offset += 1;
        if c < 0x80  {
            // Have at most seven bits
            dst[dp as usize] = c as i8;
            dp += 1;
        } else if c < 0x800u32 {
            // 2 bytes, 11 bits
            dst[dp as usize] = (0xc0 | (c >> 6)) as i8;
            dst[(dp + 1) as usize] = (0x80 | (c & 0x3f)) as i8;
            dp += 2;
        } else if c >= 0xD800u32 && c < (0xDFFFu32 + 1) { //Character.isSurrogate(c) but 1.7
            let mut uc:i32;
            let ip = offset - 1;
            if c < 0xDBFFu32 { // Character.isHighSurrogate(c)
                if sl - ip < 2 {
                    uc = -1;
                } else {
                    let d = src[ip + 1] as u32;
                    // d >= '\uDC00' && d < ('\uDFFF' + 1)
                    if d >= 0xDC00u32 && d < (0xDFFFu32 + 1) { // Character.isLowSurrogate(d)
                        uc = ((c << 10) + d) as i32 + (0x010000 - ((0xD800i32) << 10) - 0xDC00i32); // Character.toCodePoint(c, d)
                    } else {
                        //                            throw new JSONException("encode_utf8 error", new MalformedInputException(1));
                        dst[dp as usize] = b'?' as i8;
                        dp += 1;
                        continue;
                    }
                }
            } else {
                //
                // Character.isLowSurrogate(c)
                dst[dp as usize] = b'?' as i8;
                dp += 1;
                continue;
                //                        throw new JSONException("encode_utf8 error", new MalformedInputException(1));
            }

            if (uc < 0) {
                dst[dp as usize] = b'?' as i8;
                dp += 1;
            } else {
                dst[dp as usize] = (0xf0 | ((uc >> 18))) as i8;
                dst[(dp + 1) as usize] = (0x80 | ((uc >> 12) & 0x3f)) as i8;
                dst[(dp + 2)as usize] = (0x80 | ((uc >> 6) & 0x3f)) as i8;
                dst[(dp + 3)as usize] = (0x80 | (uc & 0x3f)) as i8;
                dp += 4;
                offset += 1; // 2 chars
            }
        } else {
            // 3 bytes, 16 bits
            dst[dp as usize] = (0xe0 | ((c >> 12))) as i8;
            dst[(dp + 1) as usize] = (0x80 | ((c >> 6) & 0x3f)) as i8;
            dst[(dp + 2)as usize] = (0x80 | (c & 0x3f)) as i8;
            dp += 3;
        }
    }
    return dp;
}