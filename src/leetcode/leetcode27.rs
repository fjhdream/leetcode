#[allow(dead_code)]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let n = nums.len();
    if n == 0 { return 0;}
    let mut left: usize = 0;
    for right in 0..n  {
        if nums[right] != val {
            nums[left] = nums[right];
            left += 1;
        }
    }
    left as i32
}

#[test]
fn test_example() {
    let mut nums = vec![3, 2, 2, 3];
    let ans = remove_element(&mut nums, 3);
    assert_eq!(ans, 2);
}