pub fn arrange_coins(n: i32) -> i32 {
    (((8f64 * n as f64 + 1f64).sqrt() - 1f64) / 2f64) as i32
}

pub fn arrange_coins2(n: i32) -> i32 {
    let (mut left, mut right) = (1i64, n as i64);
    while left <= right {
        let mid = left + (right - left) / 2;
        if mid * (mid + 1) == 2 * n as i64 {
            return mid as i32;
        } else if mid * (mid + 1) > 2 * n as i64 {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    left as i32 - 1
}

#[test]
fn test() {
    let ans = arrange_coins2(5);
    assert_eq!(ans, 2);
}

#[test]
fn test2() {
    let ans = arrange_coins2(8);
    assert_eq!(ans, 3);
}
