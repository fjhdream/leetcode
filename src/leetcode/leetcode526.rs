pub fn count_arrangement(n: i32) -> i32 {
    let max_mask = (1 << n) as usize;
    let mut dp = vec![0; max_mask];
    dp[0] = 1;
    for mask in 1..max_mask {
        // 第index位置上选取数字
        let index = mask.count_ones() as usize;
        // 根据mask 看选取了哪个数字
        for num in 0..(n as usize) {
            if (mask & (1 << num)) != 0 
            && (index % (num+1) == 0 || (num+1) % index == 0) {
                dp[mask] += dp[mask ^ (1 << num)];
            }
        }        
    }
    dp[(1 << n) - 1]
}

#[test]
fn test_example() {
    let n = 2;
    let ans = count_arrangement(n);
    assert_eq!(ans, 2);
}