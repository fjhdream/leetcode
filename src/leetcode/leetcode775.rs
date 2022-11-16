pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n <= 2 {
        return true; 
    }
    let mut min_stuff = nums[n - 1];
    for i in (0..=n-3).rev() {
        if nums[i] > min_stuff {
            return false;
        }
        min_stuff = min_stuff.min(nums[i+1]);
    }
    true
}

