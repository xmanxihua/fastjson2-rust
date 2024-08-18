use crate::constants::{BC_ARRAY, BC_ARRAY_FIX_MIN, BC_BIGINT, BC_BIGINT_LONG, BC_BINARY, BC_DECIMAL, BC_DECIMAL_LONG, BC_DOUBLE, BC_DOUBLE_LONG, BC_DOUBLE_NUM_0, BC_DOUBLE_NUM_1, BC_FALSE, BC_FLOAT, BC_FLOAT_INT, BC_INT16, BC_INT32, BC_INT32_BYTE_MAX, BC_INT32_BYTE_MIN, BC_INT32_NUM_MAX, BC_INT32_NUM_MIN, BC_INT32_SHORT_MAX, BC_INT32_SHORT_MIN, BC_INT64, BC_INT64_BYTE_MAX, BC_INT64_BYTE_MIN, BC_INT64_INT, BC_INT64_NUM_MAX, BC_INT64_NUM_MIN, BC_INT64_SHORT_MAX, BC_INT64_SHORT_MIN, BC_INT8, BC_LOCAL_DATE, BC_LOCAL_DATETIME, BC_LOCAL_TIME, BC_NULL, BC_OBJECT, BC_OBJECT_END, BC_REFERENCE, BC_STR_ASCII, BC_STR_ASCII_FIX_MIN, BC_STR_UTF16, BC_STR_UTF16BE, BC_STR_UTF16LE, BC_STR_UTF8, BC_SYMBOL, BC_TIMESTAMP, BC_TIMESTAMP_MILLIS, BC_TIMESTAMP_MINUTES, BC_TIMESTAMP_SECONDS, BC_TIMESTAMP_WITH_TIMEZONE, BC_TRUE, BC_TYPED_ANY};
use crate::json_writer_jsonb::JSONWriterJSONB;

pub fn bool_to_bytes(v: bool) -> Vec<i8> {
    vec![if v { BC_TRUE } else { BC_FALSE }]
}

pub fn i32_to_bytes(i: i32) -> Vec<i8> {
    if i >= BC_INT32_NUM_MIN as i32 && i <= BC_INT32_NUM_MAX as i32 {
        return vec![i as i8];
    }
    let mut json_writer = JSONWriterJSONB::new();
    json_writer.write_int32(i);
    json_writer.get_bytes()
}


pub fn type_name(r#type: i8) -> String {
    match r#type {
        BC_OBJECT =>
            String::from("OBJECT ") + r#type.to_string().as_str(),
        BC_OBJECT_END =>
            String::from("OBJECT_END ") + r#type.to_string().as_str(),
        BC_REFERENCE =>
            String::from("REFERENCE ") + r#type.to_string().as_str(),
        BC_SYMBOL =>
            String::from("SYMBOL ") + r#type.to_string().as_str(),
        BC_NULL =>
            String::from("NULL ") + r#type.to_string().as_str(),
        BC_TRUE =>
            String::from("TRUE ") + r#type.to_string().as_str(),
        BC_FALSE =>
            String::from("FALSE ") + r#type.to_string().as_str(),
        BC_STR_UTF8 =>
            String::from("STR_UTF8 ") + r#type.to_string().as_str(),
        BC_STR_UTF16 =>
            String::from("STR_UTF16 ") + r#type.to_string().as_str(),
        BC_STR_UTF16LE =>
            String::from("STR_UTF16LE ") + r#type.to_string().as_str(),
        BC_STR_UTF16BE =>
            String::from("STR_UTF16BE ") + r#type.to_string().as_str(),
        BC_INT8 =>
            String::from("INT8 ") + r#type.to_string().as_str(),
        BC_INT16 =>
            String::from("INT16 ") + r#type.to_string().as_str(),
        BC_INT32 =>
            String::from("INT32 ") + r#type.to_string().as_str(),
        BC_INT64 | BC_INT64_INT =>
            String::from("INT64 ") + r#type.to_string().as_str(),
        BC_FLOAT | BC_FLOAT_INT =>
            String::from("FLOAT ") + r#type.to_string().as_str(),
        BC_DOUBLE | BC_DOUBLE_LONG | BC_DOUBLE_NUM_0 | BC_DOUBLE_NUM_1 =>
            String::from("DOUBLE ") + r#type.to_string().as_str(),
        BC_BIGINT | BC_BIGINT_LONG =>
            String::from("BIGINT ") + r#type.to_string().as_str(),
        BC_DECIMAL | BC_DECIMAL_LONG =>
            String::from("DECIMAL ") + r#type.to_string().as_str(),
        BC_LOCAL_TIME =>
            String::from("LOCAL_TIME ") + r#type.to_string().as_str(),
        BC_BINARY =>
            String::from("BINARY ") + r#type.to_string().as_str(),
        BC_LOCAL_DATETIME =>
            String::from("LOCAL_DATETIME ") + r#type.to_string().as_str(),
        BC_TIMESTAMP =>
            String::from("TIMESTAMP ") + r#type.to_string().as_str(),
        BC_TIMESTAMP_MINUTES =>
            String::from("TIMESTAMP_MINUTES ") + r#type.to_string().as_str(),
        BC_TIMESTAMP_SECONDS =>
            String::from("TIMESTAMP_SECONDS ") + r#type.to_string().as_str(),
        BC_TIMESTAMP_MILLIS =>
            String::from("TIMESTAMP_MILLIS ") + r#type.to_string().as_str(),
        BC_TIMESTAMP_WITH_TIMEZONE =>
            String::from("TIMESTAMP_WITH_TIMEZONE ") + r#type.to_string().as_str(),
        BC_LOCAL_DATE =>
            String::from("LOCAL_DATE ") + r#type.to_string().as_str(),
        BC_TYPED_ANY =>
            String::from("TYPED_ANY ") + r#type.to_string().as_str(),
        _ => {
            if r#type >= BC_ARRAY_FIX_MIN && r#type <= BC_ARRAY {
                return String::from("ARRAY ") + r#type.to_string().as_str().as_str();
            }

            if r#type >= BC_STR_ASCII_FIX_MIN && r#type <= BC_STR_ASCII {
                return String::from("STR_ASCII ") + r#type.to_string().as_str();
            }

            if r#type >= BC_INT32_NUM_MIN && r#type <= BC_INT32_NUM_MAX {
                return String::from("INT32 ") + r#type.to_string().as_str();
            }

            if r#type >= BC_INT32_BYTE_MIN && r#type <= BC_INT32_BYTE_MAX {
                return String::from("INT32 ") + r#type.to_string().as_str();
            }

            if r#type >= BC_INT32_SHORT_MIN && r#type <= BC_INT32_SHORT_MAX {
                return String::from("INT32 ") + r#type.to_string().as_str();
            }

            if r#type >= BC_INT64_NUM_MIN && r#type <= BC_INT64_NUM_MAX {
                return String::from("INT64 ") + r#type.to_string().as_str();
            }

            if r#type >= BC_INT64_BYTE_MIN && r#type <= BC_INT64_BYTE_MAX {
                return String::from("INT64 ") + r#type.to_string().as_str();
            }

            if r#type >= BC_INT64_SHORT_MIN && r#type <= BC_INT64_SHORT_MAX {
                return String::from("INT64 ") + r#type.to_string().as_str();
            }

            r#type.to_string()
        }
    }
}
