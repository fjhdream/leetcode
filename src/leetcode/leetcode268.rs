pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for num in &nums {
        ans = ans ^ num;
    }
    for i in 0..=nums.len() as i32 {
        ans = ans ^ i;
    }

    return ans;
}

#[test]
pub fn test_example() {
    let nums = vec![9,6,4,2,3,5,7,0,1];
    let ans = missing_number(nums);
    assert_eq!(ans, 8);
}