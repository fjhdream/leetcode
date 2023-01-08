pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    let mut res = 0;
    for ele in words {
        if ele.starts_with(&pref) {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_count() {
        assert_eq!(
            prefix_count(
                vec![
                    "a".to_string(),
                    "aa".to_string(),
                    "aaa".to_string(),
                    "aaaa".to_string()
                ],
                "a".to_string()
            ),
            4
        );
        assert_eq!(
            prefix_count(
                vec![
                    "a".to_string(),
                    "aa".to_string(),
                    "aaa".to_string(),
                    "aaaa".to_string()
                ],
                "b".to_string()
            ),
            0
        );
    }
}
