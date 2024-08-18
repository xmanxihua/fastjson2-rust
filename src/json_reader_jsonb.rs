use core::ascii;
use std::borrow::Cow;
use std::string::FromUtf8Error;
use crate::constants::{BC_INT32, BC_INT32_BYTE_MAX, BC_INT32_BYTE_MIN, BC_INT32_BYTE_ZERO, BC_INT32_NUM_MAX, BC_INT32_NUM_MIN, BC_INT32_SHORT_MAX, BC_INT32_SHORT_MIN, BC_INT32_SHORT_ZERO, BC_NULL, BC_STR_ASCII, BC_STR_ASCII_FIX_MIN, BIG_ENDIAN};
use crate::jsonb::type_name;

pub struct JSONReaderJSONB {
    bytes: Vec<u8>,
    offset: usize,
    strtype: i8,
    str_begin: usize,
    strlen: usize,
    context: Context,
}

struct Context {
    features: u64,
}

mod feature {
    pub const FieldBased: u64 = 1;
    pub const IgnoreNoneSerializable: u64 = 1 << 1;
    /**
     * @since 2.0.14
     */
    pub const ErrorOnNoneSerializable: u64 = 1 << 2;
    pub const SupportArrayToBean: u64 = 1 << 3;
    pub const InitStringFieldAsEmpty: u64 = 1 << 4;
    /**
     * It is not safe to explicitly turn on autoType, it is recommended to use AutoTypeBeforeHandler
     */
    pub const SupportAutoType: u64 = 1 << 5;
    pub const SupportSmartMatch: u64 = 1 << 6;
    pub const UseNativeObject: u64 = 1 << 7;
    pub const SupportClassForName: u64 = 1 << 8;
    pub const IgnoreSetNullValue: u64 = 1 << 9;
    pub const UseDefaultConstructorAsPossible: u64 = 1 << 10;
    pub const UseBigDecimalForFloats: u64 = 1 << 11;
    pub const UseBigDecimalForDoubles: u64 = 1 << 12;
    pub const ErrorOnEnumNotMatch: u64 = 1 << 13;
    pub const TrimString: u64 = 1 << 14;
    pub const ErrorOnNotSupportAutoType: u64 = 1 << 15;
    pub const DuplicateKeyValueAsArray: u64 = 1 << 16;
    pub const AllowUnQuotedFieldNames: u64 = 1 << 17;
    pub const NonStringKeyAsString: u64 = 1 << 18;
    /**
     * @since 2.0.13
     */
    pub const Base64StringAsByteArray: u64 = 1 << 19;

    /**
     * @since 2.0.16
     */
    pub const IgnoreCheckClose: u64 = 1 << 20;
    /**
     * @since 2.0.20
     */
    pub const ErrorOnNullForPrimitives: u64 = 1 << 21;

    /**
     * @since 2.0.20
     */
    pub const NullOnError: u64 = 1 << 22;

    /**
     * @since 2.0.21
     */
    pub const IgnoreAutoTypeNotMatch: u64 = 1 << 23;

    /**
     * @since 2.0.24
     */
    pub const NonZeroNumberCastToBooleanAsTrue: u64 = 1 << 24;

    /**
     * @since 2.0.40
     */
    pub const IgnoreNullPropertyValue: u64 = 1 << 25;

    /**
     * @since 2.0.42
     */
    pub const ErrorOnUnknownProperties: u64 = 1 << 26;
}


