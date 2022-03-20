pub fn generate_possible_next_moves(current_state: String) -> Vec<String> {
    let mut str_chars = current_state.chars().collect::<Vec<_>>();
    if str_chars.len() < 2 {
        return Vec::new();
    }
    let mut ans = Vec::new();
    for i in 0..str_chars.len() - 1 {
        if str_chars[i] == '+' && str_chars[i+1] == '+' {
            str_chars[i] = '-';
            str_chars[i + 1] = '-';
            ans.push(str_chars.iter().collect::<String>());
            str_chars[i] = '+';
            str_chars[i + 1] = '+';
        }
    }
    ans
}
#[test]
fn test() {
    let ans = generate_possible_next_moves("++++".to_string());
    assert_eq!(ans, vec!["--++", "+--+", "++--"]);
}