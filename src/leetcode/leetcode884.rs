use std::collections::HashMap;

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut map = HashMap::new();
    s1.split(' ').for_each(|str| *map.entry(str).or_insert(0) += 1);
    s2.split(' ').for_each(|str| *map.entry(str).or_insert(0) += 1);
    let mut ans = Vec::new();
    for (key, val) in &map {
        if val == &1 {
            ans.push(String::from(*key));
        }
    }
    ans
}

#[test]
fn test() {
    let s1 = String::from("this apple is sweet");
    let s2 = String::from("this apple is sour");
    let ans = uncommon_from_sentences(s1, s2);
    assert_eq!(ans, vec![String::from("sweet"), String::from("sour")]);
}