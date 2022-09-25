pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 2 {
        return 0;
    }
    let(mut pre_len, mut ans, mut diff) = (0, 0, nums[1] - nums[0]);
    for i in 2..n {
        let cur_diff = nums[i] - nums[i-1];
        if cur_diff == diff {
            pre_len += 1;
        } else {
            pre_len = 0;
            diff = cur_diff;
        }
        ans += pre_len;
    }
    ans
}

#[test]
pub fn test_example() {
    let nums = vec![1, 2, 3, 4];
    let ans = number_of_arithmetic_slices(nums);
    assert_eq!(ans, 3);
}