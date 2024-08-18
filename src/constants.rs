pub const BC_CHAR: i8 = -112;                    // 0x90
pub const BC_BINARY: i8 = -111;                  // 0x91
pub const BC_TYPED_ANY: i8 = -110;               // 0x92
pub const BC_REFERENCE: i8 = -109;               // 0x93

pub const ARRAY_FIX_LEN: i32 = 15;
pub const BC_ARRAY_FIX_0: i8 = -108;             // 0x94
pub const BC_ARRAY_FIX_MIN: i8 = BC_ARRAY_FIX_0;
pub const BC_ARRAY_FIX_MAX: i8 = BC_ARRAY_FIX_MIN + ARRAY_FIX_LEN as i8; // -105
pub const BC_ARRAY: i8 = -92;                    // 0xa4 len_int32 item*

pub const BC_OBJECT_END: i8 = -91;               // 0xa5
pub const BC_OBJECT: i8 = -90;                   // 0xa6

pub const BC_LOCAL_TIME: i8 = -89;               // 0xa7 b0 b1 b2 nano_int32
pub const BC_LOCAL_DATETIME: i8 = -88;           // 0xa8 b0 b1 b2 b3 b4 b5 b6 nano_int32
pub const BC_LOCAL_DATE: i8 = -87;               // 0xa9 b0 b1 b2 b3
pub const BC_TIMESTAMP_WITH_TIMEZONE: i8 = -86;  // 0xaa b0 b1 b2 b3 b4 b5 b6 b7 str_zone
pub const BC_TIMESTAMP_MILLIS: i8 = -85;         // 0xab b0 b1 b2 b3 b4 b5 b6 b7
pub const BC_TIMESTAMP_SECONDS: i8 = -84;        // 0xac b0 b1 b2 b3
pub const BC_TIMESTAMP_MINUTES: i8 = -83;        // 0xad b0 b1 b2 b3
pub const BC_TIMESTAMP: i8 = -82;                // 0xae millis_8 + nano_int32

pub const BC_NULL: i8 = -81;             // 0xaf
pub const BC_FALSE: i8 = -80;            // 0xb0
pub const BC_TRUE: i8 = -79;             // 0xb1
pub const BC_DOUBLE_NUM_0: i8 = -78;     // 0xb2
pub const BC_DOUBLE_NUM_1: i8 = -77;     // 0xb3
pub const BC_DOUBLE_LONG: i8 = -76;      // 0xb4
pub const BC_DOUBLE: i8 = -75;           // 0xb5
pub const BC_FLOAT_INT: i8 = -74;        // 0xb6
pub const BC_FLOAT: i8 = -73;            // 0xb7
pub const BC_DECIMAL_LONG: i8 = -72;     // 0xb8
pub const BC_DECIMAL: i8 = -71;          // 0xb9
pub const BC_BIGINT_LONG: i8 = -70;      // 0xba
pub const BC_BIGINT: i8 = -69;           // 0xbb
pub const BC_INT16: i8 = -68;            // 0xbc b0 b1
pub const BC_INT8: i8 = -67;             // 0xbd b0
pub const BC_INT64: i8 = -66;            // 0xbe b0 b1 b2 b3 b4 b5 b6 b7
pub const BC_INT64_INT: i8 = -65;        // 0xbf b0 b1 b2 b3

pub const INT64_SHORT_MIN: i32 = -0x40000; // -262144
pub const INT64_SHORT_MAX: i32 = 0x3ffff;  // 262143

pub const INT64_BYTE_MIN: i32 = -0x800;    // -2048
pub const INT64_BYTE_MAX: i32 = 0x7ff;     // 2047

pub const BC_INT64_SHORT_MIN: i8 = -64;  // 0xc0
pub const BC_INT64_SHORT_ZERO: i8 = -60; //
pub const BC_INT64_SHORT_MAX: i8 = -57;  // 0xc7

pub const BC_INT64_BYTE_MIN: i8 = -56;   // 0xc8
pub const BC_INT64_BYTE_ZERO: i8 = -48;
pub const BC_INT64_BYTE_MAX: i8 = -41;   // 0xd7

pub const BC_INT64_NUM_MIN: i8 = -40;    // 0xd8 -8
pub const BC_INT64_NUM_MAX: i8 = -17;    // 0xef 15

pub const INT64_NUM_LOW_VALUE: i32 = -8;  // -8
pub const INT64_NUM_HIGH_VALUE: i32 = 15; // 15

pub const BC_INT32_NUM_0: i8 = 0;
pub const BC_INT32_NUM_1: i8 = 1;
pub const BC_INT32_NUM_16: i8 = 16;

pub const BC_INT32_NUM_MIN: i8 = -16; // 0xf0
pub const BC_INT32_NUM_MAX: i8 = 47;  // 0x2f

pub const BC_INT32_BYTE_MIN: i8 = 48;    // 0x30
pub const BC_INT32_BYTE_ZERO: i8 = 56;
pub const BC_INT32_BYTE_MAX: i8 = 63;

pub const BC_INT32_SHORT_MIN: i8 = 64; // 0x40
pub const BC_INT32_SHORT_ZERO: i8 = 68;
pub const BC_INT32_SHORT_MAX: i8 = 71; // 0x47
pub const BC_INT32: i8 = 72; // 0x48

