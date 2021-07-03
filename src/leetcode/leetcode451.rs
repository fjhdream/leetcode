use std::collections::HashMap;

pub fn frequency_sort(s: String) -> String {
    let mut map = HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    let mut v: Vec<(usize, char)> = map.into_iter().map(|(k, v)| (v,k)).collect();
    v.sort();
    v.into_iter().rev().map(|(k, c)| vec![c].repeat(k)).flatten().collect()
}