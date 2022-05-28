pub fn is_perfect_square(num: i32) -> bool {
    let (mut left, mut right) = (0i64, num as i64);
    while left <= right {
        let mid: i64 = (left + (right - left) / 2) as i64;
        if mid * mid == num as i64 {
            return true;
        } else if mid * mid < num as i64 {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return false;
}

#[test]
fn test() {
    let ans = is_perfect_square(16);
    assert!(ans);
}

#[test]
fn test1() {
    let ans = is_perfect_square(15);
    assert!(!ans);
}

#[test]
fn test2() {
    let ans = is_perfect_square(1);
    assert!(ans);
}

#[test]
fn test3() {
    let ans = is_perfect_square(i32::MAX);
    assert!(!ans);
}
