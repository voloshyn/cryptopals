use crypto_lib::arrays::base64_decode;
use crypto_lib::crypto::break_repeating_xor;
use std::env;
use std::fs::read_to_string;
use std::str;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => {
            println!("Usage: {} <filename>", env::args().next().unwrap());
            std::process::exit(1);
        }
    };
    let base64_string = read_to_string(filename).unwrap();
    let bytes = base64_decode(&base64_string).unwrap();
    let result = break_repeating_xor(&bytes);
    println!("{}", str::from_utf8(&result).unwrap());
}
