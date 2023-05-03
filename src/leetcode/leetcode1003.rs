pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for ch in s.chars() {
        let n = stack.len();
        if ch == 'c' && n >= 2 && stack[n - 1] == 'b' && stack[n - 2] == 'a' {
            stack.pop();
            stack.pop();
        } else {
            stack.push(ch);
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1003() {
        assert_eq!(is_valid("aabcbc".to_string()), true);
        assert_eq!(is_valid("abcabcababcc".to_string()), true);
        assert_eq!(is_valid("abccba".to_string()), false);
        assert_eq!(is_valid("cababc".to_string()), false);
    }
}
