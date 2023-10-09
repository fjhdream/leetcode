pub fn split_num(num: i32) -> i32 {
    let num_chars = num.to_string().chars().into_iter().collect::<Vec<_>>();
    let mut nums = num_chars
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();
    nums.sort();
    let (mut num1, mut num2) = (0, 0);
    for i in 0..nums.len() {
        if i % 2 == 0 {
            num1 = num1 * 10 + (nums[i]);
        } else {
            num2 = num2 * 10 + nums[i];
        }
    }
    num1 + num2
}

#[test]
fn test() {
    let ans = split_num(4325);
    assert_eq!(ans, 59);
}
