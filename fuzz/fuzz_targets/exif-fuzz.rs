#![no_main]
use libfuzzer_sys::fuzz_target;
use exif::{In, Reader, Tag, Value};
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let exif = Reader::new().read_from_container(&mut Cursor::new(data));
});
