use crypto_lib::arrays::hex_to_bytes;
use crypto_lib::arrays::xor_single;
use crypto_lib::english::is_english_text;
use std::str;

fn main() {
    let hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = hex_to_bytes(hex).unwrap();
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
