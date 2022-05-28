pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
    let mut idx_word1 = vec![];
    let mut idx_word2 = vec![];
    for (i, word) in words.iter().enumerate() {
        if word == &word1 {
            idx_word1.push(i);
        }
        if word == &word2 {
            idx_word2.push(i);
        }
    }
    let mut min_distance = i32::MAX;
    for idx in idx_word1 {
        match idx_word2.binary_search(&idx) {
            Ok(_) => {
                min_distance = 0;
            }
            Err(j) => {
                if j == 0 {
                    min_distance = min_distance.min((idx as i32 - idx_word2[j] as i32).abs());
                } else if j == idx_word2.len() {
                    min_distance = min_distance.min((idx as i32 - idx_word2[j - 1] as i32).abs());
                } else {
                    min_distance = min_distance.min((idx as i32 - idx_word2[j] as i32).abs());
                    min_distance = min_distance.min((idx as i32 - idx_word2[j - 1] as i32).abs());
                }
            }
        } ;
    }
    return min_distance;
}

pub fn find_closest2(words: Vec<String>, word1: String, word2: String) -> i32 {
    let mut idx1 = -1;
    let mut idx2 = -1;
    let mut res = i32::MAX;
    for (i, word) in words.iter().enumerate() {
        if word == &word1 {
            idx1 = i as i32;
        }
        if word == &word2 {
            idx2 = i as i32;
        }
        if idx1 >= 0 && idx2 >= 0 {
            res = res.min((idx1 as i32 - idx2 as i32).abs());
        }
    }
    res
}

#[test]
fn test() {
    let ans = find_closest2(
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "f".to_string(),
            "h".to_string(),
            "k".to_string(),
            "p".to_string(),
            "r".to_string(),
            "s".to_string(),
            "t".to_string(),
            "u".to_string(),
            "v".to_string(),
            "w".to_string(),
            "x".to_string(),
            "y".to_string(),
            "z".to_string(),
        ],
        "a".to_string(),
        "c".to_string(),
    );
    assert_eq!(ans, 2);
}