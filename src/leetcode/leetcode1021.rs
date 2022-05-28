pub fn remove_outer_parentheses(s: String) -> String {
    let s_chs = s.chars().collect::<Vec<_>>();
    let mut res = String::new();
    let mut stack = vec![];
    for ch in s_chs {
        if ch == ')' {
            stack.pop();
        }

        if !stack.is_empty() {
            res.push(ch);
        }

        if ch == '(' {
            stack.push(ch);
        }

    }
    res
}

#[test]
fn test() {
    let ans = remove_outer_parentheses("(()())(())".to_string());
    assert_eq!(ans, "()()()".to_string());
}

#[test]
fn test2() {
    let ans = remove_outer_parentheses("(()())(())(()(()))".to_string());
    assert_eq!(ans, "()()()()(())".to_string());
}

#[test]
fn test3() {
    let ans = remove_outer_parentheses("()()".to_string());
    assert_eq!(ans, "".to_string());
}

#[test]
fn test4() {
    let ans = remove_outer_parentheses("(()(()))".to_string());
    assert_eq!(ans, "()(())".to_string());
}