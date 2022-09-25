pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    nums.sort();
    let mut ans = vec![0;2];
    let mut pre = 0;
    for &val in nums.iter() {
        if val == pre {
            ans[0] = pre;
        } else if val - pre > 1 {
            ans[1] = pre + 1;
        }
        pre = val;
    }
    if nums[n - 1] != n as i32 {
        ans[1] = n as i32;
    }
    ans
}

#[test]
fn test_example() {
    let nums = vec![3, 2, 2];
    let ans = find_error_nums(nums);
    println!("{:?}", ans);
}