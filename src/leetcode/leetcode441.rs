pub fn arrange_coins(n: i32) -> i32 {
    (((8f64 * n as f64 + 1f64).sqrt() - 1f64) / 2f64) as i32
}

#[test]
fn test() {
    let ans = arrange_coins(5);
    assert_eq!(ans, 2);
}

#[test]
fn test2() {
    let ans = arrange_coins(8);
    assert_eq!(ans, 3);
}
