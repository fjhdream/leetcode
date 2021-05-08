pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut right = k as usize;
    let mut left = 0;
    let mut sum = 0;
    for i in 0..right {
        sum += nums[i];
    }

    let mut max_sum = sum;
    while right < nums.len() {
        sum += nums[right];
        sum -= nums[left];
        max_sum = max_sum.max(sum);
        right += 1;
        left += 1;
    }
    max_sum as f64 / k as f64
}

#[test]
pub fn test_example() {
    let numbs = vec![1,12,-5,-6,50,3];
    let k = 4;
    let ans = find_max_average(numbs, k);
    assert_eq!(ans, 12.75);
}    