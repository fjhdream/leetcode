pub fn check_palindrome_formation(a: String, b: String) -> bool {
    let a = a.chars().collect::<Vec<char>>();
    let b = b.chars().collect::<Vec<char>>();
    check_concat(&a, &b) || check_concat(&b, &a)
}

fn check_concat(a: &Vec<char>, b: &Vec<char>) -> bool {
    let mut i = 0;
    let mut j = b.len() - 1;
    while i < j && a[i] == b[j] {
        i += 1;
        j -= 1;
    }
    if i >= j {
        return true;
    }
    check_self_palindrome(&a[i..j+1]) || check_self_palindrome(&b[i..j+1])
}

fn check_self_palindrome(chars: &[char]) -> bool {
    let mut i = 0;
    let mut j = chars.len() - 1;
    while i < j && chars[i] == chars[j] {
        i += 1;
        j -= 1;
    }
    i >= j
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_palindrome_formation() {
        assert_eq!(check_palindrome_formation("ulacfd".to_string(), "jizalu".to_string()), true);
        assert_eq!(check_palindrome_formation("x".to_string(), "y".to_string()), true);
        assert_eq!(check_palindrome_formation("abdef".to_string(), "fecab".to_string()), true);
    }
}