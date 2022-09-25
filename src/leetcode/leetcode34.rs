use std::os::macos::raw::stat;

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut result = vec![-1, -1];
    if nums.is_empty() {
        return result;
    }
    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid] == target {
            let mut left_index = mid;
            let mut right_index = mid;
            while left_index > 0 && nums[left_index] == target {
                left_index -= 1;
            }
            while right_index < nums.len() && nums[right_index] == target {
                right_index += 1;
            }
            result[0] = if left_index == 0 && nums[left_index] == target {
                0
            } else {
                left_index as i32 + 1
            };
            result[1] = right_index as i32 - 1;
            break;
        } else if nums[mid] > target {
            if mid == 0 {
                break;
            }
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    result
}

pub fn search_range2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let left_index = binary_search(&nums, target, true);
    let right_index = binary_search(&nums, target, false) - 1;
    if left_index <= right_index
        && right_index < nums.len() as i32
        && nums[left_index as usize] == target
        && nums[right_index as usize] == target
    {
        return vec![left_index, right_index];
    }
    return vec![-1, -1];
}

fn binary_search(nums: &Vec<i32>, target: i32, lower: bool) -> i32 {
    let mut left = 0 as i32;
    let mut right = nums.len() as i32 - 1;
    let mut ans = nums.len() as i32;
    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid as usize] > target || (lower && nums[mid as usize] >= target) {
            right = mid - 1;
            ans = mid as i32;
        } else {
            left = mid + 1;
        }
    }
    return ans;
}

#[test]
fn test() {
    let ans = search_range2(vec![1], 0);
    assert_eq!(ans, vec![-1, -1]);
}

#[test]
fn test2() {
    let ans = search_range2(vec![1], 1);
    assert_eq!(ans, vec![0, 0]);
}
