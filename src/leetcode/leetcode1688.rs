pub fn number_of_matches(n: i32) -> i32 {
    let mut teams = n;
    let mut ans = 0;
    while teams != 1 {
       let remainder = teams % 2;
       let round = teams / 2;
       ans += round;
       teams = teams / 2 + remainder;
    }
    ans
}

pub fn number_of_matches2(n: i32) -> i32 {
    n - 1
}

#[test]
fn test() {
    let ans = number_of_matches(7);
    assert_eq!(ans, 6);
}

#[test]
fn test2() {
    let ans = number_of_matches(14);
    assert_eq!(ans, 13);
}