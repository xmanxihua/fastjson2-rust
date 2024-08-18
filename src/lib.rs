extern crate core;

pub mod jsonb;
mod constants;
mod json_writer_jsonb;
mod io_utils;
mod json_reader_jsonb;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::json_writer_jsonb::JSONWriterJSONB;
    use super::*;

    #[test]
    fn it_works() {
        let mut jsonb = JSONWriterJSONB::new();
        // jsonb.write_option_i32(Some(0x01020304));
        // let vec1 = jsonb.get_bytes();
        // println!("{:?}", vec1);
        // assert_eq!(vec1, vec![72, 1, 2, 3, 4]);
        jsonb.write_option_string_utf16(Some("𠜎国".into()));
        let vec1 = jsonb.get_bytes();
        println!("{:?}", vec1);
        // assert_eq!(vec1, vec![122, 6, -28, -72, -83, -27, -101, -67]);
        assert_eq!(vec1, vec![122, 7, -16, -96, -100, -114, -27, -101, -67]);
    }
}
