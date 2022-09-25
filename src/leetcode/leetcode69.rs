pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let (mut left, mut right) = (1i64, x as i64);
    while left <= right {
        let mid = (left + (right - left) / 2) as i64;
        if mid * mid == x as i64 {
            return mid as i32;
        } else if mid * mid > x as i64 {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return left as i32 - 1;
}

#[test]
fn test() {
    let ans = my_sqrt(2);
    assert_eq!(ans, 1);
}

#[test]
fn test2() {
    let ans = my_sqrt(4);
    assert_eq!(ans, 2);
}

#[test]
fn test3() {
    let ans = my_sqrt(8);
    assert_eq!(ans, 2);
}
