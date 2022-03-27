pub fn find_lhs(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mut ans = i32::MIN;
    let mut cur_cnt = 0;
    let n = nums.len();
    let (mut left, mut right) = (0, 0);
    while right < n {
        while nums[right] - nums[left] > 1 {
            left += 1;
        }
        if nums[right] - nums[left] == 1 {
            ans = ans.max(right as i32 - left as i32 + 1);
        }
        right += 1;
    }
    return ans;
}

#[test]
fn test() {
    let ans = find_lhs(vec![1,3,2,2,5,2,3,7]);
    assert_eq!(ans, 5);
}