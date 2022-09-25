pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let(mut min_right, mut left) = (i32::MAX , -1);
    let(mut max_left, mut right) = (i32::MIN , -1);
    for i in 0..n {
        if min_right >= nums[n - i - 1] {
            min_right = nums[n - i - 1];
        } else {
            left = (n - i - 1) as i32; 
        }

        if max_left <= nums[i] {
            max_left = nums[i];
        } else {
            right = i as i32;
        }

    }
    if left == -1 {
        0
    } else {
        right - left + 1
    }
}

#[test]
pub fn test_example() {
    let nums = vec![1];
    let ans = find_unsorted_subarray(nums);
    assert_eq!(ans, 0);
}