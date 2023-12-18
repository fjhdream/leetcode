pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    if n < 2 {
        return 0;
    }
    if nums[0] > nums[1] {
        return 0;
    }
    if nums[n as usize - 1] > nums[n as usize - 2] {
        return n - 1;
    }
    let (mut left, mut right, mut ans) = (0, n - 1, -1);
    while left <= right {
        let mid = (left + right) / 2;
        if compare(&nums, mid - 1, mid) < 0 && compare(&nums, mid, mid + 1) > 0 {
            ans = mid;
            break;
        }
        if compare(&nums, mid, mid + 1) < 0 {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    ans
}

fn get(nums: &[i32], idx: i32) -> Option<i32> {
    if idx < 0 || idx as usize >= nums.len() {
        None
    } else {
        Some(nums[idx as usize])
    }
}

fn compare(nums: &[i32], idx1: i32, idx2: i32) -> i32 {
    let num1 = get(nums, idx1).unwrap_or(i32::MIN);
    let num2 = get(nums, idx2).unwrap_or(i32::MIN);
    num1.cmp(&num2) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(find_peak_element(nums), 2);
    }
}
