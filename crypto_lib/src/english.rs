use std::collections::HashMap;

pub fn is_english_text(text: &str) -> bool {
    let score = english_score(text);
    score > 70.0
}

pub fn english_score(text: &str) -> f64 {
    let letter_frequencies = HashMap::from([
        ('e', 11.1607),
        ('a', 8.4966),
        ('r', 7.5809),
        ('i', 7.5448),
        ('o', 7.1635),
        ('t', 6.9509),
        ('n', 6.6544),
        ('s', 5.7351),
        ('l', 5.4893),
        ('c', 4.5388),
        ('u', 3.6308),
        ('d', 3.3844),
        ('p', 3.1671),
        ('m', 3.0129),
        ('h', 3.0034),
        ('g', 2.4705),
        ('b', 2.0720),
        ('f', 1.8121),
        ('y', 1.7779),
        ('w', 1.2899),
        ('k', 1.1016),
        ('v', 1.0074),
        ('x', 0.2902),
        ('z', 0.2722),
        ('j', 0.1965),
        ('q', 0.1962),
    ]);

    if text.is_empty()
        || text
            .chars()
            .any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t')
    {
        return 0.0;
    }

    let text_only_alpha = text
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();

    let mut score = 0.0;
    let mut text_letter_count = HashMap::new();

    for c in text_only_alpha.chars() {
        let lower_c = c.to_ascii_lowercase();
        text_letter_count
            .entry(lower_c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let text_len = text.len();
    letter_frequencies.iter().for_each(|(letter, frequency)| {
        let letter_count = text_letter_count.get(letter).unwrap_or(&0);
        let text_freq = match text_len {
            0 => 0.0,
            _ => *letter_count as f64 / text_len as f64 * 100.0,
        };
        let partition_overlap = text_freq * frequency;
        score += partition_overlap.sqrt();
    });
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_english_score_with_english_text() {
        let text = "This is a simple English sentence.";
        let score = english_score(text);
        assert!(score > 70.0);
    }

    #[test]
    fn test_get_english_score_with_non_english_text() {
        let text = "これは日本語の文章です。";
        let score = english_score(text);
        assert!(score < 10.0);
    }

    #[test]
    fn test_get_english_score_with_mixed_content() {
        let text = "This is 12345 and some symbols !@#$%";
        let score = english_score(text);
        assert!(score > 50.0);
    }

    #[test]
    fn test_get_english_score_with_empty_string() {
        let text = "";
        let score = english_score(text);
        assert_eq!(score, 0.0);
    }
}
