pub fn solution(word1: String, word2: String) -> String {
    let mut merged = String::new();
    let length_word_1 = word1.len();
    let length_word_2 = word2.len();

    let n = std::cmp::min(length_word_1, length_word_2);

    for i in 0..n {
        merged.push(word1.as_bytes()[i] as char);
        merged.push(word2.as_bytes()[i] as char);
    }

    if length_word_1 < length_word_2 && n >= length_word_1 {
        merged.push_str(word2.get(n..length_word_2).unwrap());
    } else if length_word_1 > length_word_2 && n >= length_word_2 {
        merged.push_str(word1.get(n..length_word_1).unwrap());
    }
    merged
}
