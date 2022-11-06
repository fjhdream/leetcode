pub fn interpret(command: String) -> String {
    let mut ans = String::new();
    let chars: Vec<char> = command.chars().collect::<Vec<_>>();
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == 'G' {
            ans.push(chars[i]);
            i += 1;
        } else {
            if chars[i] == '(' && chars[i + 1] == ')' {
                ans.push('o');
                i += 2;
            } else {
                ans.push_str("al");
                i += 4;
            }
        }
    }
    ans
}

#[test]
fn test() {
    let ans = interpret("(al)G(al)()()G".to_string());
    assert_eq!(ans, "alGalooG");
}