impl JSONReaderJSONB {
    pub fn read_string(&mut self) -> Result<Option<String>, String> {
        let bytes: &Vec<u8> = &self.bytes;
        let strtype = bytes[self.offset] as i8;
        self.offset += 1;
        self.strtype = strtype;
        if strtype == BC_NULL {
            return Ok(None);
        }

        self.str_begin = self.offset;
        let mut astr: Option<String> = None;
        let mut ascii = false;
        if (strtype >= BC_STR_ASCII_FIX_MIN && strtype <= BC_STR_ASCII) {
            ascii = true;
            let strlen: i32;
            if (strtype == BC_STR_ASCII) {
                let str_type = bytes[self.offset] as i8;
                if (str_type >= BC_INT32_NUM_MIN && str_type <= BC_INT32_NUM_MAX) {
                    self.offset += 1;
                    strlen = str_type as i32;
                } else {
                    strlen = self.read_length()? as i32;
                }
                self.str_begin = self.offset;
            } else {
                strlen = (strtype - BC_STR_ASCII_FIX_MIN) as i32;
            }
            self.strlen = strlen as usize;

            if (strlen >= 0) {
                astr = match String::from_utf8(Vec::from_iter(self.bytes[self.offset..self.offset + strlen as usize])) {
                    Ok(s) => Some(s),
                    Err(_) => None
                };
                self.offset += strlen as usize;
            }

            if (astr.is_some()) {
                if (self.context.features & feature::TrimString) != 0 {
                    astr = astr.map(|x| x.trim().into());
                }
                return Ok(astr);
            }
        }

        return readStringNonAscii(None, ascii);
    }

    fn  readStringNonAscii(astr:String, ascii:bool)->String {
    let mut charset = Charset
    if (ascii) {
    charset = ISO_8859_1;
    } else if (strtype == BC_STR_UTF8) {
    str = readStringUTF8();
    charset = StandardCharsets.UTF_8;
    } else if (strtype == BC_STR_UTF16) {
    strlen = readLength();
    strBegin = offset;
    charset = StandardCharsets.UTF_16;
    } else if (strtype == BC_STR_UTF16LE) {
    str = readUTF16LE();
    charset = StandardCharsets.UTF_16LE;
    } else if (strtype == BC_STR_UTF16BE) {
    str = readUTF16BE();
    if (str != null) {
    return str;
    }
    charset = StandardCharsets.UTF_16BE;
    } else if (strtype == BC_STR_GB18030) {
    readGB18030();
    charset = GB18030;
    } else {
    return readStringTypeNotMatch();
    }

    if (str != null) {
    if ((context.features & Feature.TrimString.mask) != 0) {
    str = str.trim();
    }
    return str;
    }

    return readString(charset);
    }

    fn get_int3(bytes: &Vec<u8>, offset: usize, r#type: i32) -> u32 {
        ((r#type - BC_INT32_SHORT_ZERO as i32) << 16) as u32
            + ((bytes[offset] & 0xFF) << 8) as u32
            + (bytes[offset + 1] & 0xFF) as u32
    }

    pub fn read_length(&mut self) -> Result<u32, String> {
        let r#type = self.bytes[self.offset] as i8;
        self.offset += 1;
        if (r#type >= BC_INT32_NUM_MIN && r#type <= BC_INT32_NUM_MAX) {
            return Ok(r#type as u32);
        }

        if (r#type >= BC_INT32_BYTE_MIN && r#type <= BC_INT32_BYTE_MAX) {
            let r = (((r#type - BC_INT32_BYTE_ZERO) as u32) << 8) + (self.bytes[self.offset] as u32 & 0xFF);
            self.offset += 1;
            return Ok(r);
        }

        if (r#type >= BC_INT32_SHORT_MIN && r#type <= BC_INT32_SHORT_MAX) {
            let len = Self::get_int3(&self.bytes, self.offset, r#type as i32);
            self.offset += 2;
            return Ok(len);
        }

        if (r#type == BC_INT32) {
            let len = Self::getInt(&self.bytes, self.offset);
            self.offset += 4;
            if (len > 1024 * 1024 * 256) {
                return Err("input length overflow".into());
            }
            return Ok(len as u32);
        }

        Err("not support length type : ".into() + type_name(r#type))
    }

    fn getInt(bytes: &Vec<u8>, offset: usize) -> i32 {
        let bytes: [u8; 4] = (&bytes[offset..offset + 4]).into_iter().collect();
        if BIG_ENDIAN {
            return i32::from_ne_bytes(bytes);
        }
        i32::from_le_bytes(bytes)
    }
}