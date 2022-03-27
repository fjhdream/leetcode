pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
    let (len, word_len) = (abbr.len(), word.len());
    let (mut abbr_len, mut num) = (0, 0);
    let word_chars = word.chars().collect::<Vec<_>>();
    let abbr_chars = abbr.chars().collect::<Vec<_>>();
    for i in 0..len {
        if abbr_chars[i].is_ascii_alphabetic() {
            abbr_len += num + 1;
            num = 0;
            if abbr_len > word_len || abbr_chars[i] != word_chars[abbr_len - 1] {
                return false;
            }
        } else {
            if num == 0 && abbr_chars[i] == '0' {
                return false;
            }
            num = num * 10 + abbr_chars[i].to_digit(10).unwrap() as usize;
        }
    }
    return abbr_len + num == word_len;
}

#[test]
fn test() {
    let ans = valid_word_abbreviation("internationalization".to_string(), "i12iz4n".to_string());
    assert!(ans)
}