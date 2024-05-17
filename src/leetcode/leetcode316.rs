pub fn remove_duplicate_letters(s: String) -> String {
    let chars = s.chars().collect::<Vec<char>>();
    let mut left = chars
        .iter()
        .enumerate()
        .fold(vec![0; 26], |mut map, (_, &c)| {
            map[c as usize - 'a' as usize] += 1;
            map
        });
    let mut res = Vec::new();
    let mut is_ans = [false; 26];
    for ele in chars {
        left[ele as usize - 'a' as usize] -= 1;
        if is_ans[ele as usize - 'a' as usize] {
            continue;
        }
        while !res.is_empty()
            && ele < *res.last().unwrap()
            && left[*res.last().unwrap() as usize - 'a' as usize] > 0
        {
            is_ans[res.pop().unwrap() as usize - 'a' as usize] = false;
        }
        res.push(ele);
        is_ans[ele as usize - 'a' as usize] = true;
    }
    res.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "bcabc".to_string();
        assert_eq!(remove_duplicate_letters(s), "abc".to_string());
        let s = "cbacdcbc".to_string();
        assert_eq!(remove_duplicate_letters(s), "acdb".to_string());
    }
}
