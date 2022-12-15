use std::iter::FromIterator;

pub fn get_lucky(s: String, mut k: i32) -> i32 {
    let mut chs = s.chars().collect::<Vec<_>>();
    while k >= 0 {
        let mut strs = String::new();
        let mut tmp_ans = 0;
        for ch in chs {
            if ch.is_ascii_alphabetic() {
                let code = (ch as u32 - 'a' as u32 + 1) as i32;
                strs.push_str(&code.to_string());
            } else {
                tmp_ans += ch.to_digit(10).unwrap() as i32;
            }
        }
        if !strs.is_empty() {
            chs = strs.chars().collect::<Vec<_>>();
        } else {
            chs = tmp_ans.to_string().chars().collect::<Vec<_>>();
        }
        k -= 1;
    }
    String::from_iter(chs.iter()).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = get_lucky("iiii".to_string(), 1);
        assert_eq!(ans, 36);
    }

    #[test]
    fn test2() {
        let ans = get_lucky("leetcode".to_string(), 2);
        assert_eq!(ans, 6);
    }
}
