use std::collections::{HashMap, HashSet};

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    set.extend(banned.iter());
    let mut max = 0;
    let mut res = String::new();
    for word in paragraph.split(|c: char| !c.is_alphabetic()) {
        if word.is_empty() {
            continue;
        }
        let word = word.to_lowercase();
        if !set.contains(&word) {
            *map.entry(word).or_insert(0) += 1;
        }
    }
    for (word, count) in map {
        if count > max {
            max = count;
            res = word;
        }
    }
    res
}

#[test]
fn test() {
    let ans = most_common_word("Bob hit a ball, the hit BALL flew far after it was hit.".to_string(), vec!["hit".to_string()]);
    assert_eq!(ans, "ball");
}

#[test]
fn test2() {
    let ans = most_common_word("a.".to_string(), vec![]);
    assert_eq!(ans, "a");
}

#[test]
fn test3() {
    let ans = most_common_word("Bob. hIt, baLl".to_string(), vec!["bob".to_string(), "hit".to_string()]);
    assert_eq!(ans, "ball");
}