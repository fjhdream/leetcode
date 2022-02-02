pub fn reverse_prefix(word: String, ch: char) -> String {
    let chars = word.chars().collect::<Vec<_>>();
    let mut pos = usize::MAX;
    for (idx, val) in chars.iter().enumerate() {
        if *val == ch {
            pos = idx;
            break;
        }
    }
    
    if pos == usize::MAX {
        return word;
    }

    let mut ans = chars[0..=pos].to_owned();
    ans.reverse();  
    ans.append(&mut chars[pos+1..chars.len()].to_owned());
    return ans.iter().collect();
}

#[test]
fn test() {
    let ans = reverse_prefix(String::from("abcdefd"), 'd');
    assert_eq!(ans, "dcbaefd");
}

#[test]
fn test2() {
    let ans = reverse_prefix(String::from("abcdefd"), 'g');
    assert_eq!(ans, "abcdefd");
}

#[test]
fn test3() {
    let ans = reverse_prefix(String::from("rzwuktxcjfpamlonbgyieqdvhs"), 's');
    assert_eq!(ans, "shvdqeiygbnolmapfjcxtkuwzr");
}