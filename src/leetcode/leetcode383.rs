use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut map: HashMap<char, i32> = HashMap::new();
    let ransom_chars = ransom_note.chars().collect::<Vec<_>>();
    let magazine_chars = magazine.chars().collect::<Vec<_>>();
    for magazine_char in magazine_chars {
        *map.entry(magazine_char).or_default() += 1;
    }
    for ransom_char in ransom_chars {
        if !map.contains_key(&ransom_char) {
            return false;
        } else {
            let entry = map.entry(ransom_char).or_default();
            if entry.to_owned() <= 0 {
                return false;
            }
            *entry -= 1;
        } 
    }
    true
}

#[test]
fn test() {
    let ransom = String::from("aa");
    let magazine = String::from("aab");
    let ans = can_construct(ransom, magazine);
    assert_eq!(ans, true);
}


#[test]
fn test2() {
    let ransom = String::from("aabb");
    let magazine = String::from("aab");
    let ans = can_construct(ransom, magazine);
    assert_eq!(ans, false);
}

#[test]
fn test3() {
    let ransom = String::from("aab");
    let magazine = String::from("aab");
    let ans = can_construct(ransom, magazine);
    assert_eq!(ans, true);
}