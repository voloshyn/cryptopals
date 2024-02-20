use crate::arrays::hamming_distance;
use crate::arrays::xor_repeating;
use crate::english::english_score;
use std::str;

pub fn guess_key_size(ciphertext: &[u8]) -> usize {
    let mut minimal_distance = u32::MAX;
    let mut key_size = 0;
    for size in 2..=40 {
        let chunks = ciphertext.len() / size;
        let mut distances: Vec<u32> = vec![];
        for i in 0..chunks - 1 {
            let a = &ciphertext[i * size..(i + 1) * size];
            let b = &ciphertext[(i + 1) * size..(i + 2) * size];
            let distance = hamming_distance(&a, &b);
            distances.push(distance);
        }
        let average_distance: u32 =
            distances.iter().sum::<u32>() / distances.len() as u32 / size as u32;
        if average_distance < minimal_distance {
            minimal_distance = average_distance;
            key_size = size;
        }
    }
    key_size
}

pub fn break_repeating_xor(ciphertext: &[u8]) -> Vec<u8> {
    let key_size = guess_key_size(ciphertext);
    println!("Key size: {}", key_size);
    let mut key = vec![];
    for i in 0..key_size {
        let block: Vec<u8> = ciphertext
            .iter()
            .skip(i)
            .step_by(key_size)
            .cloned()
            .collect();
        let mut block_bytes = vec![];
        for byte in 0..=255 {
            let key_byte = vec![byte];
            let plaintext = xor_repeating(&block, &key_byte);
            let text = match str::from_utf8(&plaintext) {
                Ok(text) => text,
                Err(_) => continue,
            };
            let score = english_score(&text);
            block_bytes.push((score, byte));
        }
        block_bytes.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        key.push(block_bytes[0].1);
    }
    println!("Key: {:?}", str::from_utf8(key.as_slice()).unwrap());
    xor_repeating(ciphertext, &key)
}
