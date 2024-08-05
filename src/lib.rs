pub mod jsonb;
mod constants;
mod json_writer_jsonb;
mod io_utils;

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
        jsonb.write_option_i32(Some(0x01020304));
        let vec1 = jsonb.get_bytes();
        println!("{:?}", vec1);
        assert_eq!(vec1, vec![72, 1, 2, 3, 4]);
    }
}
