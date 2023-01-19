pub fn strong_password_checker_ii(password: String) -> bool {
    let chs = password.chars().collect::<Vec<_>>();
    let mut lower = false;
    let mut upper = false;
    let mut digit = false;
    let mut special = false;
    let count = chs.len();
    let mut repeat = false;

    for i in 0..chs.len() {
        if chs[i].is_lowercase() {
            lower = true;
        } else if chs[i].is_uppercase() {
            upper = true;
        } else if chs[i].is_digit(10) {
            digit = true;
        } else {
            special = true;
        }

        if i > 0 && chs[i] == chs[i - 1] {
            repeat = true;
        }
    }
    lower && upper && digit && special && count >= 8 && !repeat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            strong_password_checker_ii("IloveLe3tcode!".to_string()),
            true
        );
        assert_eq!(
            strong_password_checker_ii("IloveLe3tcode".to_string()),
            false
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            strong_password_checker_ii("Me+You--IsMyDream".to_string()),
            false
        );
    }

    #[test]
    fn test3() {
        assert_eq!(strong_password_checker_ii("1aB!".to_string()), false);
    }
}
