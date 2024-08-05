use std::cmp::min;

pub fn encode_utf8(src: &Vec<char>, mut offset: usize, len: i32, dst: &mut Vec<i8>, mut dp: i32) -> i32 {
    let sl = offset + len as usize;
    let dl_ascii = dp + min(len, dst.len() as i32);

    // ASCII only optimized loop
    while dp < dl_ascii && src[offset] < '\u{0080}' {
        dst[dp] = src[offset];
        dp += 1;
        offset += 1;
    }

    while offset < sl {
        let c  = src[offset] as u32;
        offset += 1;
        if c < 0x80  {
            // Have at most seven bits
            dst[dp] = c as i8;
            dp += 1;
        } else if c < 0x800u32 {
            // 2 bytes, 11 bits
            dst[dp] = (0xc0 | (c >> 6)) as i8;
            dst[dp + 1] = (0x80 | (c & 0x3f)) as i8;
            dp += 2;
        } else if c >= '\u{D800}' as u32 && c < ('\u{DFFF}' as u32 + 1) { //Character.isSurrogate(c) but 1.7
            let mut uc:i32;
            let ip = offset - 1;
            if c < '\u{DBFF}' as u32 { // Character.isHighSurrogate(c)
                if (sl - ip < 2) {
                    uc = -1;
                } else {
                    let d = src[ip + 1] as u32;
                    // d >= '\uDC00' && d < ('\uDFFF' + 1)
                    if (d >= '\u{DC00}' as u32 && d < ('\u{DFFF}' as u32 + 1)) { // Character.isLowSurrogate(d)
                        uc = (((c << 10) + d) as i32 + (0x010000 - (('\u{D800}' as i32) << 10) - '\u{DC00}' as i32)) as i32; // Character.toCodePoint(c, d)
                    } else {
                        //                            throw new JSONException("encode_utf8 error", new MalformedInputException(1));
                        dst[dp] = b'?';
                        dp += 1;
                        continue;
                    }
                }
            } else {
                //
                // Character.isLowSurrogate(c)
                dst[dp] = b'?';
                dp += 1;
                continue;
                //                        throw new JSONException("encode_utf8 error", new MalformedInputException(1));
            }

            if (uc < 0) {
                dst[dp] = b'?';
                dp += 1;
            } else {
                dst[dp] = (0xf0 | ((uc >> 18))) as i8;
                dst[dp + 1] = (0x80 | ((uc >> 12) & 0x3f)) as i8;
                dst[dp + 2] = (0x80 | ((uc >> 6) & 0x3f)) as i8;
                dst[dp + 3] = (0x80 | (uc & 0x3f)) as i8;
                dp += 4;
                offset += 1; // 2 chars
            }
        } else {
            // 3 bytes, 16 bits
            dst[dp] = (0xe0 | ((c >> 12))) as i8;
            dst[dp + 1] = (0x80 | ((c >> 6) & 0x3f)) as i8;
            dst[dp + 2] = (0x80 | (c & 0x3f)) as i8;
            dp += 3;
        }
    }
    return dp;
}