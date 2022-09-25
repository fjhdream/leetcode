pub fn trailing_zeroes(mut n: i32) -> i32 {
    let mut ans = 0;
    while n >= 5 {
        n /= 5;
        ans += n;
    }
    ans
}

#[test] 
fn test() {
    let ans = trailing_zeroes(5);
    assert_eq!(ans, 1);
}
