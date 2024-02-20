use crypto_lib::arrays::b64_encode;
use crypto_lib::arrays::hex_to_bytes;
use std::str;

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected_base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let bytes = hex_to_bytes(hex).unwrap();
    let decoded = str::from_utf8(&bytes).unwrap();
    println!("decoded: {}", decoded);
    let encoded = b64_encode(&bytes);
    println!("base64: {}", encoded);
    assert_eq!(encoded, expected_base64);
}
