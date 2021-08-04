pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mut ans = 0i32;
    let n = nums.len();
    for i in 0..n {
        // 防止nums[i] 为0的情况
        let mut k = i;
        for j in i + 1..n {
            while k + 1 < n && nums[i] + nums[j] > nums[k + 1] {
                k += 1;
            }
            ans += if k > j { (k - j) as i32 } else { 0 }
        }
    }
    ans
}

#[test]
pub fn test() {
    let nums = vec![2, 2, 3, 4];
    let ans = triangle_number(nums);
    assert_eq!(ans, 3);
}
