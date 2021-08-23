pub fn get_maximum_generated(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let n = n as usize;
    let mut nums: Vec<i32>= vec![0; n + 1];
    nums[1] = 1;
    let mut ans = 1;
    for i in 2..=n {
        nums[i] = nums[i / 2] + i as i32 % 2 * nums[i / 2 + 1];
        ans = ans.max(nums[i]);
    }
    return ans;
}

#[test]
pub fn test() {
    let n = 7;
    let ans = get_maximum_generated(n);
    assert_eq!(ans, 3);
}