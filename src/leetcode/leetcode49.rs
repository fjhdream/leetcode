use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    for str in strs {
        let mut chs = str.chars().collect::<Vec<_>>();
        chs.sort();
        let new_str = chs.iter().collect::<String>();
        map.entry(new_str).or_insert(Vec::new()).push(str);
    }
    map.into_iter().map(|(_, v)| v).collect()
}
