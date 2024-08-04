use std::io::Bytes;
use crate::constants::{BC_FALSE, BC_INT32_NUM_MAX, BC_INT32_NUM_MIN, BC_TRUE};
use crate::json_writer_jsonb;
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