pub const INT32_BYTE_MIN: i32 = -0x800; // -2048
pub const INT32_BYTE_MAX: i32 = 0x7ff;  // 2047

pub const INT32_SHORT_MIN: i32 = -0x40000; // -262144
pub const INT32_SHORT_MAX: i32 = 0x3ffff;  // 262143

pub const BC_STR_ASCII_FIX_0: i8 = 73;
pub const BC_STR_ASCII_FIX_1: i8 = 74;
pub const BC_STR_ASCII_FIX_4: i8 = 77;
pub const BC_STR_ASCII_FIX_5: i8 = 78;

pub const BC_STR_ASCII_FIX_32: i8 = 105;
pub const BC_STR_ASCII_FIX_36: i8 = 109;

pub const STR_ASCII_FIX_LEN: i32 = 47;

pub const BC_STR_ASCII_FIX_MIN: i8 = 73; // 0x49
pub const BC_STR_ASCII_FIX_MAX: i8 = BC_STR_ASCII_FIX_MIN + STR_ASCII_FIX_LEN as i8; // 120 0x78
pub const BC_STR_ASCII: i8 = 121;
pub const BC_STR_UTF8: i8 = 122;
pub const BC_STR_UTF16: i8 = 123;
pub const BC_STR_UTF16LE: i8 = 124;
pub const BC_STR_UTF16BE: i8 = 125;
pub const BC_STR_GB18030: i8 = 126;
pub const BC_SYMBOL: i8 = 127;

pub mod features {
    pub const FIELD_BASED: u64 = 1;
    pub const IGNORE_NONE_SERIALIZABLE: u64 = 1 << 1;
    pub const ERROR_ON_NONE_SERIALIZABLE: u64 = 1 << 2;
    pub const BEAN_TO_ARRAY: u64 = 1 << 3;
    pub const WRITE_NULLS: u64 = 1 << 4;
    pub const WRITE_MAP_NULL_VALUE: u64 = 1 << 4;
    pub const BROWSER_COMPATIBLE: u64 = 1 << 5;
    pub const NULL_AS_DEFAULT_VALUE: u64 = 1 << 6;
    pub const WRITE_BOOLEAN_AS_NUMBER: u64 = 1 << 7;
    pub const WRITE_NON_STRING_VALUE_AS_STRING: u64 = 1 << 8;
    pub const WRITE_CLASS_NAME: u64 = 1 << 9;
    pub const NOT_WRITE_ROOT_CLASS_NAME: u64 = 1 << 10;
    pub const NOT_WRITE_HASH_MAP_ARRAY_LIST_CLASS_NAME: u64 = 1 << 11;
    pub const NOT_WRITE_DEFAULT_VALUE: u64 = 1 << 12;
    pub const WRITE_ENUMS_USING_NAME: u64 = 1 << 13;
    pub const WRITE_ENUM_USING_TO_STRING: u64 = 1 << 14;
    pub const IGNORE_ERROR_GETTER: u64 = 1 << 15;
    pub const PRETTY_FORMAT: u64 = 1 << 16;
    pub const REFERENCE_DETECTION: u64 = 1 << 17;
    pub const WRITE_NAME_AS_SYMBOL: u64 = 1 << 18;
    pub const WRITE_BIG_DECIMAL_AS_PLAIN: u64 = 1 << 19;
    pub const USE_SINGLE_QUOTES: u64 = 1 << 20;
    pub const MAP_SORT_FIELD: u64 = 1 << 21;
    pub const WRITE_NULL_LIST_AS_EMPTY: u64 = 1 << 22;

    pub const WRITE_NULL_STRING_AS_EMPTY: u64 = 1 << 23;

    pub const WRITE_NULL_NUMBER_AS_ZERO: u64 = 1 << 24;

    pub const WRITE_NULL_BOOLEAN_AS_FALSE: u64 = 1 << 25;


    pub const NOT_WRITE_EMPTY_ARRAY: u64 = 1 << 26;
    pub const WRITE_NON_STRING_KEY_AS_STRING: u64 = 1 << 27;

    pub const WRITE_PAIR_AS_JAVA_BEAN: u64 = 1u64 << 28;


    pub const OPTIMIZED_FOR_ASCII: u64 = 1u64 << 29;


    pub const ESCAPE_NONE_ASCII: u64 = 1u64 << 30;

    pub const WRITE_BYTE_ARRAY_AS_BASE64: u64 = 1u64 << 31;


    pub const IGNORE_NON_FIELD_GETTER: u64 = 1u64 << 32;


    pub const LARGE_OBJECT: u64 = 1u64 << 33;


    pub const WRITE_LONG_AS_STRING: u64 = 1u64 << 34;


    pub const BROWSER_SECURE: u64 = 1u64 << 35;
    pub const WRITE_ENUM_USING_ORDINAL: u64 = 1u64 << 36;


    pub const WRITE_THROWABLE_CLASS_NAME: u64 = 1u64 << 37;


    pub const UNQUOTE_FIELD_NAME: u64 = 1u64 << 38;


    pub const NOT_WRITE_SET_CLASS_NAME: u64 = 1u64 << 39;


    pub const NOT_WRITE_NUMBER_CLASS_NAME: u64 = 1u64 << 40;
}


pub const BIG_ENDIAN: bool = cfg!(target_endian = "little");
