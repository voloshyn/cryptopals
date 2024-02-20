use crypto_lib::arrays::hex_to_bytes;
use crypto_lib::arrays::xor_single;
use crypto_lib::english::is_english_text;
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
    for line in read_to_string(filename).unwrap().lines() {
        let bytes = hex_to_bytes(line).unwrap();
        for i in 0..=255 {
            let xored = xor_single(&bytes, i);
            match str::from_utf8(&xored) {
                Err(_) => continue,
                Ok(result) => {
                    if is_english_text(result) {
                        println!("result: {:?}", result);
                    }
                }
            }
        }
    }
}
