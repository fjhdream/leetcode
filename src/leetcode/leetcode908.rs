pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    if nums.len() == 1 {
        return 0;
    }
    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();
    if *max - *min <= k * 2 {
        return 0;
    } else {
        return *max - *min - k * 2;
    }
}