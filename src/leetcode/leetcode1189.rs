use std::collections::HashMap;

pub fn max_number_of_balloons(text: String) -> i32 {
    let mut map = HashMap::new();
    let ball_chars = vec!['b', 'a', 'l', 'o', 'n'];
    for ch in ball_chars {
        map.insert(ch, 0);
    }

    for ch in text.chars() {
        if is_ball_char(ch) {
            *map.entry(ch).or_insert(0) += 1;
        }
    }

    let mut ans = i32::MAX;
    for (key, val) in map {
        if key == 'l' || key == 'o' {
            ans = ans.min(val / 2);
        } else {
            ans = ans.min(val);
        }
    }
    ans
}

fn is_ball_char(ch: char) -> bool {
    return ch == 'b' || ch == 'a' || ch == 'l' || ch == 'o' || ch == 'n';
}

#[test]
fn test() {
    let ans = max_number_of_balloons(String::from("nlaebolko"));
    assert_eq!(ans, 1);
}

#[test]
fn test2() {
    let ans = max_number_of_balloons(String::from("loonbalxballpoon"));
    assert_eq!(ans, 2);
}

#[test]
fn test3() {
    let ans = max_number_of_balloons(String::from("leetcode"));
    assert_eq!(ans, 0);
}

#[test]
fn test4() {
    let ans = max_number_of_balloons(String::from("i"));
    assert_eq!(ans, 0);
}

#[test]
fn test5() {
    let ans = max_number_of_balloons(String::from("lloo"));
    assert_eq!(ans, 0);
}
