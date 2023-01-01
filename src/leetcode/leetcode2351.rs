use std::collections::HashMap;

pub fn repeated_character(s: String) -> char {
    let mut map = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
        if map.get(&c).unwrap() > &1 {
            return c;
        }
    }
    panic!("No repeated character");
}
