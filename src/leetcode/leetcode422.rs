pub fn valid_word_square(words: Vec<String>) -> bool {
    if words.is_empty() || words[0].is_empty() {
        return true;
    }
    let words_chs = words.iter().map(|word| word.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let m = words.len();
    for i in 0..m {
        let n = words_chs[i].len();
        for j in 0..n {
            if words_chs.len() <= i || words_chs.len() <= j ||  words_chs[i].len() <= j || words_chs[j].len() <= i {
                return false;
            }
            if words_chs[i][j] != words_chs[j][i] {
                return false;
            }
        }
    }
    return true;
}