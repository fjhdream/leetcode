pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut cur = nums[0];
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            cur += nums[i];
        } else {
            res = res.max(cur);
            cur = nums[i];
        } 
    }
    res.max(cur)
}

#[test]
fn test() {
    let ans = max_ascending_sum(vec![10,20,30,5,10,50]);
    assert_eq!(ans, 65);
}
