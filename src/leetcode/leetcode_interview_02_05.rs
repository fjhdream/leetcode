pub fn one_edit_away(first: String, second: String) -> bool {
    if (first.len() as i32 - second.len() as i32 ).abs() > 1 {
        return false;
    }
    if first == second {
        return true;
    }
    
    if first.len() == second.len() {
        let mut diff = 0;
        for (i, c) in first.chars().enumerate() {
            if c != second.chars().nth(i).unwrap() {
                diff += 1;
            }
        }
        return diff == 1;
    } else if first.len() > second.len() {
        return is_two_string_diff_one(&first, &second);
    } else {
        return is_two_string_diff_one(&second, &first);
    }
}

fn is_two_string_diff_one(first: &String, second: &String) -> bool {
    if first.is_empty() || second.is_empty() {
        return true;
    }

    let mut diff = 0;
    let first_chars = first.chars().collect::<Vec<_>>();
    let second_chars = second.chars().collect::<Vec<_>>();
    let mut idx = 0;
    let len = second.len();
    while idx < len {
        if first_chars[idx + diff] != second_chars[idx] {
            if diff == 0 {
                diff += 1;
                continue;
            } else {
                return false;
            }
        }
        idx += 1;
    }
    return diff == 1 || (diff == 0 && idx == len);
}

#[test]
fn test() {
    let ans = one_edit_away("pale".to_string(), "ple".to_string());
    assert_eq!(ans, true);
}

#[test]
fn test2() {
    let ans = one_edit_away("".to_string(), "p".to_string());
    assert_eq!(ans, true);
}

#[test]
fn test3() {
    let ans = one_edit_away("a".to_string(), "ab".to_string());
    assert_eq!(ans, true);
}

#[test]
fn test4() {
    let ans = one_edit_away("teacher".to_string(), "bleacher".to_string());
    assert_eq!(ans, false);
}