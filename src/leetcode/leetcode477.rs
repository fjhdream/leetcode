pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let n = nums.len();
    for i in 0..=30 {
        let mut c = 0;
        nums.iter().for_each(|num| if num>>i & 1 == 1 { c += 1 });
        ans += c * (n - c);
    }
    ans as i32
}

#[test]
pub fn test() {
    let nums = vec![4, 14, 2];
    let ans = total_hamming_distance(nums);
    assert_eq!(ans, 6);
}