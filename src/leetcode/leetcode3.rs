use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chs = s.chars().collect::<Vec<_>>();
    let mut window_set = HashSet::new();
    let mut max_len = 0;
    let n = chs.len();
    let mut right = 0;
    for i in 0..n {
        if i != 0 {
            window_set.remove(&chs[i-1]);
        }
        while right < n && !window_set.contains(&chs[right]) {
            window_set.insert(chs[right]);
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