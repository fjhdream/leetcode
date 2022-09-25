pub fn balanced_string_split(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<_>>();
    let mut diff = 0;
    let mut ans = 0;
    for ch in chars {
        if ch == 'R' {
            diff += 1;
        } 
        if ch == 'L' {
            diff -= 1;
        }
        if diff == 0 {
            ans += 1;
        }
    }
    ans
}

#[test]
fn test_example() {
    let s = String::from("RLRRLLRLRL");
    let ans = balanced_string_split(s);
    assert_eq!(ans, 4);
}

#[test]
fn test_example2() {
    let s = String::from("LLLLRRRR");
    let ans = balanced_string_split(s);
    assert_eq!(ans, 1);
}