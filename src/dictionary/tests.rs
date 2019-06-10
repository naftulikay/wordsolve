use super::*;

#[test]
fn test_is_valid_word() {
    // test too smol allowed word
    assert!(!is_valid_word(3, 7, &"to".char_count(), "to"));
    // test too big allowed word
    assert!(!is_valid_word(
        3,
        7,
        &"dictionary".char_count(),
        "dictionary"
    ));
    // test right size but not allowed
    assert!(!is_valid_word(3, 7, &"gredso".char_count(), "greed"));
    // test right size and allowed
    assert!(is_valid_word(3, 7, &"egreds".char_count(), "greed"));
    assert!(is_valid_word(3, 7, &"egreds".char_count(), "seed"));
}
