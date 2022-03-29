pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut ans = Vec::new();
    for val in left..=right {
        if is_self_divid(val) {
            ans.push(val);
        }
    }
    ans
}

fn is_self_divid(val: i32) -> bool {
    let mut num = val;
    while num > 0 {
        let div_num = num % 10;
        if div_num == 0 || val % div_num != 0 {
            return false;
        }
        num /= 10;
    }
    return true;
}


#[test]
fn test() {
    let ans = self_dividing_numbers(1, 22);
    assert_eq!(ans, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]);
}