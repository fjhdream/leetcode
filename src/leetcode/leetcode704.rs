pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0usize;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if target == nums[mid] {
            return mid as i32;
        } else if target > nums[mid] {
            left = mid + 1;
        } else {
            if mid == 0 {
                return -1;
            } else {
                right = mid - 1;
            }
        }
    }
    -1
}

#[test]
fn test_example() {
    let ans = search(vec![2, 5], 0);
    assert_eq!(ans, -1);
}

#[test]
fn test_example2() {
    let ans = search(vec![5], -5);
    assert_eq!(ans, -1);
}