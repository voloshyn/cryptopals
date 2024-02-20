use crypto_lib::arrays::hex_to_bytes;
use crypto_lib::arrays::xor;
use std::str;

fn main() {
    let a = hex_to_bytes("1c0111001f010100061a024b53535009181c").unwrap();
    let b = hex_to_bytes("686974207468652062756c6c277320657965").unwrap();
    let xored = xor(&a, &b);
    let result = str::from_utf8(&xored).unwrap();
    println!("result: {:?}", result);
}
