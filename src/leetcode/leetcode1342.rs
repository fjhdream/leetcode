pub fn number_of_steps(mut num: i32) -> i32 {
    let mut ans = 0;
    while num != 0 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num -= 1;
        }
        ans += 1;
    }
    ans
}

pub fn number_of_steps2(mut num: i32) -> i32 {
    if num == 0 { return 0 }
    32 - num.leading_zeros() as i32 + num.count_ones() as i32 - 1
}

#[test]
fn test() {
    let ans = number_of_steps2(14);
    assert_eq!(ans, 6);
}

#[test]
fn test2() {
    let ans = number_of_steps2(123);
    assert_eq!(ans, 12);
}