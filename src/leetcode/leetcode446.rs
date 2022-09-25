pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let n = nums.len();
    let mut ans = 0;
    let mut dp = vec![HashMap::<i64, usize>::new(); n];
    for i in 0..n {
        for j in 0..i {
            let dist = nums[i] as i64 - nums[j] as i64;
            let v = *dp[j].entry(dist).or_insert(0);
            ans += v as i64;
            *dp[i].entry(dist).or_insert(0) += v + 1;
        }
    }
    ans as i32
}

#[test]
fn test1() {
    let nums = vec![2,4,6,8,10];
    let ans = number_of_arithmetic_slices(nums);
    assert_eq!(ans, 7);
}

#[test]
fn test2() {
    let nums = vec![7,7,7,7,7];
    let ans = number_of_arithmetic_slices(nums);
    assert_eq!(ans, 16);
}

#[test]
fn test3() {
    let nums = vec![0,2000000000,-294967296];
    let ans = number_of_arithmetic_slices(nums);
    assert_eq!(ans, 0);
}