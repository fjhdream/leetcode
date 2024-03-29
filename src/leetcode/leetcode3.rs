use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chs = s.chars().collect::<Vec<_>>();
    let mut right = 0;
    let mut set = HashSet::new();
    let mut max_len = 0;
    for i in 0..chs.len() {
        if i != 0 {
            set.remove(&chs[i - 1]);
        }
        while right < chs.len() && !set.contains(&chs[right]) {
            set.insert(chs[right]);
            right += 1;
        }
        max_len = max_len.max(right - i)
    }
    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }
}
