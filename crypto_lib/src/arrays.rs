use base64::{engine::general_purpose, Engine as _};
use std::str;

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}

pub fn b64_encode(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

pub fn base64_decode(base64: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(base64.trim().replace("\n", ""))
}

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

pub fn xor_single(a: &[u8], b: u8) -> Vec<u8> {
    a.iter().map(|x| x ^ b).collect()
}

pub fn xor_repeating(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter().cycle()).map(|(x, y)| x ^ y).collect()
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x ^ y).count_ones())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bytes() {
        let hex = "48656c6c6f";
        let expected_bytes = b"Hello".to_vec();
        let result = hex_to_bytes(hex).unwrap();
        assert_eq!(result, expected_bytes);
    }

    #[test]
    fn test_bytes_to_base64() {
        let bytes = b"Hello";
        let expected_base64 = "SGVsbG8=";
        let result = b64_encode(bytes);
        assert_eq!(result, expected_base64);
    }

    #[test]
    fn test_xor() {
        let a = b"Hello";
        let b = b"World";
        let expected_result = vec![0x1F, 0x0A, 0x1E, 0x0, 0x0B];
        let result = xor(a, b);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_xor_single() {
        let a = b"Hello";
        let b = 0xA;
        let expected_result = vec![0x42, 0x6F, 0x66, 0x66, 0x65];
        let result = xor_single(a, b);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_xor_repeating() {
        let a = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let b = b"ICE";
        let expected_result = hex_to_bytes("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();
        let result = xor_repeating(a, b);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_hamming_distance() {
        let a = b"this is a test";
        let b = b"wokka wokka!!!";
        let expected_result = 37;
        let result = hamming_distance(a, b);
        assert_eq!(result, expected_result);
    }
}
