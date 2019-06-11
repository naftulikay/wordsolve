use super::*;

#[test]
fn test_is_valid_word_size() {
    // test too smol allowed word
    assert!(!is_valid_word_size(3, 7, "to"));
    // test too big allowed word
    assert!(!is_valid_word_size(3, 7, "dictionary"));
    // test right size of word
    assert!(is_valid_word_size(3, 7, "too"));
    assert!(is_valid_word_size(3, 7, "feed"));
    assert!(is_valid_word_size(3, 7, "feeling"));
}

#[test]
fn test_is_valid_word_chars() {
    // test invalid cases
    assert!(!is_valid_word_chars(&"total".char_count(), "too"));
    assert!(!is_valid_word_chars(&"total".char_count(), "tote"));
    // test valid cases
    assert!(is_valid_word_chars(&"dedis".char_count(), "die"));
    assert!(is_valid_word_chars(&"dedis".char_count(), "died"));
    assert!(is_valid_word_chars(&"tootal".char_count(), "too"));
    assert!(is_valid_word_chars(&"tootal".char_count(), "total"));
    assert!(is_valid_word_chars(&"tootal".char_count(), "too"));
}
