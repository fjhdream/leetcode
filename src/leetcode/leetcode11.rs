pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    while left < right {
        let capacity = (height[right].min(height[left])) * (right - left) as i32;
        max = max.max(capacity);
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max
}
