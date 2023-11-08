use std::collections::HashSet;

pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
    let mut ans = 0;
    for i in left as usize..=right as usize {
        if is_vowel(words[i].clone()) {
            ans += 1;
        }
    }
    ans
}

fn is_vowel(word: String) -> bool {
    let set = vec!['a', 'e', 'i', 'o', 'u'].into_iter().collect::<HashSet<char>>();
    let chars = word.chars().collect::<Vec<_>>();
    set.contains(chars.first().unwrap()) && set.contains(chars.last().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test() {
        let ans = vowel_strings(vec!["are".to_string(), "amy".to_string(), "u".to_string()], 0, 2);
        assert_eq!(ans, 2)
    }
}